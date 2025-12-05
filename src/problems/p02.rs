// Write a function `last_two<T>(list: &[T]) -> Option<(&T, &T)>' that returns the last two elements of a list
use super::Solution;

pub struct P02;

impl P02 {
    pub fn last_two<T>(list: &[T]) -> Option<(&T, &T)> {
        Self::last_two_rec(list)
    }

    fn last_two_rec<T>(list: &[T]) -> Option<(&T, &T)> {
        match list {
            [] | [_] => None,
            [x, y] => Some((x, y)),
            [_, rest @ ..] => Self::last_two_rec(rest),
        }
    }
}

impl Solution for P02 {
    type Input = Vec<char>;
    type Output = Option<(char, char)>;

    fn test_name() -> String {
        String::from("Last Two Elements")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [vec!['a', 'b', 'c', 'd'], vec!['a'], vec![]]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [Some(('c', 'd')), None, None]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P02::last_two(&input).map(|(c1, c2)| (*c1, *c2))
    }
}
