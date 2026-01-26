# CCIL

<p align="center">
  <img src="cciltransp.png" alt="CCIL logo" />
</p>

The Caul-Chen Interpreted Language (CCIL) is a dynamically-typed, general-purpose programming language with a syntax reminiscent of C or JavaScript.

This project serves as the reference implementation of the language, and consists of a bytecode compiler, bytecode assembler/disassembler, REPL, and Virtual Machine.

All code is copyright (c) 2025-26 Tyson Caul and Ray Chen and is [licensed under GPL-3.0](LICENSE).

## Running

All tools (`ccil`, `ccila`, `ccilb`) can be run with the
`--help` option for a full list of options.
Below is an incomplete list of useful commands.

Assemble and run CCIL assembly file:
```
cargo run --bin ccila bytecode_assembly/test.ccila
```

Assemble CCIL assembly file to binary:
```
cargo run --bin ccila bytecode_assembly/test.ccila -o bytecode/test.ccilb
```

Disassemble CCIL binary to assembly:
```
cargo run --bin ccild bytecode/test.ccilb
```

Disassemble CCIL binary to assembly file:
```
cargo run --bin ccild bytecode/test.ccilb -o test_disasm.ccila
```

## Language

## Assembly

A CCIL assembly program consists of a newline-separated sequence of operations.
Each operation is a symbol followed by zero or more more numerical arguments.
The number and bounds of arguments depends on the operation.
For example:
```
nop
const 2
const 3
add
pop
```

## Bytecode

A CCIL binary program, or a chunk of bytecode in the CCIL virtual machine,
consists of a 16-byte header followed by a list of operations.

The header consists of magic number `0xCC17`, three one-byte version numbers
(major, minor, patch), a byte used for metadata flags (such as whether the program
was compiled from source or from assembly), the 32-bit UTC Unix timestamp in seconds
(little-endian), and padding to 16 bytes.

Each operation consists of a one-byte opcode followed by zero or more
four-byte little-endian arguments.
The number and bounds of arguments depends on the operation.
For example,
```
nop
const 2
pop
```
compiled from assembly at 9:11:53 AM UTC on January 26, 2026 is encoded as
```
CC17 0102 0301 D72F 7769 0000 0000 0000 0001 0200 0000 02
```
breakdown:
```
.================ HEADER ================.
CC 17                        -- magic num
01 02 03                     -- v1.2.3
01                           -- bitflags
D7 2F 77 69                  -- timestamp
00 00 00 00 00 00            -- padding
'========================================'
.================ PROGRAM ===============.
00                           -- nop
01 02000000                  -- const 2
02                           -- pop
'========================================'
```
