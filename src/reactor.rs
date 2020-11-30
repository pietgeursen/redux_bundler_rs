use crate::State;

pub trait Reactor<S: State> {
    fn apply(&mut self, state: &S) -> Option<S::Action>;
}
