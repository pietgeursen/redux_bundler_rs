use crate::state::State;

pub type Subscriber<State> = Box<dyn Fn(&State)->() >;

pub trait Redux<S: State> {
    fn dispatch(&mut self, action: &S::Action);
    fn get_state(&mut self) -> &S;
    fn subscribe(&mut self, subscriber: Subscriber<S>);
}
