# CHIP-8 Compiler

## Usage

Run the following command in the same directory as `chip8.exe`, or store it as a global key in your environment variables.

- Parameters with `?` are optional

```sh
chip8 <src code path> <out path?>
```

The output will be saved in your current directory as `out.ch8` unless an output path was provided.

e.g.
```sh
chip8 test.asm
```

## Supported instructions

| Mnemonic | Parameters | Description                          |
| -------- | ---------- | ------------------------------------ |
| NOP      |            | No operation                         |
| CLS      |            | Clear the screen                     |
| RET      |            | Return from subroutine               |
| JP       | NNN        | Jump to address NNN                  |
| CALL     | NNN        | Call subroutine NNN                  |
| SE       | X, NN      | Skip next instruction if `VX` == NN  |
| SNE      | X, NN      | Skip next instruction if `VX` != NN  |
| SEV      | X, Y       | Skip next if `VX` == `VY`            |
| LDVN     | X, NN      | `VX` = NN                            |
| ADD      | X, NN      | `VX` += NN                           |
| LDVY     | X, Y       | `VX` = `VY`                          |
| OR       | X, Y       | `VX` OR `VY` (stored in `VX`)        |
| AND      | X, Y       | `VX` AND `VY` (stored in `VX`)       |
| XOR      | X, Y       | `VX` XOR `VY` (stored in `VX`)       |
| ADDC     | X, Y       | `VX` += VY (\w carry)                |
| SUB      | X, Y       | `VX` -= `VY`                         |
| SHR      | X, Y       | `VX` >>= 1                           |
| SUBN     | X, Y       | `VX` = `VY` - `VX`                   |
| SHL      | X, Y       | `VX` <<= 1                           |
| SNEV     | X, Y       | Skip next op if `VX` != `VY`         |
| LDI      | NNN        | Set address pointer                  |
| JPV      | NNN        | Jump to `V0` + NNN                   |
| RND      | X, NN      | `VX` = rand & NN                     |
| DRW      | X, Y, N    | Draw sprite N at pos (`VX`, `VY`)    |
| SKP      | X          | Skip next if `VX` pressed            |
| SKNP     | X          | Skip next if `VX` not pressed        |
| LDDT     | X          | Load delay timer value to `VX`       |
| LDP      | X          | Wait for key press and store in `VX` |
| LDDV     | X          | Load delay timer from `VX`           |
| LDSV     | X          | Load sound timer from `VX`           |
| ADDI     | X          | Increment `I` with `VX`              |
| LDIF     | X          | Set `I` to font address              |
| BCD      | X          | BCD of `VX` at `I`                   |
| LDIV     | X          | Store `V0`-`VX` to `I`               |
| LDVI     | X          | Load `I` into `V0`-`VX`              |

## Syntax

```asm
<mnemonic> <param1> <param2> <param3>
```

e.g.
```asm
ADD 5 0x21
```

Look at the included example: [test.asm](test.asm)

### Comments

Use `;` to write comments. Everything after the `;` will be ignored until the line end.

e.g.
```asm
ADD 5 0x21          ; This adds 0x21 to register V5
```