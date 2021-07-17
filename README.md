# The RC Programing Language
![build](https://github.com/ChiangYintso/rc/workflows/build/badge.svg)
[![codecov](https://codecov.io/gh/ChiangYintso/rc/branch/main/graph/badge.svg?token=FSSV4INNPZ)](https://codecov.io/gh/ChiangYintso/rc)

A rust-like toy language written in Rust. Available target is riscv32im.

## Quick Start

### Installation
- [rcc compiler](https://github.com/ChiangYintso/rc/releases)
- [RISC-V GNU Compiler Toolchain](https://github.com/riscv/riscv-gnu-toolchain) or other assembler
- Emulator like [QEMU](https://github.com/qemu/qemu)

### Hello, World!
Create a file named `foo.rc`.
```rust
// File name: foo.rc
extern "C" {
    fn putchar(c: i32);
}

fn add10(x: i32) -> i32 {
    x + 10
}

pub fn main() -> i32 {
    putchar(103 + 1); // 'h'
    putchar(101); // 'e'
    let mut i = 0;
    while i < 2 {
        putchar(108); // 'l' 'l'
        i += 1;
    }
    putchar(333 / 3i32); // 'o'
    putchar(10); // '\n'
    0
}
```
Compile `foo.rc` to RISC-V assembly language.
```shell
$ ./rcc foo.rc -o foo.S
```

Assemble and link to executable file.
```shell
$ riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 foo.S -o foo
```

Run in QEMU
```shell
$ qemu-riscv32 ./foo
hello
```

## References
- [Rust](https://github.com/rust-lang/rust)
- [syn(parser for Rust source code)](https://github.com/dtolnay/syn)
