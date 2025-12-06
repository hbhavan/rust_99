// Run-Length Encoding
use super::Solution;
use crate::util::{append_item::Append, with_last::WithLast};
use std::usize;

pub struct P10;

impl P10 {
    fn run_length_enc<T>(list: &Vec<T>) -> Vec<(usize, T)>
    where
        T: Clone + PartialEq,
    {
        Self::rle_rec(list, &vec![])
    }

    fn rle_rec<T>(curr: &Vec<T>, next: &Vec<(usize, T)>) -> Vec<(usize, T)>
    where
        T: Clone + PartialEq,
    {
        match (curr.first(), next.last()) {
            (None, _) => next.clone(),
            (Some(x), Some((_, y))) if x == y => Self::rle_rec(
                &curr[1..].to_vec(),
                &next.with_last(|(l, c)| (l + 1, c.clone())),
            ),
            (Some(x), _) => Self::rle_rec(&curr[1..].to_vec(), &next.append_item((1, x.clone()))),
        }
    }
}

impl Solution for P10 {
    type Input = Vec<char>;
    type Output = Vec<(usize, char)>;

    fn test_name() -> String {
        String::from("Run Length Encoding")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [
            vec![
                'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
            ],
            vec![],
            vec!['a', 'b', 'c'],
        ]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [
            vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')],
            vec![],
            vec![(1, 'a'), (1, 'b'), (1, 'c')],
        ]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P10::run_length_enc(input)
    }
}
