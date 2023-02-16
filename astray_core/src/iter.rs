use crate::{base_traits::{Expectable, Parsable}, error::{expect_error::ExpectError, parse_error::ParseError}};

pub struct TokenIter<'tokens,  T>
where
    T: Expectable<T>,
{
    pub current: usize,
    pub tokens: &'tokens [T],
    size: usize,
    pub stack: Vec<usize>,
}

impl<'tokens, T> TokenIter<'tokens, T>
where
    T: Expectable<T>,
{
    pub fn new(tokens: &'tokens [T]) -> TokenIter<'tokens, T> {
        TokenIter {
            current: 0,
            size: tokens.len(),
            tokens,
            stack: vec![],
        }
    }

    pub fn parse<P>(&'tokens mut self) -> Result<P, ParseError<T>>
    where
        P: Parsable<T>, 

    {
        self.push();
        let result = P::parse(self);
        self.clean_pop();
        // if result.is_ok() {
        //     self.clean_pop();
        // } else {
        //     self.pop();
        // }
        result
    }

    pub fn try_do<'a, F, Q, E>(&'tokens mut self, f: F) -> Result<Q, E>
    where
        F: FnOnce(& mut TokenIter<'tokens, T>) -> Result<Q, E>,
    {
        self.push();
        let result = f(self);
        if result.is_ok() {
            self.clean_pop();
        } else {
            self.pop();
        }
        result
    }


    pub fn expect<E>(&'tokens mut self, token: T) -> Result<E, ExpectError<T>> 
    where E: Expectable<T>{
        self.try_do(|token_iter| {
            E::expect(token_iter, token)
        })
    }

    pub fn push(&mut self) {
        self.stack.push(self.current);
    }

    pub fn clean_pop(&mut self) {
        self.stack.pop();
    }

    pub fn pop(& mut self) -> Option<usize> {
        match self.stack.pop() {
            Some(c) => {
                self.current = c;
                Some(c)
            }
            None => None,
        }
    }

    pub fn consume(&mut self) -> Option<T> {
        match self.get(self.current) {
            Some(element) => {self.current += 1; Some(element)},
            None => None
        }
    }

    pub fn get(&self, position: usize) -> Option<T> {
        if position < self.size {
            Some(self.tokens[position].clone())
        } else {
            None
        }
    }
}

impl <'a, T> From<&'a [T]> for TokenIter<'a, T>
where T: Expectable<T> {
    fn from(tokens: &'a [T]) -> Self {
        TokenIter::new(tokens)
    }
}

impl <'a, T> From<&'a Vec<T>> for TokenIter<'a, T>
where T: Expectable<T> {
    fn from(tokens: &'a Vec<T>) -> Self {
        TokenIter::new(tokens.as_slice())
    }
}
