// Write a function `reverse<T>(list: Vec<T>) -> Vec<T>' that returns the of elements of a list
// reversed
use super::Solution;
use crate::util::prepend_item::Prepend;

pub struct P05;

impl P05 {
    fn reverse<T>(list: &Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        Self::reverse_rec(list, &vec![])
    }

    fn reverse_rec<T>(curr: &Vec<T>, next: &Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        match curr.as_slice() {
            [] => next.clone(),
            [x, rest @ ..] => Self::reverse_rec(&rest.to_vec(), &next.prepend_item(x.clone())),
        }
    }
}

impl Solution for P05 {
    type Input = Vec<char>;
    type Output = Vec<char>;

    fn test_name() -> String {
        String::from("Reverse List")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [vec!['a', 'b', 'c'], vec![], vec!['a']]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [vec!['c', 'b', 'a'], vec![], vec!['a']]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P05::reverse(&input)
    }
}
