#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Left(usize),
    Right(usize),
    Plus(usize),
    Minus(usize),
    GetChar,
    PutChar,
    LoopStart(usize),
    LoopEnd(usize),
}
