# Astray: Effortless parsing

Automatically generate type safe [Recursive Descent Parsers (RDP)](https://en.wikipedia.org/wiki/Recursive_descent_parser) from Rust structures representing an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

**WARNING**: Astray is not ready for production yet. Some features are missing and the documentation is very incomplete.
I am working hard on these and will deliver on them as soon as possible.

This repository brings together [astray_core](https://github.com/giluis/astray_core) and [astray_macro](https://github.com/giluis/astray_macro) into one crate.


## What is Astray?

Let's think of Step 2 (Parsing) of our compiler building process.
Specifically, how one would implement the parse function:
```rust
fn parse(tokens: &[Token]) -> AST {
    //...
}
```
It seems pretty trivial for our simple syntax, but, as PseudoRust's syntax complexity grows, the task becomes harder and harder.
Imagine having to implement **arithmetic** and **logical** expressions, taking into account all possible precedences. 
Taking into account **function calls**, **context dependent** tokens, type declarations etc.

Luckily, you don't have to have all this work.
Given any set of structs or enums representing an AST, Astray will generate type-safe parsing functions for each of those types.
Note that it allows you to compose types to generate complex ASTs without a hassle

### Features
- Sequence of types, represented as a `struct`
- One of many possible types, represented as an `enum`
- Vec<T>: for consuming multiple types or Tokens
- Option<T>: for consuming a type if it is can be consumed
- Box<T>: for consuming and heap allocating a type
- (T,P): for tuples of types
- Either<T,P>: from the [either](https://crates.io/crates/either) crate
- NonZero<T>: from the [nonempty](https://crates.io/crates/nonempty) crate, allows you to consume at least one type  
- Pattern Matching on Tokens

For a more details, check the [features](./features.md) page


## Quickstart

```bash
cargo add astray
```

Given an AST definition like the below:

```rust
    struct AST {
        items: Vec<Item>
    }

    enum Item {
        Assignment(Assignment),
        Return(ReturnStatement),
    }

    struct ReturnStatement {
        return_kw: Token,
        expr: Expr

    }

    struct Assignment {
        let_kw: Token
        variable: String,
        equals_sign: Token,
        body: Expr,
        semicolon: Token::Semicolon,
    }


    struct Expr {
        left: Token
        sign: Token,
        right: Token
    }
```

1. Call the `set_token!` macro with your custom Token type
2. Annotate each type with the SN (Syntax Node) derive macro
3. Add an attribute representing the pattern you want to match for each token field 

```rust
    set_token!(Token)

    #[derive(SN)]
    struct Assignment {
        #[pat(Token::LetKw)]
        let_kw: Token
        #[pat(Token::Identifier(_))]
        variable: Token,
        #[pat(Token::EqualSign)]
        equals_sign: Token,
        body: Expr,
        #[pat(Token::SemiColon)]
        semicolon: Token::Semicolon,
    }


    #[derive(SN)]
    struct Expr {
        #[pat(Token::IntLiteral(_))]
        left: Token
        #[pat(Token::Plus | Token::Asterisk)]
        sign: Sign,
        #[pat(Token::IntLiteral(_))]
        left: Token
        right: Token
    }

```

... Astray will generate a parse function for that AST:

```rust
fn main() {
    let source_text = "let a = 1 + 2";
    let tokens: Vec<Token> = lex(&source_text)
    let result = Assignment.parse(source_text.into());
    match result {
        Ok(assignment)  => println!("{:?}"),
        Err(parse_error) => ()
    }

}
```
