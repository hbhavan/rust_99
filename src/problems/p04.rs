// Write a function `length()' that returns the number of elements of a list
use super::Solution;

pub struct P04;

impl P04 {
    pub fn length<T>(list: &[T]) -> usize {
        Self::length_rec(0, list)
    }

    fn length_rec<T>(n: usize, list: &[T]) -> usize {
        match list {
            [] => n,
            [_, rest @ ..] => Self::length_rec(n + 1, rest),
        }
    }
}

impl Solution for P04 {
    type Input = Vec<char>;
    type Output = usize;

    fn test_name() -> String {
        String::from("Length of a List")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [vec!['a', 'b', 'c'], vec![], vec!['a']]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [3, 0, 1]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P04::length(input)
    }
}
