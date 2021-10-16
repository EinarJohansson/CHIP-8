# CHIP-8
CHIP-8 emulator written in rust😸🎮

### TODO
- [x] Scale resolution
- [ ] Sound
- [ ] Input
- [ ] Timers
- [ ] Opcodes
    - [x] 00E0 - CLS
    - [ ] 00EE - RET
    - [ ] 0nnn - SYS addr
    - [x] 1nnn - JP addr
    - [ ] 2nnn - CALL addr
    - [ ] 3xkk - SE Vx, byte
    - [ ] 4xkk - SNE Vx, byte
    - [ ] 5xy0 - SE Vx, Vy
    - [x] 6xkk - LD Vx, byte
    - [x] 7xkk - ADD Vx, byte
    - [ ] 8xy0 - LD Vx, Vy
    - [ ] 8xy1 - OR Vx, Vy
    - [ ] 8xy2 - AND Vx, Vy
    - [ ] 8xy3 - XOR Vx, Vy
    - [ ] 8xy4 - ADD Vx, Vy
    - [ ] 8xy5 - SUB Vx, Vy
    - [ ] 8xy6 - SHR Vx {, Vy}
    - [ ] 8xy7 - SUBN Vx, Vy
    - [ ] 8xyE - SHL Vx {, Vy}
    - [ ] 9xy0 - SNE Vx, Vy
    - [x] Annn - LD I, addr
    - [ ] Bnnn - JP V0, addr
    - [ ] Cxkk - RND Vx, byte
    - [x] Dxyn - DRW Vx, Vy, nibble
    - [ ] Ex9E - SKP Vx
    - [ ] ExA1 - SKNP Vx
    - [ ] Fx07 - LD Vx, DT
    - [ ] Fx0A - LD Vx, K
    - [ ] Fx15 - LD DT, Vx
    - [ ] Fx18 - LD ST, Vx
    - [ ] Fx1E - ADD I, Vx
    - [ ] Fx29 - LD F, Vx
    - [ ] Fx33 - LD B, Vx
    - [ ] Fx55 - LD [I], Vx
    - [ ] Fx65 - LD Vx, [I]