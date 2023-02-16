use astray::AstNode;
use parser::iter::TokenIter;
use parser::parse::Parsable;
use token::{t, Token};

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub struct ReturnStatement {

    #[stateless_leaf(Token::KReturn)]
    k_return: Option<Token>,

    #[stateful_leaf(Token::Identifier)]
    ident: String,

    #[stateless_leaf(Token::SemiColon)]
    semi: Option<Token>,
}





fn main() {
    let tokens = &[
        t!(return),
        t!(ident "some_ident"),
        t!(;)
    ];
    let expected = ReturnStatement{
        k_return: Some(t!(return)),
        ident: "some_ident".to_string(),
        semi: Some(t!(;))
    };

    let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    assert_eq!(Ok(expected), result);
}
