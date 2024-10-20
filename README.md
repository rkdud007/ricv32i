# ricv5

## compile

note: my machine is m2 mac

```sh
# prerequisite
brew tap riscv/riscv
brew install riscv-tools
brew test riscv-tools

# compile
./rv32-binary.sh {path .s}
```

## registers

| #   | Name  | Purpose                            |
| --- | ----- | ---------------------------------- |
| x0  | zero  | Constant zero                      |
| x1  | ra    | Return address (link register)     |
| x2  | sp    | Stack pointer                      |
| x3  | gp    | Global pointer                     |
| x4  | tp    | Thread pointer                     |
| x5  | t0    | Temporary register 0               |
| x6  | t1    | Temporary register 1               |
| x7  | t2    | Temporary register 2               |
| x8  | s0/fp | Saved register 0 / Frame pointer   |
| x9  | s1    | Saved register 1                   |
| x10 | a0    | Function argument / Return value 0 |
| x11 | a1    | Function argument / Return value 1 |
| x12 | a2    | Function argument 2                |
| x13 | a3    | Function argument 3                |
| x14 | a4    | Function argument 4                |
| x15 | a5    | Function argument 5                |
| x16 | a6    | Function argument 6                |
| x17 | a7    | Function argument 7                |
| x18 | s2    | Saved register 2                   |
| x19 | s3    | Saved register 3                   |
| x20 | s4    | Saved register 4                   |
| x21 | s5    | Saved register 5                   |
| x22 | s6    | Saved register 6                   |
| x23 | s7    | Saved register 7                   |
| x24 | s8    | Saved register 8                   |
| x25 | s9    | Saved register 9                   |
| x26 | s10   | Saved register 10                  |
| x27 | s11   | Saved register 11                  |
| x28 | t3    | Temporary register 3               |
| x29 | t4    | Temporary register 4               |
| x30 | t5    | Temporary register 5               |
| x31 | t6    | Temporary register 6               |
| \_  | pc    | Program counter                    |
