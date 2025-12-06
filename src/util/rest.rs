pub trait Rest<T> {
    fn rest(&self) -> Self;
}

impl<T> Rest<T> for Vec<T>
where
    T: Clone,
{
    fn rest(&self) -> Self {
        self[1..].to_vec()
    }
}
