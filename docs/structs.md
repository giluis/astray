# Basic types

Almost any type can derive the `SN` macro. This auto implements the `Parsable<T>` trait, which comes with the  `parse` associated function:

```rust
trait Parsable<T> {
    fn parse(token_iter: TokenIterator<T>) -> Result<Self, ParseError<T>>;
    /// other stuff we'll discuss later*
}
```

A TokenIterator is an abstraction over a Vec of Tokens that can go iterate bidirectionally and do a bunch of useful stuff. We'll cover it [later](./token_iterator.md).


Before doing anything with Astray, the user must call the `set_token!(<your_token_type>)` macro to let Astray know what type will be considered a token for the parsing functions it will generate.

## structs and pattern matching
When a `struct` derives `SN`, Parsable<T> is auto implemented. You can call the parse function like this:

```rust
set_token!(Token)

#[derive(SN)]
struct Pair{
    l_element: Token,
    comma: Token,
    r_element: Token,
}

fn main() {
    // let tokens = lexer("a b c"); This will be parsed successfully as well
    let tokens = lexer("a , c");
    assert_eq!(tokens, vec![
        Token::Identifier("a".to_owned())
        Token::Comma
        Token::Identifier("c".to_owned())
    ])
    let pair: Result<Pair, ParseError<Token>> = Pair::parse(tokens.into())
}
```
Struct fields will be parsed top to bottom. *If any of the fields cannot be parsed, the struct will fail parsing as well.*

The code above will actually parse any set of 3 tokens, since we have not specified which tokens should be consumed. We can do so with pattern matching, which we'll see next.

I'm working on support for Tuple Structs:
TODO: Insert issue number here []()

```rust
set_token!(Token);

#[derive(SN)]
struct TwoNumbers(Token, Token);

fn main() {
    let tokens = lexer("1 2");
    assert_eq!(tokens, vec![
        Token::IntLiteral(1)
        Token::Identifier(2)
    ]);

    let two_numbers: Result<TwoNumbers, ParseError<Token>> = TwoNumbers::parse(tokens.into())
}
```
## pattern matching



