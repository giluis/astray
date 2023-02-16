use astray::AstNode;
use hatch_result::ResultHatchExt;
use parser::iter::TokenIter;
use parser::parse::Parsable;
use token::{t, Token};

#[derive(AstNode, PartialEq)]
#[token(Token)]
pub enum TestEnum {
    DoubleComma(DoubleComma),

    #[stateless_leaf(Token::LiteralInt)]
    LitInt(Token),

    #[stateless_leaf(Token::SemiColon)]
    SemiColon(Token),
}

#[derive(AstNode, PartialEq)]
#[token(Token)]
pub struct DoubleComma {
    #[stateless_leaf(Token::Comma)]
    comma1: Token,

    #[stateless_leaf(Token::Comma)]
    comma2: Token,
}

/// impl Parsable for Type  {
///    
/// fn parse(iter:&mut Iter) -> Result<TestEnum, String> {
///           
///    match iter.attempt::<DoubleComma>(){
///         Ok(DoubleComma) => return Ok(TestEnum::DoubleComma(DoubleComma)),
///         Err(_) => (),
///
///    };
///    match iter.peek_token(Token::LitInt(Default::default())) {
///         Ok(Token::LitInt(LitInt)) => {
///             lreturn Ok(TestEnum::LitInt(LitInt))
///         },
///         Err(_) => (),
///    };
///    match iter.peek_token(Token::SemiColon) {
///         Ok(Token::SemiColon) => {
///             lreturn Ok(TestEnum::SemiColon(Token::SemiColon))
///         },
///         Err(_) => (),
///    };
///    return Err("could not parse any of the variants for this sum node".to_string())
/// }
///

fn main() {
    let mut tokens = TokenIter::new(&[t!(,), t!(,)]);
    let result = TestEnum::parse(&mut tokens);
    match result {
        Ok(TestEnum::DoubleComma(DoubleComma {
            comma1: Token::Comma,
            comma2: Token::Comma,
        })) => (),
        Ok(_) => panic!("Expect DoubleComma variant, but didn't get that "), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
    // assert!(currentBefore + 1 == iter.current );
}
