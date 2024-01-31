# Pattern Matching

You can pattern match on each field in a struct (or enum, as we'll see later) to tell Astray that it should parse a specific instance of a Token (or type) for that specific field.

## No Pattern matching

If pattern matching is not specified, then any instance of that type may be parsed. This includes Tokens:

```rust
set_token!(Token)

#[derive(SN)]
struct Pair{
    l_element: Token,
    comma: Token,
    r_element: Token,
}

fn main() {
    let tokens = lexer("a b c");
    assert_eq!(tokens, vec![
        Token::Identifier("a".to_owned())
        Token::Identifier("b".to_owned())
        Token::Identifier("c".to_owned())
    ])
    // Any three tokens will be successfully parsed. This is pretty useless
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
}
```

Of course, parsing Tokens without a specific

## With pattern matching

If the user wants to parse a specific instance of a type, annotate the desired field with the pattern that field is expected to have. 
```rust
set_token!(Token)

#[derive(SN)]
struct Pair{
    #[pat(Token::Identifier(_))]
    l_element: Token,
    #[pat(Token::Comma)]
    comma: Token,
    #[pat(Token::Identifier(_))]
    r_element: Token,
}

fn main() {
    let tokens = [
        Token::Identifier("a".to_owned()),
        Token::Identifier("b".to_owned()),
        Token::Identifier("c".to_owned()),
    ]
    // result is err
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
    assert!(pair.is_err());

    let tokens = [
        Token::Identifier("a".to_owned()),
        Token::Comma,
        Token::Identifier("c".to_owned()),
    ];
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
    assert_eq!(pair, Ok(Pair {
        l_element: Token::Identifier("a".to_owned()),
        comma: Token::Comma,
        r_element : Token::Identifier("c".to_owned()),
    }))
}
```

Of course this works for all parsable types, not just tokens:


```rust
struct Expr {
    #[pat(Token::IntLiteral(_))]
    left: Token,
    #[pat(Sign::Add)]
    sign: Sign,
    #[pat(Token::IntLiteral(_))]
    right: Token,
}

enum Sign {
    #[pat(Token::Plus)]
    Add,
    #[pat(Token::Minus)]
    Sub,
}


fn main() {
    let tokens = [
        Token::IntLiteral(3),
        Token::Plus,
        Token::IntLiteral(2),
    ]
    let expr_result = Expr::parse(tokens.into());
    assert_eq!(expr_result, Ok(
        Expr {
            left: Token::IntLiteral(3),
            sign: Sign::Add
            right: Token::IntLiteral(2),
        }
    ))

    let tokens = [
        Token::IntLiteral(3),
        Token::Minus,
        Token::IntLiteral(2),
    ]
    let expr_result = Expr::parse(tokens.into());
    // Does not parse, since Expr is expecting specifically a Sign::Add, which may not be parsed when Token::Minus is present instead of Token::Plus
    assert!(expr_result.is_err())
}
```


## Extract values

Currently a WIP, you can extract specific values form a matched pattern, should you want to keep only the inner values of a struct / enum in your AST

```rust
set_token!(Token)

#[derive(SN)]
struct Pair{
    #[extract(Token::Identifier(l_element))]
    l_element: String,
    #[pat(Token::Comma)]
    comma: Token,
    #[extract(Token::Identifier(r_element))]
    r_element: String,
}

fn main() {
    let tokens = [
        Token::Identifier("a".to_owned()),
        Token::Comma,
        Token::Identifier("c".to_owned()),
    ];
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
    assert_eq!(pair, Ok(Pair {
        l_element: "a".to_owned(),
        comma: Token::Comma,
        r_element : "c".to_owned(),
    }))
}
```

## Either this or that

As you'd expect, it's possible to use patterns with pipes to make Astray parse one possible type from a set of types. 
This can be a replacement for moving a type to a separate enum, and will very likely be faster. 
TODO: Benchmark this

```rust
set_token!(Token)

#[derive(SN)]
struct Pair{
    #[extract(Token::Identifier(l_element))]
    l_element: String,
    #[pat(Token::Comma | Token::SemiColon)]
    comma: Token,
    #[extract(Token::Identifier(r_element))]
    r_element: String,
}

fn main() {
    let tokens = [
        Token::Identifier("a".to_owned()),
        Token::Identifier(",".to_owned()),
        Token::Identifier("c".to_owned()),
    ]

    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
    assert_eq!(pair, Ok(Pair {
        l_element: "a".to_owned(),
        comma: Token::Comma,
        r_element : "c".to_owned(),
    }))

    let tokens = [
        Token::Identifier("a".to_owned()),
        Token::Comma,
        Token::Identifier("c".to_owned()),
    ];
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
    assert_eq!(pair, Ok(Pair {
        l_element: "a".to_owned(),
        comma: Token::Comma,
        r_element : "c".to_owned(),
    }))
}
```