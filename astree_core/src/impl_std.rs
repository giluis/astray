use crate::{
    iter::TokenIter,
    parse::{Expectable, Parsable}, error::{expect_error::ExpectError, parse_error::ParseError},
};

impl<T, P> Parsable<T> for Vec<P>
where
    T: Expectable<T>,
    P: Parsable<T>,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
        T: Expectable<T>,
    {
        let mut results = vec![];
        while let Ok(r) = P::parse(iter) {
            results.push(r);
        }
        Ok(results)
    }
}

impl<T, P> Parsable<T> for Option<P>
where
    T: Expectable<T>,
    P: Parsable<T>,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
        T: Expectable<T>,
    {
        let r = P::parse(iter);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => unimplemented!("Error types are necessary here"),
        }
    }
}

impl<T> Expectable<T> for Option<T>
where
    T: Expectable<T>,
{
    fn expect(iter: &mut TokenIter<T>, token: T) -> Result<Self, ExpectError<T>>
    where
        Self: Sized,
        T: Expectable<T>,
    {
        let r = T::expect(iter, token);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => Ok(None),
        }
    }
}




#[cfg(test)]
mod tests {
    use token::{Token, t};

    use crate::{parse::{Parsable, Expectable}, iter::TokenIter, error::parse_error::ParseError};


    #[derive(PartialEq)]
    struct TestStruct {
        ident: String,
        semi: Option<Token>
    }

    impl Parsable<Token> for TestStruct 
    {
        fn parse(iter: & mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
    where
        Self: Sized {
            let ident = match Token::expect(iter,Token::Identifier(Default::default()))? {
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by expect should be of the same variant as the token passed as argument"),
            };
            let semi = <Option<Token> as Expectable<Token>>::expect(iter, Token::SemiColon)?;
            Ok(TestStruct {
                ident,
                semi
            })
        }
    }

    #[test]
    fn parse_option_none(){
        let tokens = [
            t!(ident "ident1")
        ];

        let result = TestStruct::parse(&mut TokenIter::new(&tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi.is_none());
    }

    #[test]
    fn parse_option_some(){
        let tokens = [
            t!(ident "ident1"),
            t!(;),
        ];

        let result = TestStruct::parse(&mut TokenIter::new(&tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi == Some(Token::SemiColon));
    }

}