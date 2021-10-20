# CHIP-8
CHIP-8 emulator written in rust😸🎮

### TODO
- [x] Display
- [x] Scale resolution
- [x] Timers
    - [x] Delay
    - [x] Sound
- [x] Input
- [ ] Sound
- [x] Opcodes
    - [x] 00E0 - CLS
    - [x] 00EE - RET
    - [ ] <strike>0nnn - SYS addr</strike>
    - [x] 1nnn - JP addr
    - [x] 2nnn - CALL addr
    - [x] 3xkk - SE Vx, byte
    - [x] 4xkk - SNE Vx, byte
    - [x] 5xy0 - SE Vx, Vy
    - [x] 6xkk - LD Vx, byte
    - [x] 7xkk - ADD Vx, byte
    - [x] 8xy0 - LD Vx, Vy
    - [x] 8xy1 - OR Vx, Vy
    - [x] 8xy2 - AND Vx, Vy
    - [x] 8xy3 - XOR Vx, Vy
    - [x] 8xy4 - ADD Vx, Vy
    - [x] 8xy5 - SUB Vx, Vy
    - [x] 8xy6 - SHR Vx {, Vy}
    - [x] 8xy7 - SUBN Vx, Vy
    - [x] 8xyE - SHL Vx {, Vy}
    - [x] 9xy0 - SNE Vx, Vy
    - [x] Annn - LD I, addr
    - [x] Bnnn - JP V0, addr
    - [x] Cxkk - RND Vx, byte
    - [x] Dxyn - DRW Vx, Vy, nibble
    - [x] Ex9E - SKP Vx
    - [x] ExA1 - SKNP Vx
    - [x] Fx07 - LD Vx, DT
    - [x] Fx0A - LD Vx, K
    - [x] Fx15 - LD DT, Vx
    - [x] Fx18 - LD ST, Vx
    - [x] Fx1E - ADD I, Vx
    - [x] Fx29 - LD F, Vx
    - [x] Fx33 - LD B, Vx
    - [x] Fx55 - LD [I], Vx
    - [x] Fx65 - LD Vx, [I]
