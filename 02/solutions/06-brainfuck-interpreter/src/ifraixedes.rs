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

/// Below you can find a set of unit tests.
use std::borrow::Cow;

const MAX_NUMBER_EXECTUED_INSTRUCTIONS: u32 = 10_000;

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    UnknownInstruction { location: i32, instruction: char },
    UnmatchedLoop { location: i32 },
}

#[derive(Debug, Eq, PartialEq)]
enum ExecuteError {
    NoInputLeft,
    InfiniteLoop,
}

#[derive(Debug, Eq, PartialEq)]
struct Program {
    code: Vec<char>,
}

impl Program {
    pub fn execute<'a>(
        self,
        input: Vec<u8>,
        mut data: Vec<u8>,
    ) -> Result<Cow<'a, str>, ExecuteError> {
        if self.code.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        let mut inst_idx = 0;
        let mut inst_input = 0;
        let mut data_idx = 0;
        let mut loop_stack = Vec::new();
        let mut output_bytes = Vec::new();
        let mut executed_instructions: i64 = -1;

        while inst_idx < self.code.len() {
            executed_instructions += 1;

            // NOTE this a loose way to detect an infinite loop. It's just here because it's a
            // suggestion in the exercise's statement.
            if executed_instructions >= MAX_NUMBER_EXECTUED_INSTRUCTIONS as i64 {
                return Err(ExecuteError::InfiniteLoop);
            }

            let inst = self.code[inst_idx];
            match inst {
                '>' => {
                    data_idx += 1;
                    if data_idx >= data.len() {
                        // NOTE instead of panicking, we could assume that once the top level is
                        // reached it doesn't go further up and it stays at the top limit.
                        panic!("invalid program, data access out of range, calculated index \
                        '{data_idx}' from maximum size '{}'. Instruction {inst} at index {inst_idx}",
                        data.len());
                    }
                },
                '<' => {
                    // NOTE instead of panicking, we could assume that once the bottom level is
                    // reached it doesn't go further down and it stays at the bottom limit.
                    if data_idx == 0 {
                        panic!("invalid program, data access out of range, calculated index '-1.' \
                        Instruction {inst} at index {inst_idx}");
                    }

                    data_idx -= 1;
                },
                '+' => data[data_idx] = data[data_idx].saturating_add(1),
                '-' => data[data_idx] = data[data_idx].saturating_sub(1),
                '.' => output_bytes.push(data[data_idx]),
                ',' => {
                    if let Some(i) = input.get(inst_input) {
                        inst_input += 1;
                        data[data_idx] = *i;
                    } else {
                        return Err(ExecuteError::NoInputLeft);
                    }
                },
                '[' => {
                    if data[data_idx] == 0 {
                        let mut opened: usize = 0;
                        let mut loop_end_idx = None;
                        for (idx, inst) in self.code[inst_idx+1..].iter().enumerate() {
                            match inst {
                                '[' => opened = opened.saturating_add(1),
                                ']' if opened > 0 =>  opened = opened.saturating_sub(1),
                                ']' if opened == 0 => loop_end_idx = Some(idx),
                                _ => {},
                            };
                        }

                        if let Some(idx) = loop_end_idx {
                            // +2 because the slice started at the next instruction after '[' which
                            // is at index 0 and we want to follow with instruction after ']'.
                            inst_idx += idx + 2;
                        } else {
                            panic!("invalid program, unmatched loop, open at index '{inst_idx}");
                        }

                        continue;
                    }

                    loop_stack.push(inst_idx);
                },
                ']' => {
                    if data[data_idx] != 0 {
                        if let Some(idx) = loop_stack.last() {
                            // Detect empty infinite loops. This is the instructions `[]` when the
                            // pointed data isn't 0.
                            // This only detect this specific case that can cause an infinite loop,
                            // but there are many other ways.
                            if idx+1 == inst_idx {
                                return Err(ExecuteError::InfiniteLoop);
                            }

                            // +1 because we want to follow with the instruction after the '['.
                            inst_idx = idx+1;
                        } else {
                            panic!("invalid program, unmatched loop, closed at index '{inst_idx}");
                        }

                        continue;
                    }

                    loop_stack.pop().expect("invalid program, unmatched loop, not start found");
                },
                _ =>  panic!("invalid program, found {inst}, this should never happen if Program was created with the parse_program function"),
            };

            inst_idx += 1;
        }

        Ok(Cow::Owned(String::from_utf8(output_bytes).expect(
            "invalid output because contains invalid UTF-8 characters",
        )))
    }
}

fn parse_program(program_text: &str) -> Result<Program, ParseError> {
    let code = program_text.chars();

    let mut loop_stack = Vec::new();
    for (i, c) in code.enumerate() {
        match c {
            '[' => loop_stack.push(i),
            ']' => {
                if loop_stack.pop().is_none() {
                    return Err(ParseError::UnmatchedLoop { location: i as i32 });
                }
            }
            '>' | '<' | '+' | '-' | '.' | ',' => {}
            _ => {
                return Err(ParseError::UnknownInstruction {
                    location: i as i32,
                    instruction: c,
                })
            }
        };
    }

    if !loop_stack.is_empty() {
        Err(ParseError::UnmatchedLoop {
            location:  loop_stack.pop()
                .expect("BUG: impossible that this happens because we have checked in the condition that isn't empty before poping it")
                as i32,
        })
    } else {
        Ok(Program {
            code: program_text.chars().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_program, ExecuteError, ParseError};

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
        check_output("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "", "Hello World!\n");
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .unwrap_or_else(|_| panic!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}
