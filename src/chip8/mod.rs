pub mod video;
mod keyboard;
use core::panic;
use video::Video;
use rand::Rng;
use keyboard::Keyboard;

/// A chip-8 emulator🎮
pub struct Chip8 {
    memory:         Vec<u8>,    // Stored big-endian, 4 kB
    v:              Vec<u8>,    // General purpose registers
    i:              u16,        // Index register
    pc:             u16,        // Program counter
    stack:          Vec<u16>,   // Used to call subroutines/functions and return from them
    delay_timer:    u8,         // Delay timer
    sound_timer:    u8,         // Sound timer
    pub video:      Video,      // Graphics
    pub keyboard:   Keyboard    // Keyboard
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
            v:              vec![0x00; 0x10],
            i:              0x0000,
            pc:             0x0200,
            stack:          vec![0x0000; 0x10],
            delay_timer:    0,
            sound_timer:    0,
            video:          Video::new(),
            keyboard:       Keyboard::new()
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
        // Execute at around 500 Hz
        for _ in 0..10 {
            let instruction = self.fetch();
            self.execute(instruction);
        }

        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.delay_timer.saturating_sub(1);
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
                        }
                    },
                    _ => {
                        panic!("Wat dis mean doe: {:x}", instruction);
                    }
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
                let wrapped_value = self.v[x].wrapping_add(nn);
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
                    _ => {
                        panic!("Wat dis mean doe: {:x}", instruction);
                    } 
                }
            },
            0x9 => {
                if n == 0x0 {
                    // Skips the next instruction if VX does not equal VY
                    if self.v[x] != self.v[y] {
                        self.pc += 2;
                    }
                }
            },
            0xA => {
                // Set index register to NNN
                self.i = nnn;
            },
            0xB => {
                // Jumps to the address NNN plus V0
                let wrapped_value = (self.v[0] as u16).wrapping_add(nnn);
                self.pc = wrapped_value;
                increment_pc = false;
            },
            0xC => {
                // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                let random_number: u8 = rand::thread_rng().gen();
                self.v[x] = random_number & nn;
            },
            0xD => {
                // Draw
                self.v[0xF] = 0;
                let x_coordinate = self.v[x] as usize;
                let y_coordinate = self.v[y] as usize;

                for byte_index in 0..n as usize {
                    let byte = self.memory[self.i as usize + byte_index];

                    for bit_index in 0..8 {
                        if (byte & (0x80 >> bit_index)) != 0 {
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
            },
            0xE => {
                match nn {
                    0x9E => {
                        // Skips the next instruction if the key stored in VX is pressed. 
                        if self.keyboard.is_pressed(self.v[x]) {
                            self.pc += 2;
                            self.keyboard.clear();
                        }
                    },
                    0xA1 => {
                        // Skips the next instruction if the key stored in VX is not pressed. 
                        if !self.keyboard.is_pressed(self.v[x]) {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        panic!("Wat dis mean doe: {:x}", instruction);
                    }
                };
            },
            0xF => {
                match nn {
                    0x07 => {
                        // Sets VX to the value of the delay timer.
                        self.v[x] = self.delay_timer;
                    },
                    0x15 => {
                        // Sets the delay timer to VX.
                        self.delay_timer = self.v[x];
                    },
                    0x18 => {
                        // Sets the sound timer to VX.
                        self.sound_timer = self.v[x];
                    }
                    0x1E => {
                        // Adds VX to I. VF is not affected
                        let wrapped_value  = self.i.wrapping_add(self.v[x] as u16);
                        self.i = wrapped_value;
                    },
                    0x29 => {
                        // Sets I to the location of the sprite for the character in VX
                        let wrapped_value  = (self.v[x] as u16).wrapping_mul(5);
                        self.i = wrapped_value;
                    },
                    0x33 => {
                        // Get the hundreds digit and place it in I.
                        self.memory[self.i as usize] = self.v[x] / 100;

                        // Get tens digit and place it in I+1. Gets a value between 0 and 99,
                        // then divides by 10 to give us a value between 0 and 9.
                        self.memory[(self.i + 1) as usize] =(self.v[x] % 100) / 10;

                        // Get the value of the ones (last) digit and place it in I+2.
                        self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                    },
                    0x55 => {
                        // Stores V0 to VX (including VX) in memory starting at address I.
                        for index in 0..=x {
                            self.memory[self.i as usize + index] = self.v[index]
                        }
                    },
                    0x65 => {
                        // Fills V0 to VX (including VX) with values from memory starting at address I.
                        for index in 0..=x {
                            self.v[index] = self.memory[self.i as usize + index]
                        }
                    },
                    _ => {
                        panic!("Wat dis mean doe: {:x}", instruction);                        
                    }
                }
            },
            _ => {
                panic!("Wat dis mean doe: {:x}", instruction);
            }
        }

        if increment_pc {
            self.pc += 2;
        }
    }
}