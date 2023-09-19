use crate::state_transition::State;

#[derive(Clone, Debug, Copy)]
pub struct CpuState {
    pub pc: usize,
}

impl CpuState {
    pub fn init() -> Self {
        CpuState { pc: 0 }
    }
}

impl State for CpuState {}
