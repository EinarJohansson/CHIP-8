pub mod video;
use video::Video;

/// A chip-8 emulator🎮
#[allow(dead_code)]
pub struct Chip8 {
    memory: Vec<u8>,    // Stored big-endian, 4 kB
    opcode:     u16,        // Current opcode
    v:          Vec<u8>,    // General purpose registers
    i:          u16,        // Index register
    pc:         u16,        // Program counter
    stack:      Vec<u16>,   // Used to call subroutines/functions and return from them
    sp:         u16,        // Remember which level of the stack is used
    pub video:  Video,      // Graphics
}

impl Chip8 {
    /// Create a new chip-8 emulator😸
    pub fn new() -> Self {
        // Initialize memory to 4 kB
        let mut memory = vec![0x00; 0x1000];

        // Fontset used by chip-8
        let fontset: Vec<u8> = vec![ 
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        
        // Load fontset into memory
        for i in 0..fontset.len() {
            memory[i] = fontset[i];
        }

        Self {
            memory,
            opcode: 0x00,
            v:      vec![0x00; 0x10],
            i:      0x0000,
            pc:     0x0200,
            stack:  vec![0x0000; 0x10],
            sp:     0x0000,
            video:  Video::new()
        }
    }

    /// Load a program into memory.
    pub fn load(&mut self, path: &std::path::Path) {
        let instructions = std::fs::read(path).unwrap();

        // Load the instructions into memory
        for i in 0..instructions.len() {
            // Start at adress 0x0200
            self.memory[i + 0x0200] = instructions[i];
        }
    }

    /// Execute one instruction from the program
    pub fn cycle(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

    /// Combines a pair of u8's at the pc adress into one u16 opcode
    fn fetch(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc+1) as usize] as u16)
    }

    /// Decode opcode and execute it
    fn execute(&mut self, instruction: u16) {
        let mut increment_pc = true;

        let nnn =   instruction & 0x0FFF;
        let nn =    (instruction & 0x00FF) as u8;
        let n =     (instruction & 0x000F) as u8;       
        let x =     ((instruction & 0x0F00) >> 8) as usize;
        let y =     ((instruction & 0x00F0) >> 4) as usize;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match nnn {
                    0x0E0 => {
                        // Clear screen
                        self.video.clear_screen();
                    },
                    0x0EE => {
                        // Return from a subroutine
                        if let Some(adress) = self.stack.pop() {
                            self.pc = adress;
                            increment_pc = false;
                        }
                    },
                    _ => ()
                }
            },
            0x1 => {
                // Jump
                self.pc = nnn;
                increment_pc = false;
            },
            0x2 => {
                // Call subroutine at NNN
                self.stack.push(self.pc);
                self.pc = nnn;
                increment_pc = false;
            },
            0x3 => {
                // Skips the next instruction if VX equals NN
                if self.v[x] == nn {
                    self.pc += 2;
                }
            },
            0x4 => {
                // Skips the next instruction if VX does not equal NN.
                if self.v[x] != nn {
                    self.pc += 2;
                }
            },
            0x5 => {
                if n == 0x0 {
                    // Skips the next instruction if VX equals VY. 
                    if self.v[x] == self.v[y] {
                        self.pc += 2;
                    }
                }
            },
            0x6 => {
                // Update Vx
                self.v[x] = nn;
            },
            0x7 => {
                // Add to Vx
                let (wrapped_value, _) = self.v[x].overflowing_add(nn);
                self.v[x] = wrapped_value;
            },
            0x8 => {
                match n {
                    0x0 => {
                        // Sets VX to the value of VY.
                        self.v[x] = self.v[y];
                    },
                    0x1 => {
                        // Sets VX to VX or VY. (Bitwise OR operation);
                        self.v[x] |= self.v[y];
                    },
                    0x2 => {
                        // Sets VX to VX and VY. (Bitwise AND operation);
                        self.v[x] &= self.v[y];
                    },
                    0x3 => {
                        // Sets VX to VX xor VY
                        self.v[x] ^= self.v[y];
                    },
                    0x4 => {
                        // Adds VY to VX
                        let (wrapped_value, overflow) = self.v[x].overflowing_add(self.v[y]);
                        self.v[0xF] = if overflow {1} else {0}; // Set carry flag
                        self.v[x] = wrapped_value;
                    },
                    0x5 => {
                        // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                        let (wrapped_value, overflow) = self.v[x].overflowing_sub(self.v[y]);
                        self.v[0xF] = if !overflow {1} else {0}; // Set carry flag
                        self.v[x] = wrapped_value;
                    },
                    0x6 => {
                        // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                    },
                    0x7 => {
                        // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                        let (wrapped_value, overflow) = self.v[y].overflowing_sub(self.v[x]);
                        self.v[0xF] = if !overflow {1} else {0}; // Set carry flag
                        self.v[x] = wrapped_value;
                    },
                    0xE => {
                        // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                        self.v[0xF] = self.v[x] & 0x80;
                        self.v[x] <<= 1;
                    },
                    _ => () 
                }
            },
            0xA => {
                // Set index register to NNN
                self.i = nnn;
            },
            0xD => {
                // Draw
                self.v[0xF] = 0;
                let x_coordinate = self.v[x] as usize;
                let y_coordinate = self.v[y] as usize;

                for byte_index in 0..n as usize {
                    let byte = self.memory[self.i as usize + byte_index];

                    for bit_index in 0..8 {
                        let bit = byte & (0x80 >> bit_index);
                        if bit != 0 {
                            let gfx_index = self.video.get_index(
                                x_coordinate + bit_index,
                                y_coordinate + byte_index
                            );

                            let pixel = self.video.get_pixel(gfx_index);

                            if pixel == 1 {
                                self.v[0xF] = 1
                            }
                            
                            self.video.xor(gfx_index);
                        }

                    }
                }
            }
            _ => ()
        }

        if increment_pc {
            self.pc += 2;
        }
    }
}