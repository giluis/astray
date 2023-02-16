use crate::parse::Expectable;

use super::expect_error::ExpectError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError<T>
where
    T: Expectable<T>,
{
    UnexpectedToken(ExpectError<T>),
    NoMoreTokens(ExpectError<T>),
    FailedToParseBranch(Box<ParseError<T>>),
}

impl <T> From<ExpectError<T>> for ParseError<T>
where T: Expectable<T> {
    fn from(expect_error: ExpectError<T>) -> Self {
        match expect_error {
            ExpectError::ExpectedNotFound{..} => Self::UnexpectedToken(expect_error),
            ExpectError::NoMoreTokens{..} => Self::NoMoreTokens(expect_error),

        }
    }
}
