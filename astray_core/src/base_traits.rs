use crate::{error::{expect_error::ExpectError, parse_error::ParseError}, iter::TokenIter};
use std::fmt::Debug;

pub trait Expectable<T>: Debug + PartialEq + Eq + Clone
where
    T: Expectable<T>,
    T: PartialEq,
{
    // TODO: errors
    fn expect(iter: &mut TokenIter<T>, token: T) -> Result<Self, ExpectError<T>>;
}

pub trait Parsable<T>: PartialEq
where
    T: Expectable<T>,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized;
}


