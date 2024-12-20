//  brainfuck has a few characters for it's operation
// > : move the pointer to the next
// < : move the pointer to the previous cell
// + : increase the cell from the pointer by 1
// - : decrease the cell from the pointer by 1
// . : output the ascii rep of the current cell
// , : get input from user

use core::str;
use std::{
    env,
    fs::File,
    io::{self, stdout, Error, Read, Write},
};

use getch::Getch;

// brainfuck operators
const COMMANDS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

struct Interpreter {
    max_pointer: usize,
    pointer: usize,
    cells: [u8; 30000],
    program_counter: usize,
    loop_program_pointer: Vec<usize>,
}

impl Interpreter {
    fn new() -> Self {
        return Interpreter {
            max_pointer: 0, // maximum accessed pointer throughout the program
            pointer: 0,
            program_counter: 0,
            cells: [0; 30000],
            loop_program_pointer: vec![],
        };
    }

    fn excecute_str(&mut self, code: &str) -> io::Result<Vec<u8>> {
        // remove lines beginning with ; for line comment
        let line_comment_striped: Vec<&str> =
            code.split('\n').filter(|x| !x.starts_with('#')).collect();
        let line_comment_striped: String = line_comment_striped.concat();

        let program = Vec::from_iter(
            line_comment_striped
                .chars()
                .into_iter()
                .filter(|x| COMMANDS.contains(x)),
        );
        let mut output: Vec<u8> = vec![];

        return loop {
            if let Some(command) = program.get(self.program_counter) {
                match command {
                    '>' => {
                        self.pointer += 1;
                        self.max_pointer = if self.pointer > self.max_pointer {
                            self.pointer
                        } else {
                            self.max_pointer
                        };
                    }
                    '<' => {
                        self.pointer -= 1;
                    }
                    '+' => {
                        self.cells[self.pointer] += 1;
                    }
                    '-' => {
                        self.cells[self.pointer] -= 1;
                    }
                    '.' => {
                        let out = self.cells[self.pointer];
                        let mut handler = stdout().lock();
                        handler.write(&[out])?;
                        handler.flush()?;

                        output.push(out);
                    }
                    ',' => {
                        let g = Getch::new();

                        match g.getch() {
                            Ok(input) => {
                                self.cells[self.pointer] = input;
                            }
                            Err(_e) => {
                                println!("Error getting character from standard input");
                                panic!();
                            }
                        }
                    }
                    '[' => {
                        self.loop_program_pointer.push(self.program_counter);
                    }
                    ']' => {
                        if self.cells[self.pointer] == 0 {
                            self.loop_program_pointer.pop();
                        } else {
                            self.program_counter = self.loop_program_pointer.last().unwrap() + 1;
                            continue;
                        }
                    }
                    _ => {}
                }

                self.program_counter += 1;
            } else {
                // end of program
                break Ok(output);
            }
        };
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(file_path) = args.get(1) {
        let mut file_handler = File::open(file_path)?;
        let mut buffer: Vec<u8> = vec![];

        file_handler.read_to_end(&mut buffer)?;
        let source_code = str::from_utf8(&buffer).map_err(|_e| {
            Error::new(
                io::ErrorKind::InvalidData,
                "Invalid source code encoding; expected utf8 ",
            )
        })?;

        let mut interpreter = Interpreter::new();

        match interpreter.excecute_str(source_code) {
            Ok(out) => {
                if let Some(val) = args.get(2) {
                    match val.as_str() {
                        "--debug" => {
                            println!(
                                "\n+------------\noutput: {:?}\ncells: {:?}\ncurrent pointer location: {:?}\nlast pointer location: {:?}",
                                out,
                                &interpreter.cells[1..interpreter.max_pointer],
                                interpreter.pointer,
                                interpreter.max_pointer
                            );
                        }
                        n => {
                            println!("Invalid argument {n}");
                        }
                    }
                } else {
                    println!();
                }
            }
            Err(_err) => {
                println!("Error running fuck.")
            }
        }
    } else {
        let program_name = args.get(0).unwrap().to_owned();
        println!("Usage {program_name} <filename.bf> [--debug]")
    }

    return Ok(());
}
