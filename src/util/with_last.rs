pub trait WithLast<T> {
    fn with_last<F>(&self, last: F) -> Self
    where
        F: FnOnce(&T) -> T;
}

impl<T> WithLast<T> for Vec<T>
where
    T: Clone,
{
    fn with_last<F>(&self, last: F) -> Self
    where
        F: FnOnce(&T) -> T,
    {
        let l = self.len();
        match self.last() {
            Some(x) => [&self[..l - 1], &[last(x)]].concat(),
            None => vec![],
        }
    }
}
