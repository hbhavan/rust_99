// Write a function `last<T>(list: &[T]) -> Option<&T>' that returns the last element of a list

use super::Solution;

pub struct P1;

impl P1 {
    pub fn last<T>(list: &[T]) -> Option<&T> {
        Self::last_rec(list)
    }

    fn last_rec<T>(list: &[T]) -> Option<&T> {
        match list {
            [] => None,
            [x] => Some(x),
            [_, rest @ ..] => Self::last_rec(rest),
        }
    }
}

impl Solution for P1 {
    type Input = Vec<char>;
    type Output = Option<char>;

    fn test_name() -> String {
        String::from("Tail of a List")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [vec!['a', 'b', 'c', 'd'], vec!['a'], vec![]]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [Some('d'), Some('a'), None]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P1::last(input).map(|c| *c)
    }
}
