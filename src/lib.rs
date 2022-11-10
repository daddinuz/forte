#![allow(clippy::unit_arg)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};
use std::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};
use std::slice;

use opcode::Opcode;
use program::Program;

pub mod opcode;
pub mod parser;
pub mod program;

#[derive(Default)]
#[repr(align(64))]
pub struct Vm {
    dictionary: BTreeMap<i32, usize>,
    call_stack: Vec<usize>,
    loop_stack: Vec<u32>,
    data_stack: Vec<i32>,
    code: Vec<Opcode>,
    pc: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(program: Program) -> Self {
        let code = program.into_inner();

        Self {
            code,
            ..Default::default()
        }
    }

    pub fn extend_program(&mut self, program: Program) {
        self.code.append(&mut program.into_inner());
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.pc < self.code.len() {
            let opcode = self.fetch();
            self.execute(opcode)?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        if self.pc < self.code.len() {
            let opcode = self.fetch();
            return self.execute(opcode);
        }

        Ok(())
    }

    fn execute(&mut self, opcode: Opcode) -> Result<(), RuntimeError> {
        match opcode {
            Opcode::Push0 => Ok(self.data_stack.push(0)),
            Opcode::Push1 => Ok(self.data_stack.push(1)),
            Opcode::Push2 => Ok(self.data_stack.push(2)),
            Opcode::Push3 => Ok(self.data_stack.push(3)),
            Opcode::Push4 => Ok(self.data_stack.push(4)),
            Opcode::Push5 => Ok(self.data_stack.push(5)),
            Opcode::Push6 => Ok(self.data_stack.push(6)),
            Opcode::Push7 => Ok(self.data_stack.push(7)),
            Opcode::Push8 => Ok(self.data_stack.push(8)),
            Opcode::Push9 => Ok(self.data_stack.push(9)),

            Opcode::Fold0 => self.unary(|v| *v *= 10),
            Opcode::Fold1 => self.unary(|v| *v = *v * 10 + 1),
            Opcode::Fold2 => self.unary(|v| *v = *v * 10 + 2),
            Opcode::Fold3 => self.unary(|v| *v = *v * 10 + 3),
            Opcode::Fold4 => self.unary(|v| *v = *v * 10 + 4),
            Opcode::Fold5 => self.unary(|v| *v = *v * 10 + 5),
            Opcode::Fold6 => self.unary(|v| *v = *v * 10 + 6),
            Opcode::Fold7 => self.unary(|v| *v = *v * 10 + 7),
            Opcode::Fold8 => self.unary(|v| *v = *v * 10 + 8),
            Opcode::Fold9 => self.unary(|v| *v = *v * 10 + 9),

            Opcode::Neg => self.unary(|v| *v = -*v),
            Opcode::Add => self.binary(AddAssign::add_assign),
            Opcode::Sub => self.binary(SubAssign::sub_assign),
            Opcode::Mul => self.binary(MulAssign::mul_assign),
            Opcode::Div => self.binary(DivAssign::div_assign),
            Opcode::Rem => self.binary(RemAssign::rem_assign),

            Opcode::Eq => self.binary(|v, w| *v = (*v == w) as i32),
            Opcode::Gt => self.binary(|v, w| *v = (*v > w) as i32),
            Opcode::Lt => self.binary(|v, w| *v = (*v < w) as i32),

            Opcode::BitNot => self.unary(|v| *v = !*v),
            Opcode::BitAnd => self.binary(BitAndAssign::bitand_assign),
            Opcode::BitXor => self.binary(BitXorAssign::bitxor_assign),
            Opcode::BitOr => self.binary(BitOrAssign::bitor_assign),
            Opcode::BitShl => self.binary(ShlAssign::shl_assign),
            Opcode::BitShr => self.binary(ShrAssign::shr_assign),

            Opcode::Pop => self
                .data_stack
                .pop()
                .map(|_| ())
                .ok_or(RuntimeError::StackUnderflow),
            Opcode::Dup => self
                .data_stack
                .last()
                .copied()
                .map(|v| self.data_stack.push(v))
                .ok_or(RuntimeError::StackUnderflow),
            Opcode::Swap => {
                if self.data_stack.len() < 2 {
                    return Err(RuntimeError::StackUnderflow);
                }

                let top = self.data_stack.len() - 1;
                self.data_stack.swap(top, top - 1);
                Ok(())
            }

            Opcode::Ask => {
                let mut stdin = io::stdin();
                let mut c = 0;

                stdin
                    .read_exact(slice::from_mut(&mut c))
                    .map_err(|_| RuntimeError::IoError)
                    .map(|_| self.data_stack.push(c as i32))
            }
            Opcode::Say => self
                .data_stack
                .pop()
                .map(|v| {
                    let c = char::from_u32(v as u32).unwrap_or(char::REPLACEMENT_CHARACTER);
                    print!("{c}")
                })
                .ok_or(RuntimeError::StackUnderflow),
            Opcode::Print => self
                .data_stack
                .pop()
                .map(|v| println!("{v}"))
                .ok_or(RuntimeError::StackUnderflow),

            Opcode::While => {
                let guard = self.data_stack.pop().ok_or(RuntimeError::StackUnderflow)?;

                if guard != 0 {
                    self.loop_stack.push(guard.unsigned_abs());
                } else {
                    let mut acc: usize = 1;
                    let offset = self.code[self.pc..]
                        .iter()
                        .position(|opcode| {
                            match opcode {
                                Opcode::While => acc += 1,
                                Opcode::Until => acc -= 1,
                                _ => (),
                            }

                            acc == 0
                        })
                        .unwrap();

                    self.pc += offset + 1;
                }

                Ok(())
            }
            Opcode::Until => {
                let guard = self
                    .loop_stack
                    .last_mut()
                    .map(|v| {
                        *v -= 1;
                        *v
                    })
                    .unwrap();

                if guard == 0 {
                    self.loop_stack.pop();
                } else {
                    let mut acc: usize = 1;
                    let offset = self.code[..self.pc - 1]
                        .iter()
                        .rev()
                        .position(|opcode| {
                            match opcode {
                                Opcode::While => acc -= 1,
                                Opcode::Until => acc += 1,
                                _ => (),
                            }

                            acc == 0
                        })
                        .unwrap();

                    self.pc -= offset + 1;
                }

                Ok(())
            }

            Opcode::Let => {
                let index = self.data_stack.pop().ok_or(RuntimeError::StackUnderflow)?;

                let mut acc: usize = 1;
                let offset = self.code[self.pc..]
                    .iter()
                    .position(|opcode| {
                        match opcode {
                            Opcode::Let => acc += 1,
                            Opcode::End => acc -= 1,
                            _ => (),
                        }

                        acc == 0
                    })
                    .unwrap();

                self.dictionary.insert(index, self.pc);
                self.pc += offset + 1;
                Ok(())
            }

            Opcode::End | Opcode::Ret => {
                self.pc = self.call_stack.pop().unwrap_or(usize::MAX);
                Ok(())
            }
            Opcode::Call => {
                let index = self.data_stack.pop().ok_or(RuntimeError::StackUnderflow)?;

                if let Some(pc) = self.dictionary.get(&index).copied() {
                    self.call_stack.push(self.pc);
                    self.pc = pc;
                }

                Ok(())
            }

            Opcode::Halt => Ok(self.pc = usize::MAX),
        }
    }

    pub fn unwind(&mut self) {
        self.call_stack.clear();
        self.loop_stack.clear();
        self.pc = self.code.len();
    }

    fn fetch(&mut self) -> Opcode {
        let opcode = self.code[self.pc];
        self.pc += 1;
        opcode
    }

    fn unary(&mut self, f: impl FnOnce(&mut i32)) -> Result<(), RuntimeError> {
        self.data_stack
            .last_mut()
            .map(f)
            .ok_or(RuntimeError::StackUnderflow)
    }

    fn binary(&mut self, f: impl FnOnce(&mut i32, i32)) -> Result<(), RuntimeError> {
        match self.data_stack[..] {
            [.., ref mut v, w] => {
                f(v, w);
                self.data_stack.pop();
                Ok(())
            }
            _ => Err(RuntimeError::StackUnderflow),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    StackUnderflow,
    IoError,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::IoError => write!(f, "IO error"),
        }
    }
}

impl Error for RuntimeError {}
