use std::fmt::Debug;

pub trait State: Clone + Debug {}

pub trait Executable: Clone + Debug {
    type State: State;
    fn execute(&self, state: Self::State) -> Self::State;
}

pub trait Instance: Clone + Debug {
    type ProvableObject: Clone + Debug;
    type ProvedObject: Clone + Debug;
    type Error: Clone + Debug;
    type State: Clone + Debug;

    fn prove(&self) -> Result<Self::ProvedObject, Self::Error>;
    fn verify(&self) -> bool;
}

pub trait StateTransition: Clone + Debug {
    type Provable: Instance;
    type Executable: Clone + Debug;

    type Error: Clone + Debug;

    // if it uses folding or recursive method,
    // then the number of element of each parameter should be 1
    // And run this vstf for all steps
    fn vstf(
        input_instance: Vec<Self::Provable>,
        result_instance: Self::Provable,
        execution: Vec<Self::Executable>,
    ) -> bool;
}
