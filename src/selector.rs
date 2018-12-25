/// `Selector` is a function that takes a reference to some type and returns some new type.
pub type Selector<State, Selected> = fn(&State) -> Selected;

/// `create_selector` is conceptually the same as the [reselect `create_selector`](https://github.com/reduxjs/reselect#createselectorinputselectors--inputselectors-resultfunc) **except**:
/// - the result func is the first argument (not the last), followed by the input selectors.
/// - it doesn't support passing the input selectors as an array.
///
/// # Example
/// ```
/// # use redux_bundler_rs::selector::{ Selector };
/// # use redux_bundler_rs::create_selector;
/// struct Person {
///     age: u8,
///     kind: bool
/// }
///
/// let p = Person { age: 35, kind: true };
///
/// let age_selector: Selector<Person, u8> = |person| person.age;
/// let kind_selector: Selector<Person, bool> = |person| person.kind;
///
/// let is_kind_adult_selector = create_selector!(|age, kind| age >= 18 && kind, age_selector, kind_selector);
///
/// assert!(is_kind_adult_selector(&p));
/// ```
#[macro_export]
macro_rules! create_selector {
    ( $lambda:expr, $($selector:expr),* ) => {
        {
            |state|{
                ($lambda)($(
                        $selector(state),
                        )+)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::selector::Selector;
    #[test]
    fn selector() {
        let s: Selector<u64, u64> = |i| i * 2;
        assert_eq!(s(&2), 4);
    }
    #[test]
    fn basic_macro() {
        let s1: Selector<u64, u64> = |i| i * 2;
        let l = create_selector!(|i| i * 2, s1);
        assert_eq!(l(&2), 8);
    }
    #[test]
    fn create_selector_multi_selectors() {
        let s1: Selector<u64, u64> = |i| i * 2;
        let s2: Selector<u64, u64> = |i| i * 2;
        let s3: Selector<u64, u64> = |i| i * 2;

        let s4 = create_selector!(|a, b, c| a * b * c, s1, s2, s3);

        assert_eq!(s4(&2), 64);
    }
    #[test]
    fn create_selector_multi_selectors_struct() {
        struct Person {
            age: u8,
            kind: bool,
        }

        let p = Person {
            age: 35,
            kind: true,
        };

        let age_selector: Selector<Person, u8> = |person| person.age;
        let kind_selector: Selector<Person, bool> = |person| person.kind;

        let is_kind_adult_selector =
            create_selector!(|age, kind| age >= 18 && kind, age_selector, kind_selector);

        assert!(is_kind_adult_selector(&p));
    }
}
