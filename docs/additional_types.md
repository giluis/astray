# Additional parsable types

Some useful types implement Parsable<T> and may be used with Astray.

## Vec

`Vec<P>` is a parsable over `TokenIterator<T>` as long as `P: Parsable<T>`
Patterns are applied to each element, rather than the Vec as a whole.
- If no pattern matching is specified, Vec<P> will parse P until the first failure.
Vec<P>::parse(token_iter) always succeeds. If no P can be parsed, then Ok(vec![]) is returned.
- If pattern matching is specified, the pattern is applied to each element, and parsing stops on the first element for which parsing fails, OR for which the pattern doesn't apply.

```rust

struct Array {
    #[pat(Token::LBracket)]
    left_b: Token

    #[pat(Token::IntLiteral(_))]
    elements: Vec<Token>

    #[pat(Token::RBracket)]
    right_b: Token
}

fn main() {
    let tokens = [
        Token::LBracket,
        Token::IntLiteral(2),
        Token::IntLiteral(3),
        Token::IntLiteral(4),
        Token::IntLiteral(5),
        Token::RBracket,
    ]

    let array_result = Array::parse(tokens.into());
    assert_eq!(array_result , Ok(
        Array {
            left_b: Token::LBracket,
            elements: vec![
                Token::IntLiteral(2),
                Token::IntLiteral(3),
                Token::IntLiteral(4),
                Token::IntLiteral(5),
            ],
            right_b : Token::RBracket,
        }
    ))

}
```

## Option
**WARNING** Vec<Option<P>> is unsound and will make consume every Token until. Cases like this (of which this is the only particular instance I know) are very hard to solve without severely complicating the type definitions of [astray_core](https://github.com/giluis/astray_core). Eagerly awaiting for [specialization](https://rust-lang.github.io/rfcs/1210-impl-specialization.html) to be hit nightly, as it would ease the resolution of this issue.

`Option<P>` is a parsable over `TokenIterator<T>` as long as `P: Parsable<T>`
- If no pattern matching is specified, Option<P> will parse P. 
    - If P can be parsed, Ok(Some(P)) is returned
    - If not, None is returned
    - So, Option<P>::parse(token_iter) always succeeds.
- If pattern matching is specified, the pattern is applied the inner type.
    - If type is parsed and pattern applies, Ok (Some(P)) is returned
    - If type is either not parse or pattern does not apply, None is returned 

```rust

/*Parses a declaration with an optional semicolon
Example: 
    let a;
    let b
    
    */
struct Declaration {
    #[pat(Token::LetKw)]
    let_kw: Token

    #[pat(Token::Identifier(_))]
    var_ident: Token

    #[pat(Token::SemiColon)]
    opt_semi: Option<Token>
}

fn main() {
    let tokens = [
        Token::LetKw,
        Token::Identifier("a".to_owned()),
        Token::SemiColon,
    ]

    let declaration_result = Declaration::parse(tokens.into());
    assert_eq!(declaration_result , Ok(
        Declaration {
        let_kw: Token::LetKw,
        var_ident: Token::Identifier("a".to_owned()),
        opt_semi: Some(Token::SemiColon),
        }
    ));

    let tokens = [
        Token::LetKw,
        Token::Identifier("a".to_owned()),
    ]

    let declaration_result = Declaration::parse(tokens.into());
    assert_eq!(declaration_result , Ok(
        Declaration {
            let_kw: Token::LetKw,
            var_ident: Token::Identifier("a".to_owned()),
            opt_semi: None,
        }
    ));

}
```

## Tuples
Tuples of all arities between 1 and 12 are parsable over `TokenIterator<T>`. Thi is true for (T_0, ... T_N)  as long as T_N : Parsable<T>, for all N < 12
Types are parsed starting at the first element (rather predictably) and if any type fails, the tuple fails to parse.
Patterns are applied to the whole tuple. If pattern matching fails, tuple fails to parse. 

```rust

struct CommaSeparatedArray {
    #[pat(Token::LBracket)]
    left_b: Token
    #[pat((Token::IntLiteral(_), Token::Comma))]
    elements: Vec<(Token, Token)>
    #[pat(Token::RBracket)]
    right_b: Token
}

fn main() {
    let tokens = [
        Token::LBracket,
        Token::IntLiteral(2),
        Token::Comma,
        Token::IntLiteral(2),
        Token::Comma,
        Token::IntLiteral(2),
        Token::Comma,
        Token::RBracket,
    ]

    let array_result = Array::parse(tokens.into());
    assert_eq!(array_result , Ok(
        Array {
            left_b: Token::LBracket,
            elements: vec![
                Token::IntLiteral(2),
                Token::IntLiteral(3),
                Token::IntLiteral(4),
                Token::IntLiteral(5),
            ],
            right_b : Token::RBracket,
        }
    ))

}
```

## Box

`Box<P>` is a parsable over `TokenIterator<T>` as long as `P: Parsable<T>`
- If no pattern matching is specified:
    - If P can be parsed, Ok(Box::new(P)) is returned
    - If not, parsing fails 
- If pattern matching is specified, the pattern is applied the inner type:
    - If type is parsed and pattern applies, Ok (Box::new(P)) is returned
    - If type is either not parse or pattern does not apply, parsing fails

this might be removed in the future
TODO: file issue and link it here 

TODO: add nonzero and either