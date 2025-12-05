// Find out whether a list is a palindrome
use super::Solution;

pub struct P06;

impl P06 {
    fn palindrome<T>(list: &[T]) -> bool
    where
        T: PartialEq,
    {
        Self::palindrome_rec(list)
    }

    fn palindrome_rec<T>(list: &[T]) -> bool
    where
        T: PartialEq,
    {
        match list {
            [] => true,
            [_] => true,
            [x, rest @ .., y] if x == y => Self::palindrome_rec(rest),
            _ => false,
        }
    }
}

impl Solution for P06 {
    type Input = Vec<char>;
    type Output = bool;

    fn test_name() -> String {
        String::from("P0alindrome")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [vec!['x', 'a', 'm', 'a', 'x'], vec![], vec!['a', 'b']]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [true, true, false]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P06::palindrome(input)
    }
}
