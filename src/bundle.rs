use crate::state::State;
use crate::selector::Reactor;

pub struct Bundle<S: State> {
    pub state: S,
    pub reactors: Vec<Reactor<S, S::Action>>
}

impl<S: State> Bundle<S> {

    pub fn dispatch(&mut self, action: &S::Action) {
        self.state.apply(&action);

        let actions: Vec<_> = self.reactors
            .iter()
            .map(|reactor|{
                reactor(&self.state)
            })
            .collect();

        for action in actions {
            match action {
                Some(a) => self.dispatch(&a),
                _ => ()
            }
        }
            
    }
}
