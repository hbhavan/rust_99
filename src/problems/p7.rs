// Flatten a nested list
use super::Solution;

pub struct P7;

pub enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

impl P7 {
    fn flatten<T>(list: &Vec<Node<T>>) -> Vec<T>
    where
        T: Clone,
    {
        Self::flatten_rec(list)
    }

    fn flatten_rec<T>(list: &Vec<Node<T>>) -> Vec<T>
    where
        T: Clone,
    {
        list.iter()
            .flat_map(|n| match n {
                Node::One(x) => vec![x.clone()],
                Node::Many(xs) => Self::flatten_rec(xs),
            })
            .collect()
    }
}

impl Solution for P7 {
    type Input = Vec<Node<char>>;
    type Output = Vec<char>;

    fn test_name() -> String {
        String::from("Flatten")
    }

    fn get_inputs() -> [Self::Input; 3] {
        use Node::*;
        [
            vec![
                One('a'),
                Many(vec![One('b'), Many(vec![One('c'), One('d')]), One('e')]),
            ],
            vec![],
            vec![Many(vec![One('a'), One('b'), One('c')])],
        ]
    }

    fn get_outputs() -> [Self::Output; 3] {
        [vec!['a', 'b', 'c', 'd', 'e'], vec![], vec!['a', 'b', 'c']]
    }

    fn assert(expected: Self::Output, result: Self::Output) -> bool {
        expected == result
    }

    fn execute(input: &Self::Input) -> Self::Output {
        P7::flatten(input)
    }
}
