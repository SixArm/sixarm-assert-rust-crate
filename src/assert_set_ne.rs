/// Asserts that two sets are not equal to each other.
///
/// This implementation uses [`HashSet`] and [`assert_eq!`].
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assert_ne!`], this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate sixarm_assert; fn main() {
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6];
/// assert_set_ne!(a, b);
///
/// assert_set_ne!(a, b, "we are testing with {:#?} and {:#?}", a, b);
/// # }
/// ```
#[macro_export]
macro_rules! assert_set_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                assert_ne!(left_set, right_set);
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                assert_ne!(left_set, right_set, $($arg)+);
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_set_ne_with_arity_2_return_success() {
        assert_set_ne!(vec![1, 2], vec![3, 4]);
    } 

    #[test]
    fn test_assert_set_ne_with_arity_3_return_success() {
        assert_set_ne!(vec![1, 2], vec![3, 4], "message");
    } 

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_set_ne_with_arity_2_return_failure() {
        assert_set_ne!(vec![1, 2], vec![1, 2]);
    } 

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_set_ne_with_arity_3_return_failure() {
        assert_set_ne!(vec![1, 2], vec![1, 2], "message");
    } 

}
