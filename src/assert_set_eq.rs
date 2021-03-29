/// Asserts that two sets are equal to each other.
///
/// This implementation uses [`HashSet`] and [`assert_eq!`].
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assert_eq!`], this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate sixarm_assert; fn main() {
/// let a = vec![1, 2, 3];
/// let b = vec![3, 2, 1];
/// assert_set_eq!(a, b);
///
/// assert_set_eq!(a, b, "we are testing with {:#?} and {:#?}", a, b);
/// # }
/// ```
#[macro_export]
macro_rules! assert_set_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                assert_eq!(left_set, right_set);
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                assert_eq!(left_set, right_set, $($arg)+);
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_set_eq_with_arity_2_return_success() {
        assert_set_eq!(vec![1, 2], vec![1, 2]);
    } 

    #[test]
    fn test_assert_set_eq_with_arity_3_return_success() {
        assert_set_eq!(vec![1, 2], vec![1, 2], "message");
    } 

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_set_eq_with_arity_2_return_failure() {
        assert_set_eq!(vec![1, 2], vec![3, 4]);
    } 

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_set_eq_with_arity_3_return_failure() {
        assert_set_eq!(vec![1, 2], vec![3, 4], "message");
    } 

}
