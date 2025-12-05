use crate::util::append_item::Append;

// Eliminate consecutive duplicates (compress) of list elements
use super::Solution;

pub struct P8;

impl P8 {
    fn compress<T>(list: &Vec<T>) -> Vec<T>
    where
        T: Clone + PartialEq,
    {
        Self::compress_rec(list, &vec![])
    }

    fn compress_rec<T>(curr: &Vec<T>, next: &Vec<T>) -> Vec<T>
    where
        T: Clone + PartialEq,
    {
        match (curr.first(), next.last()) {
            (None, _) => next.clone(),
            (Some(x), Some(y)) if x != y => {
                Self::compress_rec(&curr[1..].to_vec(), &next.append_item(x.clone()))
            }
            (Some(x), None) => {
                Self::compress_rec(&curr[1..].to_vec(), &next.append_item(x.clone()))
            }
            (_, _) => Self::compress_rec(&curr[1..].to_vec(), next),
        }
    }
}

impl Solution for P8 {
    type Input = Vec<char>;
    type Output = Vec<char>;

    fn test_name() -> String {
        String::from("Compress List")
    }

    fn get_inputs() -> [Self::Input; 3] {
        [
            vec![
                'a', 'a', 'a', 'a', 'b', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
            ],
            vec![],
            vec!['a', 'b', 'c'],
        ]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [
            vec!['a', 'b', 'c', 'a', 'd', 'e'],
            vec![],
            vec!['a', 'b', 'c'],
        ]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P8::compress(input)
    }
}
