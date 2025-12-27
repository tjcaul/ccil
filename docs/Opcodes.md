# Opcode list

| Opcode | Arguments | Description |
|:------:|:---------:|:------------|
| NOP    |           | Do nothing |
| CONST  | constant  | Push constant to the stack |
| POP    |           | Remove the top item on the stack; same as DROP 1 |
| DROP   | count     | Remove count items from the top of the stack |
| COPY   |           | Copy the index-th item in the stack onto the top |
| STORE  | index     | Pop the top item off the stack and store it in the index-th spot in the stack, replacing the existing value there |
| SWAP   |           | Swap the top two items on the stack; same as ROT 1 |
| ROT    | count     | Lift up count items on the stack and move the top item to the count position (ROT 2 means `a, b, c, d -> a, d, c, b`) |
| NEG    |           | Negate the top number on the stack |
| ADD    |           | Pop two numbers off the stack and push their sum |
| SUB    |           | Pop two numbers off the stack and push their difference (`a, b, c -> a, b-c`) |
| MUL    |           | Pop two numbers off the stack and push their product |
| DIV    |           | Pop two numbers off the stack and push their quotient (`a, b, c -> a, b/c`) |
| MOD    |           | Pop two numbers off the stack and push their remainder (`a, b, c -> a, b%c`) |
| BNOT   |           | Bitwise invert the top item on the stack |
| BOR    |           | Pop two items off the stack and push their bitwise disjunction |
| BAND   |           | Pop two items off the stack and push their bitwise conjunction |
| BXOR   |           | Pop two items off the stack and push their bitwise exclusive disjunction |
| NOT    |           | Boolean invert the top item on the stack |
| OR     |           | Pop two items off the stack and push their boolean disjunction |
| AND    |           | Pop two items off the stack and push their boolean conjunction |
| XOR    |           | Pop two items off the stack and push their boolean exclusive disjunction |
| SHL    |           | Pop two items x, then y off the stack and shift y left by x bits (`a, y, x -> a, y << x`) |
| SHRL   |           | Pop two items x, then y off the stack and shift y right by x bits, filling the most significant positions with zeros (`a, y, x -> a, y >> x`) |
| SHRA   |           | Pop two items x, then y off the stack and shift y right by x bits, copying the most significant bit rightwards (`a, y, x -> a, y >> x`) |
| JUMP   | address   | Jump to the given address |
| IFZ    | address   | Pop the top of the stack; if it is zero, jump to the given address |
| IFNZ   | address   | Pop the top of the stack; if it is not zero, jump to the given address |
| CALL   | address   | Push the address of the next operation to the stack, then jump to the given address |
| RETURN | count     | Discard count items from the stack, the pop the return address off the stack and jump to it |
