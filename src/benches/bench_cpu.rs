use crate::{
    execution::isa::Instruction,
    state::cpu::CpuState,
    state_transition::{Executable, Instance, StateTransition},
    Error,
};

#[derive(Debug, Clone)]
struct LookupCpu {
    instructions: Vec<Instruction>,
    initial: CpuState,
}

impl LookupCpu {
    fn init() -> Self {
        LookupCpu {
            instructions: vec![],
            initial: CpuState::init(),
        }
    }

    fn full_pass(&self) {
        let instances = self.run_all_instructions();

    }

    fn run_all_instructions(&self) -> Vec<LookupInstance> {
        let res = vec![];
        let mut state = self.initial;
        for instr in self.instructions.iter() {
            state = instr.execute(state);
        }
        res
    }
}

#[derive(Debug, Clone)]
struct LookupQuery {}

#[derive(Debug, Clone)]
struct LookupInstance {
    state: CpuState,
    lookup_query: LookupQuery,
}

impl Instance for Vec<LookupInstance> {
    type ProvableObject = Vec<LookupInstance>;
    type ProvedObject = LookupArgument;

    type State = CpuState;
    type Error = Error;

    fn prove(&self) -> Result<Self::ProvedObject, Self::Error> {
        Ok(LookupArgument {  })
    }

    fn verify(&self) -> bool {
        todo!()
    }
}

impl Instance for LookupInstance {
    type ProvableObject = LookupInstance;
    type ProvedObject = LookupArgument;

    type State = CpuState;
    type Error = Error;

    // TODO
    fn prove(&self) -> Result<Self::ProvedObject, Self::Error> {
        Ok(LookupArgument {  })
    }

    fn verify(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct LookupArgument {

}

impl StateTransition for LookupCpu {
    type Executable = Instruction;

    type Provable = LookupInstance;
    type Error = Error;

    fn vstf(
        prev_instance: Vec<Self::Provable>,
        result_instance: Self::Provable,
        execution: Vec<Self::Executable>,
    ) -> bool {
        // Lookup argument
        result_instance.verify()
    }
}

#[cfg(test)]
fn test_full() {
    let cpu = LookupCpu::init();
}
