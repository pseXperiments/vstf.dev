use std::fmt::Debug;

pub trait Instance: Clone + Debug {
    type ProvableObject: Clone + Debug;
    type ProvedObject: Clone + Debug;
    type Error: Clone + Debug;

    fn prove(instance: &Self::ProvableObject) -> Result<Self::ProvedObject, Self::Error>;
    fn verify(instance: &Self::ProvedObject) -> bool;
}

pub trait StateTransition: Clone + Debug {
    type State: Clone + Debug;
    type Provable: Instance;
    type Executable: Clone + Debug;

    type Error: Clone + Debug;

    fn single_stf(
        state: Self::State,
        prev_instance: Option<Self::Provable>,
        execution: &Self::Executable,
    ) -> (Option<Self::Provable>, Self::State);

    fn full_stf(
        initial_state: Self::State,
        executions: Vec<Self::Executable>,
    ) -> Result<(Self::Provable, Self::State), Self::Error>;

    fn verify_stf(proof: Self::Provable, executions: Vec<Self::Executable>) -> bool;
}
