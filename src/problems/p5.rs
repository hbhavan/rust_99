// Write a function `reverse<T>(list: Vec<T>) -> Vec<T>' that returns the of elements of a list
// reversed
use super::Solution;

pub struct P5;

impl P5 {
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
            [x, rest @ ..] => {
                let rev = vec![vec![x.clone()], next.clone()].concat();

                Self::reverse_rec(&rest.to_vec(), &rev)
            }
        }
    }
}

impl Solution for P5 {
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
        P5::reverse(&input)
    }
}
