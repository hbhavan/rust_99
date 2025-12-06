// Pack consecutive duplicates
use super::Solution;
use crate::util::{append_item::Append, rest::Rest};

pub struct P09;

impl P09 {
    fn pack<T>(list: &Vec<T>) -> Vec<Vec<T>>
    where
        T: Clone + PartialEq,
    {
        Self::pack_rec(list, &mut vec![])
    }

    fn pack_rec<T>(curr: &Vec<T>, next: &mut Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone + PartialEq,
    {
        match (curr.first(), next.last().and_then(|l| l.last())) {
            (None, _) => next.clone(),
            (Some(x), Some(y)) if x == y => {
                Self::append_to_end(next, x.clone());
                Self::pack_rec(&curr.rest(), next)
            }
            (Some(x), _) => Self::pack_rec(&curr.rest(), &mut next.append_item(vec![x.clone()])),
        }
    }

    fn append_to_end<T>(list: &mut Vec<Vec<T>>, item: T) {
        match list.last_mut() {
            Some(x) => x.push(item),
            None => list.push(vec![item]),
        }
    }
}

impl Solution for P09 {
    type Input = Vec<char>;
    type Output = Vec<Vec<char>>;

    fn test_name() -> String {
        String::from("Pack List")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [
            vec![
                'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'd', 'e', 'e', 'e', 'e',
            ],
            vec![],
            vec!['a', 'b', 'c'],
        ]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['b'],
                vec!['c', 'c'],
                vec!['a', 'a'],
                vec!['d', 'd'],
                vec!['e', 'e', 'e', 'e'],
            ],
            vec![],
            vec![vec!['a'], vec!['b'], vec!['c']],
        ]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P09::pack(input)
    }
}
