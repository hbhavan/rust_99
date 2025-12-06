// Decode run-length encoded list
use super::Solution;
use crate::util::{amount::Amount, append_item::Append, rest::Rest};

pub struct P12;

impl P12 {
    fn decode_run_length_enc<T>(list: &Vec<Amount<T>>) -> Vec<T>
    where
        T: Clone,
    {
        Self::drle_rec(list, &vec![])
    }

    fn drle_rec<T>(curr: &Vec<Amount<T>>, next: &Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        match curr.first() {
            None => next.clone(),
            Some(amount) => match amount {
                Amount::One(x) => Self::drle_rec(&curr.rest(), &next.append_item(x.clone())),
                Amount::Many(i, x) => {
                    let v: Vec<T> = (0..*i).into_iter().map(|_| x.clone()).collect();
                    Self::drle_rec(
                        &curr.rest(),
                        &next.iter().map(|y| y.clone()).chain(v).collect(),
                    )
                }
            },
        }
    }
}

impl Solution for P12 {
    type Input = Vec<Amount<char>>;
    type Output = Vec<char>;

    fn test_name() -> String {
        String::from("Decode Run Length Encoding")
    }

    fn get_inputs() -> [Self::Input; 3] {
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

    fn get_outputs() -> [Self::Output; 3] {
        [
            vec![
                'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
            ],
            vec![],
            vec!['a', 'b', 'c'],
        ]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P12::decode_run_length_enc(input)
    }
}
