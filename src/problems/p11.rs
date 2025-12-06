// Modified Run-Length Encoding
use super::Solution;
use crate::util::{amount::Amount, append_item::Append, rest::Rest, with_last::WithLast};

pub struct P11;

impl P11 {
    fn mod_run_length_enc<T>(list: &Vec<T>) -> Vec<Amount<T>>
    where
        T: Clone + PartialEq,
    {
        Self::mrle_rec(list, &vec![])
    }

    fn mrle_rec<T>(curr: &Vec<T>, next: &Vec<Amount<T>>) -> Vec<Amount<T>>
    where
        T: Clone + PartialEq,
    {
        match (curr.first(), next.last()) {
            (None, _) => next.clone(),
            (Some(x), Some(amount)) => match amount {
                Amount::One(y) if x == y => Self::mrle_rec(
                    &curr.rest(),
                    &next.with_last(|_| Amount::Many(2, y.clone())),
                ),
                Amount::Many(i, y) if x == y => Self::mrle_rec(
                    &curr.rest(),
                    &next.with_last(|_| Amount::Many(i + 1, y.clone())),
                ),
                _ => Self::mrle_rec(&curr.rest(), &next.append_item(Amount::One(x.clone()))),
            },
            (Some(x), _) => Self::mrle_rec(&curr.rest(), &next.append_item(Amount::One(x.clone()))),
        }
    }
}

impl Solution for P11 {
    type Input = Vec<char>;
    type Output = Vec<Amount<char>>;

    fn test_name() -> String {
        String::from("Modified Run Length Encoding")
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
        use Amount::*;
        [
            vec![
                Many(4, 'a'),
                One('b'),
                Many(2, 'c'),
                Many(2, 'a'),
                One('d'),
                Many(4, 'e'),
            ],
            vec![],
            vec![One('a'), One('b'), One('c')],
        ]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P11::mod_run_length_enc(input)
    }
}
