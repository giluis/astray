use astree::{AstNode};
use token::{Token, t};
use parser::{iter::TokenIter, parse::Parsable, error::parse_error::ParseError};


#[derive(AstNode, PartialEq)]
#[token(Token)]
pub struct KInt {
    #[stateless_leaf(Token::KInt)]
    kint: Token,
}

// impl Parsable<Token>  for KInt  {
//    fn parse(iter:&mut TokenIter<Iter>) -> Result<KInt, String> {
//      let kint = iter.expect(Token::KInt)?;
//      Ok(Identifier{kint})
//    }
// }

fn main() {

    let tokens = vec![
        t!( int )
    ];
    let mut iter = TokenIter::new(tokens.as_slice());
    let result = KInt::parse(&mut iter);
    
    assert!(result.unwrap().kint == Token::KInt);

}
