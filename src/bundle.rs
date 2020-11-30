use crate::reactor::Reactor;
use crate::redux::*;
use crate::state::State;
use std::sync::Arc;

pub struct Bundle<S: State> {
    pub state: Arc<S>,
    pub reactors: Vec<Box<dyn Reactor<S>>>,
    pub subscribers: Vec<Subscriber<S>>,
}

impl<S: State> Redux<S> for Bundle<S> {
    fn get_state(&mut self) -> &S {
        &self.state
    }

    fn dispatch(&mut self, action: &S::Action) {
        Arc::<S>::get_mut(& mut self.state).unwrap().apply(&action);
         
        let state = self.state.clone();

        self.subscribers
            .iter()
            .for_each(|subscriber| subscriber(&self.state));

        let actions: Vec<_> = self
            .reactors
            .iter_mut()
            .map(|reactor| reactor.apply(&state))
            .collect();

        actions.iter().for_each(|action| match action {
            Some(a) => self.dispatch(&a),
            _ => (),
        })
    }

    fn subscribe(&mut self, subscriber: Subscriber<S>) {
        self.subscribers.push(subscriber);
    }
}
