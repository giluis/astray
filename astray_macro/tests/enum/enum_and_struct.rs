use astray::AstNode;
use parser::iter::TokenIter;
use hatch_result::ResultHatchExt;
use parser::parse::Parsable;
use token::{t, Token};

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub struct FnCall {
    #[stateless_leaf(Token::Identifier)]
    ident: Token,
    args: Args,
}

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub enum Args {
    EmptyArgs(EmptyArgs),
    FullArgs(FullArgs),
}

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub struct EmptyArgs {
    #[stateless_leaf(Token::LParen)]
    l_paren: Token,

    #[stateless_leaf(Token::RParen)]
    r_paren: Token,
}

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub struct FullArgs {
    #[stateless_leaf(Token::LParen)]
    l_paren: Token,

    ty1: ArgType,

    #[stateful_leaf(Token::Identifier)]
    ident1: String,

    #[stateless_leaf(Token::Comma)]
    comma: Token,

    ty2: ArgType,

    #[stateful_leaf(Token::Identifier)]
    ident2: String,

    #[stateless_leaf(Token::RParen)]
    r_paren: Token,
}

#[derive(AstNode, PartialEq, Debug)]
#[token(Token)]
pub enum ArgType {
    #[stateless_leaf(Token::KFloat)]
    KFloat(Token),

    #[stateless_leaf(Token::KInt)]
    KInt(Token),
}


fn main() {
    test_empty_args();
    test_full_args();
}

fn test_full_args() {
    let expected_fnident1 = "fn1";
    let expected_arg_ident1 = "fn1";
    let expected_arg_ident2 = "fn1";
    let tokens = &[
        t!(ident expected_fnident1), 
        t!(l_paren), 
        t!(float),
        t!(ident expected_arg_ident1),
        t!(,),
        t!(int),
        t!(ident expected_arg_ident2),
        t!(r_paren)];
    let mut token_iter = TokenIter::new(tokens);
    let empty_result = FnCall::parse(&mut token_iter);
    let expected_result = FnCall {
        ident: expected_fnident1.to_string(),
        args: Args::FullArgs(FullArgs {
            l_paren: t!(l_paren),
            ty1: ArgType::KFloat(Token::KFloat),
            ident1: expected_arg_ident1.to_string(),
            comma: t!(,),
            ty2: ArgType::KInt(Token::KInt),
            ident2:expected_arg_ident2.to_string(),
            r_paren: t!(r_paren),
        }),
    };
    match empty_result {
        Ok(result) => assert!(result == expected_result),
        Err(msg) => panic!("Expected Ok, but got error {msg}"),
    }
}

fn test_empty_args() {
    let expected_fn_ident = "fn1";
    let tokens = &[t!(ident expected_fn_ident), t!(l_paren), t!(r_paren)];
    let mut token_iter = TokenIter::new(tokens);
    let result = FnCall::parse(&mut token_iter);
    let expected = FnCall {
        ident: expected_fn_ident.to_string(),
        args: Args::EmptyArgs(EmptyArgs {
            l_paren: t!(l_paren),
            r_paren: t!(r_paren),
        }),
    };
    match result {
        Ok(inner_result) => assert_eq!(inner_result, expected),
        Err(msg) => panic!("Expected Ok, but got error {msg}"),
    }
}