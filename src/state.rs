pub trait State {
    type Action;

    fn apply(&mut self, action: &Self::Action);
}
