#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Push0,
    Push1,
    Push2,
    Push3,
    Push4,
    Push5,
    Push6,
    Push7,
    Push8,
    Push9,

    Fold0,
    Fold1,
    Fold2,
    Fold3,
    Fold4,
    Fold5,
    Fold6,
    Fold7,
    Fold8,
    Fold9,

    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Eq,
    Gt,
    Lt,

    BitNot,
    BitAnd,
    BitXor,
    BitOr,
    BitShl,
    BitShr,

    Pop,
    Dup,
    Swap,

    Ask,
    Say,
    Print,

    While,
    Until,

    Let,
    End,

    Call,
    Ret,

    Halt,
}
