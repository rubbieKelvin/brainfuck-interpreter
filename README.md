# Brainfuck Interpreter

A simple and efficient Brainfuck interpreter implemented in Rust. Brainfuck is a minimalist programming language with a small set of commands, designed to challenge and amuse programmers.

## Features

- Supports all Brainfuck commands: `>`, `<`, `+`, `-`, `.`, `,`, `[`, and `]`.
- Can execute Brainfuck programs from a file.
- Optionally outputs debug information.

## Brainfuck Commands

| Command | Description                                   |
|---------|-----------------------------------------------|
| `>`     | Move the pointer to the next cell            |
| `<`     | Move the pointer to the previous cell        |
| `+`     | Increase the value at the current cell by 1  |
| `-`     | Decrease the value at the current cell by 1  |
| `.`     | Output the ASCII representation of the current cell |
| `,`     | Get a single byte of input from the user     |
| `[`     | Start a loop; execute if the current cell is non-zero |
| `]`     | End a loop; jump back if the current cell is non-zero |

## Usage

### Building the Interpreter

To build the interpreter, ensure you have [Rust installed](https://www.rust-lang.org/tools/install). Clone this repository and run:

```bash
cargo build --release
```

### Running a Brainfuck Program

Provide the filename of the Brainfuck program as an argument:

```bash
./brainfuck-interpreter <filename.bf>
```

### Debugging Mode

To see the raw output as a vector of bytes, use the --debug flag:

```bash
./brainfuck-interpreter <filename.bf> --debug
```

### Example

Given a file hello.bf containing:

```brainfuck
+[>[<+>-]++++++++[<++++++>-]<.]
```

Run the program:

```bash
./brainfuck-interpreter hello.bf
# Expected output:
# H
```

### Error Handling

The interpreter validates input files and checks for invalid commands.
If the program encounters unmatched loops ([ and ]), it may produce unintended behavior.

### License

This project is open-source and available under the MIT License.

### Contributing

Feel free to open issues or submit pull requests for improvements and new features.

Happy Brainfuck coding hehehe 🤣!
