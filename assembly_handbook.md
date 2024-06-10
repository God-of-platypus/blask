# Assembly Handbook

This is the official documentation of the blasm assembler.

## Instructions

In all instructions, the first operand is always a register and is always the destination register.

### ADD

Adds two registers and puts the result in another register.

```asm
# $3 = $2 + $1
add 3, 2, 1
```

Can be used to copy the value of a register into another register using the 0 register.

```asm
# $1 = $2
add $1, $2, $0
```

### SUB

Subtracts a register from another register and puts the result in a third register.

```asm
# $3 = $2 - $1
sub 3, 2, 1
```

### OR

Computes the bitwise OR between two registers and puts the result in a third register.

```asm
# $3 = $2 | $1
or 3, 2, 1
```

Can be used to copy the value of a register into another register using the 0 register.

```asm
# $1 = $2
or $1, $2, $0
```

### AND

Computes the bitwise AND between two registers and puts the result in a third register.

```asm
# $3 = $2 & $1
and 3, 2, 1
```

### XOR

Computes the bitwise XOR between two registers and puts the result in a third register.

```asm
# $3 = $2 ^ $1
xor 3, 2, 1
```

### SLL

Shifts the value in a register to the left, by the amount specified in another register and then puts the result in a third register.

```asm
# $3 = $2 << $1
sll 3, 2, 1
```

### SRL

Shifts the value in a register to the right, by the amount specified in another register and then puts the result in a third register.

```asm
# $3 = $2 >> $1
srl 3, 2, 1
```

### ADDI

Adds an immediate value to the value of a register and puts the result in another register.

```asm
# $2 = $1 + 42
addi 2, 1, 42
```

Can be used to load an immediate value into a register using the 0 register.

```asm
# $1 = 42
addi $1, $0, 42
```

### SUBI

Subtracts an immediate value from the value of a register and puts the result in another register.

```asm
# $2 = $1 - 42
subi 2, 1, 42
```

This is actually a pseudo-instruction.
It is translated to an `addi` instruction with the immediate value replaced by its 2's complement.

### ORI

Computes the bitwise OR between an immediate value and the value of a register and puts the result in another register.

```asm
# $3 = $2 | 42
ori 3, 2, 42
```

Can be used to load an immediate value into a register using the 0 register.

```asm
# $1 = 42
ori $1, $0, 42
```

### ANDI

Computes the bitwise AND between an immediate value and the value of a register and puts the result in another register.

```asm
# $3 = $2 & 42
andi 3, 2, 42
```

### XORI

Computes the bitwise XOR between an immediate value and the value of a register and puts the result in another register.

```asm
# $3 = $2 ^ 42
xori 3, 2, 42
```

### SLLI

Shifts the value in a register to the left, by an immediate value and then puts the result in another register.

```asm
# $3 = $2 << 5
slli 3, 2, 5
```

### SRLI

Shifts the value in a register to the right, by an immediate value and then puts the result in another register.

```asm
# $3 = $2 << 5
srli 3, 2, 5
```

### LD

Loads a value from RAM at the address contained in a register plus an immediate value offset and puts the result in another register.

```asm
# $2 = *($1 + 42)
ld $2, $1, 42
```

### STR

Stores a value from a register in RAM at the address contained in a register plus an immediate value offset.

```asm
# (*$2 + 42) = $1
str $2, $1, 42
```

### BE

Jumps to the address specified by an immediate value if the first register is equal to the second register.

```asm
# if ($1 == $2) {
#   goto 42
# }
be $1, $2, 42

You can also use a label.

```asm
# if ($1 == $2) {
#   goto label
# }
be $1, $2, @label
```

### BNE

Jumps to the address specified by an immediate value if the first register is not equal to the second register.

```asm
# if ($1 != $2) {
#   goto 42
# }
bne $1, $2, 42

You can also use a label.

```asm
# if ($1 != $2) {
#   goto label
# }
bne $1, $2, @label
```

### BLT

Jumps to the address specified by an immediate value if the first register is lower than the second register.

```asm
# if ($1 < $2) {
#   goto 42
# }
blt $1, $2, 42

You can also use a label.

```asm
# if ($1 < $2) {
#   goto label
# }
blt $1, $2, @label
```

### BGE

Jumps to the address specified by an immediate value if the first register is greater than or equal to the second register.

```asm
# if ($1 >= $2) {
#   goto 42
# }
bge $1, $2, 42

You can also use a label.

```asm
# if ($1 >= $2) {
#   goto label
# }
bge $1, $2, @label
```

### BLTU

Jumps to the address specified by an immediate value if the first register is lower than the second register.

```asm
# if ((uint16_t) $1 < (uint16_t) $2) {
#   goto 42
# }
bltu $1, $2, 42

You can also use a label.

```asm
# if ((uint16_t) $1 < (uint16_t) $2) {
#   goto label
# }
bltu $1, $2, @label
```

### BGEU

Jumps to the address specified by an immediate value if the first register is greater than or equal to the second register.

```asm
# if ((uint16_t) $1 >= (uint16_t) $2) {
#   goto 42
# }
bgeu $1, $2, 42

You can also use a label.

```asm
# if ((uint16_t) $1 >= (uint16_t) $2) {
#   goto label
# }
bgeu $1, $2, @label
```
