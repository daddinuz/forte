use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem;

use crate::opcode::Opcode;
use crate::program::Program;

#[derive(Default)]
pub struct Parser {
    brackets: usize,
    braces: usize,
    line: usize,
    col: usize,
    was_digit: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, stream: impl IntoIterator<Item = char>) -> Result<Program, ParseError> {
        let mut program = Program::new();

        for c in stream.into_iter() {
            match self.step(c) {
                Ok(Some(opcode)) => program.push(opcode),
                Err(error) => return Err(error),
                _ => continue,
            }
        }

        Ok(program)
    }

    pub fn step(&mut self, c: char) -> Result<Option<Opcode>, ParseError> {
        let was_digit = mem::replace(&mut self.was_digit, false);

        let result = match c {
            '\n' => {
                self.line += 1;
                self.col = 0;
                Ok(None)
            }

            '0' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold0
                } else {
                    Opcode::Push0
                }))
            }
            '1' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold1
                } else {
                    Opcode::Push1
                }))
            }
            '2' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold2
                } else {
                    Opcode::Push2
                }))
            }
            '3' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold3
                } else {
                    Opcode::Push3
                }))
            }
            '4' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold4
                } else {
                    Opcode::Push4
                }))
            }
            '5' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold5
                } else {
                    Opcode::Push5
                }))
            }
            '6' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold6
                } else {
                    Opcode::Push6
                }))
            }
            '7' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold7
                } else {
                    Opcode::Push7
                }))
            }
            '8' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold8
                } else {
                    Opcode::Push8
                }))
            }
            '9' => {
                self.was_digit = true;

                Ok(Some(if was_digit {
                    Opcode::Fold9
                } else {
                    Opcode::Push9
                }))
            }

            '-' if was_digit => Ok(Some(Opcode::Neg)),
            '+' => Ok(Some(Opcode::Add)),
            '-' => Ok(Some(Opcode::Sub)),
            '*' => Ok(Some(Opcode::Mul)),
            '/' => Ok(Some(Opcode::Div)),
            '%' => Ok(Some(Opcode::Rem)),

            '=' => Ok(Some(Opcode::Eq)),
            '>' => Ok(Some(Opcode::Gt)),
            '<' => Ok(Some(Opcode::Lt)),

            '~' => Ok(Some(Opcode::BitNot)),
            '&' => Ok(Some(Opcode::BitAnd)),
            '^' => Ok(Some(Opcode::BitXor)),
            '|' => Ok(Some(Opcode::BitOr)),
            '«' => Ok(Some(Opcode::BitShl)),
            '»' => Ok(Some(Opcode::BitShr)),

            '.' => Ok(Some(Opcode::Pop)),
            '_' => Ok(Some(Opcode::Dup)),
            ',' => Ok(Some(Opcode::Swap)),

            '?' => Ok(Some(Opcode::Ask)),
            '!' => Ok(Some(Opcode::Say)),
            '¡' => Ok(Some(Opcode::Print)),

            '[' => {
                self.brackets += 1;
                Ok(Some(Opcode::While))
            }
            ']' => {
                if self.brackets > 0 {
                    self.brackets -= 1;
                    Ok(Some(Opcode::Until))
                } else {
                    Err(ParseError::new(self.line, self.col, "mismatching `]`"))
                }
            }

            '{' => {
                self.braces += 1;
                Ok(Some(Opcode::Let))
            }
            '}' => {
                if self.braces > 0 {
                    self.braces -= 1;
                    Ok(Some(Opcode::End))
                } else {
                    Err(ParseError::new(
                        self.line,
                        self.col,
                        "At {}:{}: mismatching `}}`",
                    ))
                }
            }

            '@' => Ok(Some(Opcode::Call)),
            '$' => Ok(Some(Opcode::Ret)),

            '§' => Ok(Some(Opcode::Halt)),

            _ => Ok(None),
        };

        self.col += 1;
        result
    }
}

pub fn parse(stream: impl IntoIterator<Item = char>) -> Result<Program, ParseError> {
    let mut parser = Parser::new();
    parser.parse(stream)
}

#[derive(Debug)]
pub struct ParseError(Box<dyn Error>);

impl ParseError {
    fn new(line: usize, col: usize, message: &str) -> Self {
        Self(format!("At '{line}:{col}': {message}").into())
    }

    pub fn into_inner(self) -> Box<dyn Error> {
        self.0
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}
