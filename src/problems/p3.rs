// Write a function `nth<T>(list: &[T]) -> Option<&T>' that returns the nth element of a list

use super::Solution;

pub struct P3;

impl P3 {
    pub fn nth<T>(n: usize, list: &[T]) -> Option<&T> {
        Self::nth_rec(n, list)
    }

    fn nth_rec<T>(n: usize, list: &[T]) -> Option<&T> {
        match (n, list) {
            (_, []) => None,
            (0, [x, ..]) => Some(x),
            (_, [_, rest @ ..]) => Self::nth_rec(n - 1, rest),
        }
    }
}

impl Solution for P3 {
    type Input = (usize, Vec<char>);
    type Output = Option<char>;

    fn test_name() -> String {
        String::from("Nth Element")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [
            (2, vec!['a', 'b', 'c', 'd', 'e']),
            (1, vec![]),
            (2, vec!['a']),
        ]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [Some('c'), None, None]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P3::nth(input.0, &input.1).map(|c| *c)
    }
}
