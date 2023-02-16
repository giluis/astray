use crate::{iter::TokenIter, parse::{Parsable, Expectable}, error::parse_error::ParseError};
use token::{t, Token};

#[derive(PartialEq, Debug)]
struct TestStruct {
    var_type: Token,
    var_name: String,
    equals_sign: Token,
    value: u32,
}

impl Parsable<Token> for TestStruct {
    fn parse<'a>(iter: &mut TokenIter<Token>) -> Result<TestStruct, ParseError<Token>> {
        let var_type = Token::expect(iter, t!(int))?;

        let var_name = match Token::expect(iter,t!(ident))? {
            Token::Identifier(ident_str) => ident_str,
            _ => panic!("Internal error, should be ident_str"),
        };

        let equals_sign = Token::expect(iter,t!( = )).unwrap();
        let value = match Token::expect(iter,t!(litint)).unwrap() {
            Token::LiteralInt(value) => value,
            _ => panic!("Internal error: should be lit int"),
        };
        Ok(TestStruct {
            var_type,
            var_name,
            equals_sign,
            value,
        })
    }
}


//TODO: check error is the correct one
#[test]
fn failed_expect() {
    let tokens = vec![t!(litint 32)];
    let mut iter = TokenIter::from(tokens.as_slice());
    let result = Token::expect(&mut iter,t!(return));
    assert!(result.is_err());
    assert!(iter.current == 0);
}

#[test]
fn failed_parse() {
    let tokens = vec![
        t!(int),
        // variable_name is missing, so TestStruct will not be parsed
        t!( = ),
        t!(litint 3),
    ];

    let mut iter = TokenIter::from(tokens.as_slice());
    let result = TestStruct::parse(&mut iter);
    assert!(result.is_err());

    // current should be zero, since struct was not parsed
    assert!(iter.current == 0)
}

#[test]
fn successful_parse() {
    let expected_var_name = "variable1";
    let expected_value = 3;
    let tokens = vec![
        t!(int),
        t!(ident expected_var_name),
        t!( = ),
        t!(litint expected_value),
        Token::LiteralInt(expected_value),
        t!( ; ),
    ];
    let mut iter = TokenIter::from(tokens.as_slice());
    let expected_struct = TestStruct {
        var_type: t!(int),
        var_name: expected_var_name.to_string(),
        equals_sign: t!( = ),
        value: expected_value,
    };
    let result = TestStruct::parse(&mut iter)
        .expect("Should succeed, since tokens represent a valid TestStruct");
    assert_eq!(result, expected_struct);
}

// #[test]
// fn peek_token() {
//     let mut iter = TokenIter::from(vec![
//         t!(int),
//         Token::Identifier("variable".to_string()),
//         t!( = ),
//         Token::LiteralInt(2),
//         t!( ; ),
//     ]);
//     let rint = iter.peek_token(t!(int));
//     let rident = iter.peek_token(t!(ident));
//     assert!(rint.is_ok());
//     assert!(rident.is_err());
//     assert_eq!(iter.current, 0);
//     iter.increment();

//     let rident = iter.peek_token(t!(ident));
//     assert!(rident.unwrap() == Token::Identifier("variable".to_string()));
//     assert_eq!(iter.current, 1);
// }

#[test]
fn test_push_pop() {
    let tokens = vec![
        t!(int),
        Token::Identifier("variable".to_string()),
        t!( = ),
        Token::LiteralInt(2),
        t!( ; ),
    ];
    let mut iter = TokenIter::from(tokens.as_slice());
    iter.push();
    assert_eq!(iter.stack, vec![0]);
    let r =Token::expect(&mut iter,t!(int)) ;
    assert!(r.is_ok());
    assert_eq!(iter.current, 1);
    iter.pop();
    assert_eq!(iter.current, 0);
}

#[test]
fn test_expect_empty_tokenlist() {
    let tokens = vec![];
    let mut iter = TokenIter::from(tokens.as_slice());

    let result = Token::expect(&mut iter,t!(l_paren));
    assert!(result.is_err());
    assert!(iter.current == 0);
}

#[test]
fn test_expect() {
    let tokens = vec![t!(l_paren), t!(r_paren), t!(,), t!(litint 4)];
    let mut iter = TokenIter::new(tokens.as_slice());

    let lparen_r = Token::expect(&mut iter,t!(l_paren));
    assert!(lparen_r.unwrap() == t!(l_paren)); 

    let rparen_r = Token::expect(&mut iter,t!(r_paren));
    assert!(rparen_r.unwrap() == t!(r_paren));

    let comma_r = Token::expect(&mut iter,t!( , ));
    assert_eq!(comma_r.unwrap(), t!( , ));

    let litint_r = Token::expect(&mut iter,t!(litint));
    assert!(litint_r.unwrap() == t!(litint 4));

    assert!(iter.current == 4)
}

#[test]
fn test_new() {
    let tokens = vec![
        t!(l_paren),
        t!(litint 21),
        t!(,),
        t!(litint 2),
        t!(,),
        t!(litint 21),
        t!(,),
        t!(litint 2),
        t!(,),
        t!(r_paren),
    ];
    let iter = TokenIter::from(tokens.as_slice());
    assert_eq!(iter.current, 0);
    assert_eq!(iter.tokens.len(), 10);
    // assert_eq!(iter.size, 0); // how to test private method?
    assert_eq!(iter.stack.len(), 0);
}

#[test]
fn test_new_empty() {
    let tokens = vec![];
    let iter = TokenIter::<Token>::new(tokens.as_slice());
    assert_eq!(iter.current, 0);
    assert_eq!(iter.tokens.len(), 0);
    // assert_eq!(iter.size, 0); // how to test private method?
    assert_eq!(iter.stack.len(), 0);
}
