use crate::state::cpu::CpuState;
use crate::state_transition::Executable;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    BinOp { op: BinaryOperation },
    Branch(Branch),
    Jump(usize),
    Load,
    Save,
}

impl Executable for Instruction {
    type State = CpuState;
    fn execute(&self, state: Self::State) -> Self::State {
        match self {
            Self::Nop => state,
            _ => todo!(),
        }
    }
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
