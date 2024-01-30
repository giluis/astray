# What is Astray?

In our [compiler primer](./compiler_primer.md), we saw that compilation of a programming language is usually broken down in at least 3 phases:

1. Lexing (text -> Tokens)
2. Parsing (Tokens -> AST)
3. Code generation (AST -> Assembly)
4. Assembling (Assembly -> Machine Code)

Astray highjacks development in step 2 of this list: building a parser.
More specifically, Astray can generate parsing functions for any AST definition, just using a few derive annotations and some attributes.
To get a feel for why this is useful, let'ci's compare a parse function with and without Astray.

## Parser implemented by hand

In more detail, let's think about the previous page' example of a parsing function and try to implement it by hand.

```rust
/**
* Syntax:
* let <i> = <x> <sign> <y>;
* Where:
* - <i>    :=  [a-z]([1-9] | [a-z])*
* - <x>    :=  [0-9];
* - <y>    :=  [0-9];
* - <sign> :=  + | *;

* Check [here](./compiler_primer.md#2-parsing) for AST defintion
*/
fn parse(tokens: &[Token]) -> Result<AST, String> {
    let mut token_ptr = 0;
    /*parse let kw*/
    match  = tokens.get(token_ptr)  {
        Some(Token::LetKw) => (),
        _ =>return Err(format!("Failed to parse 'let' keyword at {token_ptr}"))
    }
    // move on to next token
    token_ptr += 1;

    /*parse variable identifier */
    let var_ident =  match  tokens.get(token_ptr)  {
        Some( Token::Identifier(var_ident)) => var_ident
        _ =>return Err(format!("Failed to parse identifier of variable at {token_ptr}"))
    }
    // move on to next token
    token_ptr += 1;

    /*parse equal sign */
     match tokens.get(token_ptr)  {
Some(Token::EqualSign) => (),
_ => return Err(format!("Failed to parse '=' at {token_ptr}"))
     }
    // move on to next token
    token_ptr += 1;

    /*parse left side of expr*/
    let left =  match tokens.get(token_ptr)  {
        Some(left @ Token::IntLiteral(_)) => left,
        _ => return Err(format!("Failed to parse integer literal at {token_ptr}"))
    }
    // move on to next token
    token_ptr += 1;

    /* parse sign (+ or *) */
    let sign = match tokens.get(token_ptr)  {
        Some(Token::Plus) => Sign::Add
        Some(Token::Asterisk) => Sign::Mult
        _ => return Err(format!("Failed to parse + or * at {token_ptr}"))

    }

    // move on to next token
    token_ptr += 1;

    /*parse right side of expr*/
    let right =  match tokens.get(token_ptr)  {
        Some(right @ Token::IntLiteral(_)) => right,
        _ => return Err(format!("Failed to parse integer literal at {token_ptr}"))
    }
    // move on to next token
    token_ptr += 1;

    /* parse semi colon*/
    match tokens.get(token_ptr)  {
        Some(Token::SemiColon) => (),
        _ => return Err(format!("Failed to parse '=' at {token_ptr}"))
    }
    // move on to next token
    token_ptr += 1;

    // if there any tokens besides these, error
    if token_ptr != tokens.len(){
        return Err("There were too many tokens")
    }

    Ok(AST {
        let_kw: Token::LetKw,
        var_ident,
        equals_sign: Token::EqualSign,
        body: Expr {
            left,
            sign,
            right,
        }
        semicolon: Token::SemiColon
    })

}
```

There are a bunch of obvious problems with this implementation:
1. Precise manipulation of a pointer to tokens might foment logic errors, since it gives us a lot of freedom, especially if we ever need to backtrack  
2. Very repetitive code. We could make it smaller with some abstractions, but it would still be quite repetitive for larger, more complex syntaxes.
3. No mechanism for reusing 

Now, imagine a syntax like [Rust's](https://doc.rust-lang.org/stable/reference/). Building parsing functions for it is a tremendously difficult job that grows quickly.


## Astray to the rescue

Luckily, Astray can help us with parsing functions.

Given any set of structs or enums representing an AST, Astray will generate type-safe parsing functions for each of those types.
It allows you to compose types to generate complex ASTs without a hassle

So, for our previous AST definition, we would have to add some macros:

```rust
    #[derive(SN)]
    struct AST {
        #[pat(Token::LetKw)] 
        let_kw: Token
        #[extract(Token::Identifier(var_ident))] 
        var_ident: String,
        #[pat(Token::EqualSign)] 
        equals_sign: Token,
        body: Expr,
        #[pat(Token::SemiColon)] 
        semicolon: Token,
    }


    #[derive(SN)]
    struct Expr {
        #[pat(Token::LiteralInt(_))]
        left: Token
        sign: Sign,
        #[pat(Token::LiteralInt(_))]
        right: u32
    }

    #[derive(SN)]
    enum Sign {
        #[pat(Token::Plus)]
        Add
        #[pat(Token::Mult)]
        Mult
    }
```

Now, instead of using comments to denote what Tokens are expected, we use `pat(<token pattern>)`. 
We annotate each type with a `#derive[SN]` to let Astray know to implement parsing functions for this particular type.

Now, it's pretty easy to use our parser

```rust
fn parser () {
    let tokens: Vec<Token> = lex("let a = 1 + 1;")
    // `parse` is now an associated function
    let ast: Result<AST, ParseError> = AST::parse(tokens.into());
}

```

**Feature breakdown**

- Parse a sequence of types, represented as a `struct`
- Parse one of many possible types, represented as an `enum`
- Pattern Matching on Tokens
- Vec<T>: for consuming multiple types or Tokens
- Option<T>: for consuming a type if it is there
- Box<T>: for consuming and heap allocating a type
- (T,P): for tuples of types (only arity <=3 implemented for now)
- Either<T,P>: from the [either](https://crates.io/crates/either) crate
- NonEmpty<T>: from the [nonempty](https://crates.io/crates/nonempty) crate, allows you to consume a sequence of at least one type

For more details, keep reading the book!

