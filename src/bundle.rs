use crate::state::State;
use crate::reactor::Reactor;
use crate::redux::*;

pub struct Bundle<S: State> {
    pub state: S,
    pub reactors: Vec<Reactor<S, S::Action>>,
    pub subscribers: Vec<Subscriber<S>>
}

impl<S: State> Redux<S> for Bundle<S> {

    fn get_state(&mut self) -> &S {
        &self.state
    }

    fn dispatch(&mut self, action: &S::Action) {

        self.state.apply(&action);

        self.subscribers
            .iter()
            .for_each(|subscriber| subscriber(&self.state));

        let actions: Vec<_> = self.reactors
            .iter()
            .map(|reactor|{
                reactor(&self.state)
            })
            .collect();

        actions
            .iter()
            .for_each(|action|{
                match action {
                    Some(a) => self.dispatch(&a),
                    _ => ()
                }
            })
            
    }

    fn subscribe(&mut self, subscriber: Subscriber<S>){
        self.subscribers.push(subscriber);
    }
}
