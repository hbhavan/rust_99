pub trait Prepend<T> {
    fn prepend_item(&self, item: T) -> Self;
}

impl<T> Prepend<T> for Vec<T>
where
    T: Clone,
{
    fn prepend_item(&self, item: T) -> Self {
        [&[item], &self[..]].concat()
    }
}
