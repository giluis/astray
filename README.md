# Astray: Effortless parsing

Automatically generate type safe [Recursive Descent Parsers (RDP)](https://en.wikipedia.org/wiki/Recursive_descent_parser) from Rust structures representing an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

This repository brings together [astray_core](https://github.com/giluis/astray_core) and [astray_macro](https://github.com/giluis/astray_macro) into one crate.

**WARNING**: **Astray is currently unmaintained**. I took this decision for two reasons:
- There is too much of an overlap between this crate and others in the field, like serde, unsynn and facet. Unsynn and facet did not exist at the time I started developing this, but they have actually take similar design decisions and gotten much bigger, and so I see as pointelss the effort of duplicating that work
- the architectural choices I've made put in me in an optimization deadend, where ASTs cannot be optimized further.
- In the end, it's too much work for something that I see no future in.


### Features

- Parse a sequence of types, represented via `struct`
- Parse one of many possible types, represented via `enum`
- `Vec<T>`: for consuming multiple types or Tokens
- `Option<T>`: for consuming a type if it is can be consumed
- `Box<T>`: for consuming and heap allocating a type
- `(T,P)`: for tuples of types
- `Either<T,P>`: from the [either](https://crates.io/crates/either) crate
- `NonEmpty<T>`: from the [nonempty](https://crates.io/crates/nonempty) crate, allows you to consume at least one type  
- Pattern Matching on any Parsable type

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
        ident: String,
        equals_sign: Token,
        body: Expr,
        semicolon: Token,
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
    set_token!(Token);

    #[derive(SN)]
    struct AST {
        items: Vec<Item>,
    }

    #[derive(SN)]
    enum Item {
        Assignment(Assignment),
        Return(ReturnStatement),
    }

    #[derive(SN)]
    struct ReturnStatement {
        #[pat(Token::ReturnKw)]
        return_kw: Token,
        expr: Expr,

    }

    #[derive(SN)]
    struct Assignment {
        #[pat(Token::LetKw)]
        let_kw: Token,

        #[extract(Token::Identifier(ident))]
        ident: String,

        #[pat(Token::EqualsSign)]
        equals_sign: Token,

        body: Expr,

        #[pat(Token::SemiColon)]
        semicolon: Token,
    }


    #[derive(SN)]
    struct Expr {
        #[extract(Token::IntLiteral(left))]
        left: u32,

        #[pat(Token::Plus | Token::Asterisk)]
        sign: Sign,

        #[extract(Token::IntLiteral(left))]
        right: u32,
    }

```

... Astray will generate a parse function for that AST:

```rust
fn main() {
    let source_code = "let a = 1 + 2;";
    let tokens: Vec<Token> = lex(&source_code)
    let result = AST::parse(tokens.into());
    match result {
        Ok(ast)  => println!("{:?}"),
        Err(parse_error) => ()
    }
}
```


Check the [docs](https://giluis.github.io/astray/) for more info.

