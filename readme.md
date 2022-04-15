# Rust CPU
A Learning tool for the fundamentals of Central processing units

## Built with
- Rust 1.60.0
- EGui 0.17.0 (EFrame 0.17.0)

## Run
You should be able to find prebuilt binaries in the releases of this project

## Build
Using Cargo: ```cargo build --release```

## Instruction Format
| Instruction Type | Data           | 
|------------------|----------------|
| 0000             | 0000 0000000   |
| 4-Bits           | 12-Bits        |

One Clock Cycle takes 2 bytes and splits/processes them as shown above

## Operations
```
Value (8-bits) (0x00) (00000000)
Address (12-bits) (0x000) (000000000000)
```

| Hex Code | Asm Code     | Data    | Explanation                                                                             |
|----------|--------------|---------|-----------------------------------------------------------------------------------------|
| 0x0      | noop         | X       | Don't perform any operation                                                             |
| 0x1      | set          | Value   | Gets the last 8 bits of the current instruction and inserts them into the math register |
| 0x2      | load         | Address | Loads value from memory, based on the 12-bit address provided, into math register       |
| 0x3      | save         | Address | Saves Value from math register into memory, based on the 12-bit address provided.       |
| 0x4      | add          | Value   | Adds Provided 8-bit number to the math register                                         |
| 0x5      | subtract     | Value   | Subtracts Provided 8-bit number to the math register                                    |
| 0x6      | add_mem      | Address | Add number at address to the math register                                              |
| 0x7      | subtract_mem | Address | Subtract number at address from the math register                                       |
| 0x8      | increase     | Address | Increase Value in memory, based on the 12-bit address provided.                         |
| 0x9      | decrease     | Address | Decrease Value in memory, based on the 12-bit address provided.                         |
| 0xA      | jmp_inz      | Address | Jumps to the 12-bit address if the number in the math register is not zero              |
| 0xB      | jmp_iz       | Address | Jumps to the 12-bit address if the number in the math register is zero                  |
| 0xC      | jmp          | Address | Jumps to the 12-bit address unconditionally                                             |
| 0xD      | jmp_ic       | Address | Jumps to the 12-bit address if the carry flag is set                                    |
| 0xE      | wext         | Address | (WIP) Write to external device                                                          |
| 0xF      | rext         | Address | (WIP) Read from external device                                                         |