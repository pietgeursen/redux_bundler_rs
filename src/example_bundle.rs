use crate::state::State;

pub enum Action {
    Increment(isize),
    Decrement(isize),
    Reset,
}

pub struct CounterState {
    count: isize,
}

impl State for CounterState {
    type Action = Action;

    fn apply(&mut self, action: &Self::Action) {
        match action {
            Action::Increment(amount) => self.count += amount,
            Action::Decrement(amount) => self.count -= amount,
            Action::Reset => self.count = 0,
        }
    }
}

pub fn select_count(state: &CounterState) -> isize {
    state.count
}

pub fn react_needs_reset(state: &CounterState) -> Option<Action> {
    let count = select_count(state);

    if count >= 10 {
        return Some(Action::Reset);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::example_bundle::*;
    use crate::bundle::*;
    use crate::redux::*;
    #[test]
    fn create_a_bundle() {
        let initial_state = CounterState { count: 0 };

        let mut bundle = Bundle {
            state: initial_state,
            reactors: vec![react_needs_reset],
            subscribers: vec![],
        };

        bundle.dispatch(&Action::Increment(5));

        assert_eq!(bundle.state.count, 5);

        bundle.subscribe(|state|{
            assert!(state.count == 10 || state.count == 0);
        });

        bundle.dispatch(&Action::Increment(5));

        assert_eq!(bundle.state.count, 0);
    }
}
