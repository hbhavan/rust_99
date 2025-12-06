#[derive(Clone, PartialEq)]
pub enum Amount<T> {
    One(T),
    Many(usize, T),
}
