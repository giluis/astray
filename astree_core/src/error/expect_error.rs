use crate::base_traits::Expectable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectError<T> 
where T: Expectable<T> {
    NoMoreTokens{failed_at: usize, could_not_consume: T},
    ExpectedNotFound{failed_at: usize, expected: T, found: T},
}
