use crate::{
    execution::isa::Instruction,
    state::cpu::State,
    state_transition::{Instance, StateTransition},
};

#[derive(Debug, Clone)]
struct LookupCpu();

#[derive(Debug, Clone)]
struct LookupInstance {}

#[derive(Debug, Clone)]
enum LookupError {
    Misc,
}

#[derive(Debug, Clone)]
enum StateTransitionError {
    Misc,
}

impl Instance for LookupInstance {
    type ProvableObject = LookupInstance;
    type ProvedObject = LookupInstance;

    type Error = LookupError;

    // TODO
    fn prove(instance: &Self::ProvableObject) -> Result<Self::ProvedObject, Self::Error> {
        Ok(instance.clone())
    }

    fn verify(instance: &Self::ProvedObject) -> bool {
        todo!()
    }
}

impl StateTransition for LookupCpu {
    type Executable = Instruction;
    type State = State;
    type Provable = LookupInstance;
    type Error = StateTransitionError;

    fn full_stf(
        state: Self::State,
        executions: Vec<Self::Executable>,
    ) -> Result<(Self::Provable, Self::State), Self::Error> {
        let mut curr_state = state;
        let mut instance = None;
        for exec in executions.iter() {
            (instance, curr_state) = Self::single_stf(curr_state, instance, exec);
        }
        let final_instance = match instance {
            Some(i) => i,
            None => return Err(StateTransitionError::Misc),
        };
        Ok((final_instance, curr_state))
    }

    fn single_stf(
        state: Self::State,
        instance: Option<Self::Provable>,
        execution: &Self::Executable,
    ) -> (Option<Self::Provable>, Self::State) {
        todo!()
    }

    fn verify_stf(proof: Self::Provable, executions: Vec<Self::Executable>) -> bool {
        todo!()
    }
}
