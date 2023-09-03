#[derive(Debug, Clone)]
pub enum Instruction {
    BinOp { op: BinaryOperation },
    Branch(Branch),
    Jump(usize),
    Load,
    Save,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch {
    Beq,
    Bne,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    Temp(usize),
}
