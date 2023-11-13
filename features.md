# Features

Here we'll cover all the features of Astray
This is a WIP

Astray generates parsing functions from type definitions representing an AST.
Throughout this guide we'll use and expand the syntax for our dummy programming language PseudoRust.

You can find the current syntax [here](./features.md#syntax)

## Basics


### The Parsable<T> trait

At the heart of Astray lies the `Parsable<T>` trait. Check its definition [here](https://github.com/giluis/astray_core/main/blob/src/parsable.rs). 
`Parsable<T>` marks a type as consumable type. This means that given a `TokenIterator<T>`, any struct implementing `Parsable<T>` may be parsed from those tokens.

Its definition:

```rust

pub trait Parsable<T>: std::fmt::Debug
where
    T: Parsable<T>,
    T: ConsumableToken
    Self: Sized,

{

    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>;

    /* The rest of the trait definition has been omitted. It will be handled later*/
}
```

Given a `token_iterator: TokenIterator<T>` and `P: Parsable<T>`:

1. `P` shall called a parsable type 
2. `P` may be parsed from token iterator with `P::parse(&mut token_iterator)`
3. `P::parse(&mut token_iterator)` always produces:
    - `Ok(P {/*fields*/})` if parsing succeeds. The iterator is left at the position pointing to the token after the last token that was consumed for parsing `P`
    - `Err(ParseError<T> /*different errors exist*/)`. In this case, the iterator is reset to the position it was before parsing was attempted
2. `T: Parsable<T>` ([with some caveats](./features.md#notes))
    - Calling `<Token as Parsable<Token>>::parse(&mut token_iterator)` just consumes the next token


### TokenIterator 

A `TokenIterator` is a simple collection that goes through a `Vec<Token>`. It can go forwards or backwards.
Each consumable type has a certain length in terms of the tokens it comprises. This length might be [known only at runtime](./features.md#repeated-types).
As an example, considering PseudoRust syntax:
```rust
let a = 1 + 1;
```
The Assignment type has a length of 7 tokens. It could be more, if, for example, the arithmetic expression on the right side of the equal sign had more elements.

`TokenIterator<T>` has 3 important functions:

#### TokenIterator::try_do

```rust
fn try_do<F, Q, E>(&mut self, f: F) -> Result<Q, E>
    where F: FnOnce(&mut TokenIter<T>) -> Result<Q, E>,
```

try_do executes `f`. `f` is will modify the iterator on execution. 
If `f` returns Ok(Q), try_do returns the same and leaves the iterator be.
If `f` returns Err(E), try_do try_do restores the iterator back to the place it was before executing `f`, and returns that Err

#### TokenIterator::parse

```rust
fn parse<P>(&mut self) -> Result<P,ParseError<T>> 
    where P: Parsable<T> {
        Self::try_do(P::parse)
    }
```

As you can see, `parse` is just a wrapper that passes `<P as Parsable<T>>::parse` to `try_do`.
If parsing succeeds, the iterator will stay at where the `parse` function left it
If not, it will reset the iterator to the place it was before the `parse` function was executed.

Why does `TokenIterator<T>::parse` exist, if it's just passing `<Self as Parsable<T>>::parse` to `TokenIterator<T>::try_do`?
Because it allows very concise and clean auto implementation of <Self as Parsable<T>>::parse, [as we'll see in the next chapter](./features.md#structs-enums-and-automatic-derivation).

## structs, enums and automatic derivation

Deriving `SN`, which stands for "Syntax Node" means that the Parsable trait will automatically be implemented.
Check below for what automatic implementations for structs and enums look like

[Any](./features.md#notes) type that derives
```rust
set_token!(Token)

// This parsable type parses 3 tokens in a row
#[derive(SN)]
struct Tuple {
    left: Token,
    sign: Sign,
    right: Token, 
}

impl Parsable<Token> for Expression {
    fn parse(iter : &mut TokenIterator) -> Result<Expression, ParseError<T>> {
        let left = iter.parse()?;
        let sign = iter.parse()?;
        let right  = iter.parse()?;
        Ok(Expression{left, sign, right})
    }
}

// This parsable type parses 1 token, and then another consumable type
#[derive(SN)]
enum Sign {
    Plus(Token),
    Minus(Token),
}

impl Parsable<Token> for Expression {
    fn parse(iter : &mut TokenIterator) -> Result<Expression, ParseError<T>> {
        let plus = iter.parse().hatch()?;
        let minus = iter.parse().hatch()?;
        Err(ParseError::disjunct())
    }
}
```

Some key points about the scenario above:
1. Parsable types are composable. Notice how parsing `Tuple` requires parsing `Sign`.
2. [hatch_result](https://crates.io/crates/hatch_result) is used to allow concise enum parsing. It returns on Ok values, rather than Err values. An Err value is constructed in the final line of Sign::parse since reaching that line means no enum variant could be parsed.
3. Both Tuple and Sign parse any Token from the iterator. This is, of course, rather useless in a real scenario, where we would want specific Tokens to be parsed. This requires pattern matching, which we will tackle later. Of course, in this scenario, only the Sign::Plus(_) variant would be parsed, since it is attempted first
4. `iter.parse` is a useful wrapper around <P as Parsable<T>>::parse that keeps the state of the iterator if parsing fails. We'll cover it next.

As seen above with `Expr` and `Sign`, consumable types may require parsing other consumable types as well as token.


### TokenIterator::parse_if_match



### Sequential types

The most simple use case for Astray is sequential consumption of tokens or other parsable types.
This can be represented with a struct, where fields will be sequentially consumed from top to bottom.
If a field is expected to be a specific token, annotate with `#[pat(<pattern>)]` for the <patttern> that would match the expected token.

```rust

set_token!(Token)

// will parse `let a = 1`
#[derive(SN)]
struct Assignment {
    declaration: Declaration
    #[pat(Token::EqualSign)]
    equal_sign: Token
    #[pat(Token::IntLiteral(_))]
    lit_int: Token
}

#[derive(SN)]
struct Declaration {
    #[pat(Token::LetKw)]
    let_kw: Token
    #[pat(Token::Identifier(_))]
    identifier: Token
}

// generated by Astray

impl Parsable<Token> for Assignment {
    fn parse(&mut TokenIterator<Token>) -> Result<Assignment,ParseError<T>> {
        let declaration = iter.parse()?;
        // we'll talk later about parse_if_match
        let equal_sign = iter.parse_if_match(|tok|matches!(tok, Token::EqualSign))?;

        let lit_int = iter.parse_if_match(|tok|matches!(tok, Token::EqualSign))?;

        Ok(Assignment {declaration, equal_sign, lit_int} )
    }
}

impl Parsable<Token> for Declaration {
    fn parse(&mut TokenIterator<Token>) -> Result<Declaration,ParseError<T>> {
        // we'll talk later about parse_if_match
        let let_kw = iter.parse_if_match(|tok|matches!(tok, Token::LetKw))?;

        let lit_int = iter.parse_if_match(|tok|matches!(tok, Token::EqualSign))?;

        Ok(Assignment {declaration, equal_sign, lit_int} )
    }
}
```


## Sequential types

## PseudoRust Syntax

```rust
    let a = 1 + 1;
```

## Repeated types

## Notes