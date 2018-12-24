pub mod bundle;
pub mod example_bundle;
pub mod selector;
pub mod state;

#[cfg(test)]
mod tests {
    use crate::selector::Selector;
    #[test]
    fn selector() {
        let s: Selector<u64, u64> = |i| i * 2;
    }
}
