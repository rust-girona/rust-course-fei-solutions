//! Run this file with `cargo test --test 06_brainfuck_interpreter`.

// TODO (bonus): Create an interpreter for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) language.
// The Brainfuck program will be parsed out of a string and represented as a struct.
//
// Handle both parsing and execution errors using enums representing error conditions,
// see tests for details.
// A parsing error can be either an unknown instruction or an unpaired loop instruction.
// An execution error can be either that the program tries to read input, but there is no more
// input available, or when the program executes more than 10000 instructions (which probably
// signals an infinite loop).
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `ParseError`, `ExecuteError` and `Program`
// (and any other custom types nested inside them) so that asserts in tests work.

use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    UnknownInstruction { location: usize, instruction: char },
    UnmatchedLoop { location: usize },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecuteError {
    NoInputLeft,
    InfiniteLoop,
}

// The #[derive] attribute enables nicer error messages in tests.
#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Increment,
    Decrement,
    Advance,
    Back,
    Output,
    Input,
    Block(Block),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Block {
    instructions: Vec<Instruction>,
    fp: usize, // instruction pointer
}

impl Block {
    fn new(instructions: Vec<Instruction>) -> Self {
        Block {
            instructions: instructions,
            fp: 0,
        }
    }

    fn execute(&self, exe: &mut Execution) -> Result<(), ExecuteError> {
        let mut fp: usize = 0;
        //let instructions = self.instructions.clone();
        while let Some(instruction) = self.instructions.get(fp) {
            match instruction {
                Instruction::Increment => exe.data[exe.dp] += 1,
                Instruction::Decrement => exe.data[exe.dp] -= 1,
                Instruction::Advance => exe.dp += 1,
                Instruction::Back => exe.dp -= 1,
                Instruction::Output => {
                    exe.output.push(exe.data[exe.dp] as u8);
                    exe.ip += 1;
                }
                Instruction::Input => {
                    match exe.input.get(exe.ip) {
                        Some(c) => exe.data[exe.dp] = *c as u8,
                        None => return Err(ExecuteError::NoInputLeft),
                    }
                    exe.ip += 1;
                }
                Instruction::Block(block) => {
                    while exe.data[exe.dp] != 0 {
                        block.execute(exe)?;
                        exe.ni += 1;
                        if exe.ni >= 10000 {
                            return Err(ExecuteError::InfiniteLoop);
                        }
                    }
                }
            }
            fp += 1;
            exe.ni += 1;
            if exe.ni >= 10000 {
                return Err(ExecuteError::InfiniteLoop);
            }
        }
        Ok(())
    }
}

struct Execution<'a> {
    input: &'a Vec<u8>,
    data: &'a mut Vec<u8>,
    output: Vec<u8>,
    ip: usize, // input pointer
    dp: usize, // data pointer
    ni: u32,   // number of instructions executed
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn execute(&self, input: Vec<u8>, mut data: Vec<u8>) -> Result<String, ExecuteError> {
        let mut exe = Execution {
            input: &input,
            data: &mut data,
            output: Vec::new(),
            ip: 0,
            dp: 0,
            ni: 0,
        };
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Increment => exe.data[exe.dp] += 1,
                Instruction::Decrement => exe.data[exe.dp] -= 1,
                Instruction::Advance => exe.dp += 1,
                Instruction::Back => exe.dp -= 1,
                Instruction::Output => {
                    exe.output.push(exe.data[exe.dp] as u8);
                }
                Instruction::Input => {
                    match exe.input.get(exe.ip) {
                        Some(c) => exe.data[exe.dp] = *c as u8,
                        None => {
                            return Err(ExecuteError::NoInputLeft);
                        }
                    }
                    exe.ip += 1;
                }
                Instruction::Block(block) => {
                    while exe.data[exe.dp] != 0 {
                        block.execute(&mut exe)?;
                        exe.ni += 1;
                        if exe.ni >= 10000 {
                            return Err(ExecuteError::InfiniteLoop);
                        }
                    }
                }
            }
            exe.ni += 1;
            if exe.ni >= 10000 {
                return Err(ExecuteError::InfiniteLoop);
            }
        }
        Ok(String::from_utf8(exe.output).expect("Failed to convert output to string"))
    }
}

