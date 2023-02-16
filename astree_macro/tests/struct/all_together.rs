use astree::AstNode;
use token::{Token, t};
use parser::{iter::TokenIter, parse::Parsable, error::parse_error::ParseError};

#[derive(AstNode, PartialEq)]
#[token(Token)]
pub struct AssignStatement {
    ty: Type,

    #[stateless_leaf( Token::Identifier )]
    ident: Token,

    #[stateless_leaf( Token::Assign )]
    equals_sign: Token,

    #[stateless_leaf( Token::LiteralInt )]
    value: Token
}

// impl Parsable for AssignStatement  {
//    fn parse(iter:&mut Iter) -> Result<AssignStatement, String> {
//      let ident = match iter.get_next() {
//          Some(Token::Identifier(ident)) => ident,
//          Some(other_token) => return Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => return Err(format!("No more tokens",_)),
//      }

//      let ty = iter.parse::<Type>()?;
//
//      let kint = match iter.get_next() {
//          Some(Token::Identifier) => Token::Identifier,
//          Some(other_token) => Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => Err(format!("Expected Token::Identifier, but got {}",_)),
//      }

//      let value = match iter.get_next() {
//          Some(Token::LiterlInt(value)) => value,
//          Some(other_token) => return Err(format!("Expected Token::LiteralInt, but got {}", other_token))
//          _ => return Err(format!("No more tokens",_)),
//      }
//      Ok(AssignStatement {
//          ident,
//          ty,
//          equals_sign,
//          value,
//      })
//    }
// }

#[derive(AstNode, PartialEq)]
#[token(Token)]
pub struct Type {
    #[stateless_leaf(Token::KInt)]
    int: Token,
}

// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      let int = match iter.expect_token(Token::KInt) ? {
//          Token::KInt(int) => int,
//          _ => panic!("Internal error: Ok result for iter.expect should always yield token of the same kind as input "),
//      }
//
//      Ok(Type {
//          int
//      })
//    }
// }
//

fn main() {
    let tokens = &[
            t!( int ),
            t!( ident "var1" ),
            t!( = ),
            t!( litint 1999 ),
    ];
    let mut token_iter = TokenIter::new(tokens);
     let result = AssignStatement::parse(&mut token_iter);

    let expected = AssignStatement::new(
        Type::new(Token::KInt),
        "var1".to_string(),
        Token::Assign,
        1999
    );
    assert!(Ok(expected) == result);
}
