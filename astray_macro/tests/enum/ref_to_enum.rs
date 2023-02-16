use astray::AstNode;
use parser::iter::TokenIter;
use hatch_result::ResultHatchExt;
use parser::parse::Parsable;
use token::{t, Token};

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub enum Punct {
    #[stateless_leaf(Token::Assign)]
    EqualSign(Token), 
    SemiOrComma(SemiOrComma),
}

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub enum SemiOrComma {
    #[stateless_leaf(Token::Comma)]
    Comma(Token),

    #[stateless_leaf(Token::SemiColon)]
    Semi(Token),
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
    let mut token_iter = TokenIter::new(&[
        t!(=),
        ]);
    let result = Punct::parse(&mut token_iter);
    match result {
        Ok(Punct::EqualSign(Token::Assign)) => (),
        Ok(other) => panic!("Expect Punct::EqualSign variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok(Punct::EqualSign(Token::Assign))(Punct::EqualSign(Token::Assign)) Result, but got error"),
    }

    let mut token_iter = TokenIter::new( &[
        t!(,),
        ] );
    let result = Punct::parse(&mut token_iter);
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Comma(Token::Comma))) => (),
        Ok(other) => panic!("Expected Punct::SemiOrComma(SemiOrComma::Semi(Token::Comma)) variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        Err(msg) => panic!("Expected Ok(Punct::SemiOrComma(SemiOrComma::Comma(Token::Comma))) Result, but got error: {}", msg),
    }
    // assert!(currentBefore + 1 == iter.current );

    let mut token_iter =TokenIter::new(  &[
        t!(;),
        ] );
    let result = Punct::parse(&mut token_iter);
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon))) => (),
        Ok(other) => panic!("Expected Punct Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon)) variant, but got {:?}", other), // internal error: token should always be t! (int) when result is OK
        Err(msg) => panic!("Expected Ok Result, but got error: {:?}", msg),
    }
}
