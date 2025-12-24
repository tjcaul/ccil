# CCIL

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
constant 2
constant 3
add
pop
```

## Bytecode

A CCIL binary program, or a chunk of bytecode in the CCIL virtual machine,
consists of a 16-byte header followed by a list of operations.

The header consists of magic number `0xCC17`, three one-byte version numbers
(major, minor, patch), and padding.

Each operation consists of a one-byte opcode followed by zero or more
four-byte little-endian arguments.
The number and bounds of arguments depends on the operation.
For example,
```
nop
constant 2
pop
```
is encoded as
```
CC17 0001 0000 0000 0000 0000 0000 0000 0001 0200 0000 03
```
breakdown:
```
.================ HEADER ================.
CC17                         -- magic num
010203                       -- v1.2.3
0000 0000 0000 0000 0000 00  -- padding
'========================================'
.================ PROGRAM ===============.
00                           -- nop
01 02000000                  -- constant 2
03                           -- pop
'========================================'
```
