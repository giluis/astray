use crate::{error::{expect_error::ExpectError, parse_error::ParseError}, iter::TokenIter};
use std::fmt::Debug;
use token::Token;

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

impl Expectable<Token> for Token {
    fn expect(iter: &mut TokenIter<Token>, token: Token) -> Result<Self, ExpectError<Token>> {
        let result = iter.consume().ok_or(ExpectError::NoMoreTokens {
            failed_at: iter.current,
            could_not_consume: token.clone(),
        })?;
        if result.stateless_equals(&token) {
            Ok(result)
        } else {
            Err(ExpectError::ExpectedNotFound {
                failed_at: iter.current,
                expected: token,
                found: result,
            })
        }
    }
}
