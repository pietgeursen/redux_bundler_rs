pub type Reactor<State, Action> = fn(&State) -> Option<Action>;