fn parse_block(chars: &mut Chars, fp: &mut usize) -> Result<Block, ParseError> {
    let mut instructions = Vec::new();
    let loop_start = *fp;
    while let Some(c) = chars.next() {
        match c {
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '>' => instructions.push(Instruction::Advance),
            '<' => instructions.push(Instruction::Back),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => instructions.push(parse_block(chars, fp).map(Instruction::Block)?),
            ']' => {
                *fp += 1;
                return Ok(Block::new(instructions));
            }
            _ => {
                return Err(ParseError::UnknownInstruction {
                    location: *fp,
                    instruction: c,
                });
            }
        }
        *fp += 1;
    }
    Err(ParseError::UnmatchedLoop {
        location: loop_start,
    })
}

pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let mut instructions = Vec::new();
    let mut chars = input.chars();
    let mut fp = 0;

    while let Some(c) = chars.next() {
        match c {
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '>' => instructions.push(Instruction::Advance),
            '<' => instructions.push(Instruction::Back),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => instructions.push(parse_block(&mut chars, &mut fp).map(Instruction::Block)?),
            /* Left this as a reminder of the improvement suggested
            '[' => {
                let result = parse_block(&mut chars, &mut fp);
                match result {
                    Ok(block) => instructions.push(Instruction::Block(block)),
                    Err(err) => return Err(err),
                }
            }
            */
            ']' => {
                return Err(ParseError::UnmatchedLoop { location: fp });
            }
            _ => {
                return Err(ParseError::UnknownInstruction {
                    location: fp,
                    instruction: c,
                });
            }
        }
        fp += 1;
    }
    Ok(Program {
        instructions: instructions,
    })
}

use std::io;

fn main() {
    loop {
        let mut program_line = String::new();
        print!("Please enter program line (or ctrl+c to exit): \n");
        let stdin = io::stdin();
        match stdin.read_line(&mut program_line) {
            Ok(_) => (),
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        }
        let program;
        match parse_program(&program_line.trim()) {
            Ok(parsed) => {
                println!("Program parsed successfully");
                program = parsed;
            }
            Err(err) => {
                println!("Error: {:#?}", err);
                continue;
            }
        }
        let mut input = String::new();
        print!("Please enter input line (or ctrl+c to exit): \n");
        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        }
        let input_str = input.trim();
        match program.execute(input_str.to_string().into_bytes(), vec![0; 30000]) {
            Ok(output) => {
                println!("Program Output:\n {}", output);
            }
            Err(err) => {
                println!("Error: {:#?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecuteError, ParseError, parse_program};

    #[test]
    fn parse_empty() {
        check_output("", "", "");
    }

    #[test]
    fn parse_unknown_instruction() {
        assert!(matches!(
            parse_program(">p"),
            Err(ParseError::UnknownInstruction {
                location: 1,
                instruction: 'p'
            })
        ));
    }

    #[test]
    fn parse_unmatched_loop_start() {
        assert_eq!(
            parse_program(">++[+>][++>"),
            Err(ParseError::UnmatchedLoop { location: 7 })
        );
    }

    #[test]
    fn parse_unmatched_loop_end() {
        assert_eq!(
            parse_program(">++[+>][++>]+]"),
            Err(ParseError::UnmatchedLoop { location: 13 })
        );
    }

    #[test]
    fn missing_input() {
        let program = parse_program(",").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::NoInputLeft));
    }

    #[test]
    fn infinite_loop() {
        let program = parse_program("+[]").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::InfiniteLoop));
    }

    #[test]
    fn copy_input() {
        check_output(",.>,.>,.>,.>,.", "hello", "hello");
    }

    #[test]
    fn output_exclamation_mark() {
        check_output("+++++++++++++++++++++++++++++++++.", "", "!");
    }

    #[test]
    fn three_exclamation_marks() {
        check_output(">+++++++++++++++++++++++++++++++++<+++[>.<-]", "", "!!!");
    }

    #[test]
    fn hello_world() {
        check_output(
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
            "",
            "Hello World!\n",
        );
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .expect(&format!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}
