pub trait Append<T> {
    fn append_item(&self, item: T) -> Self;
}

impl<T> Append<T> for Vec<T>
where
    T: Clone,
{
    fn append_item(&self, item: T) -> Self {
        [&self[..], &[item]].concat()
    }
}
