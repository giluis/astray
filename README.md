# Astray: Effortless parsing

Automatically generate type safe [Recursive Descent Parsers (RDP)](https://en.wikipedia.org/wiki/Recursive_descent_parser) from Rust structures representing an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

**WARNING**: Astray is not ready for production yet. Some features are missing and the documentation is very incomplete.
I am working hard on these and will deliver on them as soon as possible.

This repository brings together [astray_core](https://github.com/giluis/astray_core) and [astray_macro](https://github.com/giluis/astray_macro) into one crate.

## A primer on compilers

Let's imagine a programming language called PseudoRust where the only valid statement is:

```rust
let <i> = <x> <sign> <y>;
```
Where:
- <i>    :=  [a-z]([1-9] | [a-z])*
- <x>    :=  [0-9];
- <y>    :=  [0-9];
- <sign> :=  + | *;

If confusing, see [here](https://medium.com/factory-mind/regex-tutorial-a-simple-cheatsheet-by-examples-649dc1c3f285)

Our goal is to write a Rust compiler that takes PseudoRust text and turns it into machine code for our computer to run.
A compiler can be divided into (at least) 3 steps:

1. Lexing / Tokenization
2. Parsing
3. Code Generation

Real world compilers include other features like type-checking and optimizations.  

### 1. Lexing / Tokenization

Tokens are the smallest meaningful units in a programming language.

E.g. in PseudoRust, the following would be tokens:

- `Let`: let keyword
- `+`: plus sign
- `123`: integer literal
- `abc`: identifier
- `*`: asterisk sign

Tokens can be easily represented as enums, as seen below.
Other representations might be possible, if you want to store extra information in each token

Lexing means taking as input text representing code as input and transforming it into a list of Tokens.
Take a look at the pseudo-rust found below


For a full tutorial on lexers, check [here](https://mohitkarekar.com/posts/pl/lexer/)
Below, an example of how a lexer for the `PseudoRust` programming language could be typed in Rust: 
```rust

enum Token {
    LetKw,
    Plus,
    Asterisk,
    IntLiteral(u32),
    Identifier(String),
}

/* Example of storing  additional data

struct TokenStruct {
    index_in_source: usize,
    token_len: usize,
    token_type: Token
}

*/

fn lex(text: &str) -> Vec<Token> {
    /* Loop through the text, match characters against all possible tokens. Record additional data if needed  */
}

```

### 2. Parsing

After lexing, the list of Tokens is parsed into an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
An AST is a representation of the logical relations between Tokens.
Let's take a look at how a parse function could work:
E.g The foloowing `PseudoRust` source file:
```rust
    // PseudoRust
    let a = 1 + 3;
```
... could be lexed into these tokens:
```rust
    // the product of our PseudoRust lexer
    vec![ Token::LetKw, Token::Identifier("a"),
        Token::IntLiteral(1), Token::Plus,
        Token::IntLiteral(3)
    ]
```
```rust
fn parse(tokens: &[Token]) -> AST {
    ...
}
```
... and, given the following Rust AST definition:
```rust
    struct AST {
        assignment: Assignment
    }

    struct Assignment {
        let_kw: Token
        variable: String,
        equals_sign: Token,
        body: Expr,
        semicolon: Token::Semicolon,
    }


    struct Expr {
        left: u32
        sign: Token,
        right: Token::LiteralInt(3)
    }
```

... and parse function:
```rust
fn parse(tokens: &[Token]) -> AST {
    // ...
}
```

... the Tokens could be parsed into:
```rust
    AST {
        assignment: Assignment {
            let_kw: Token::LetKw
            variable: Token::Identifier("a"),
            equals_sign: Token::EqualsSign
            body: Expr {
                left: Token::LiteralInt(1)
                sign: Token::Plus,
                right: Token::LiteralInt(3)
            }
            semicolon: Token::Semicolon
        }
    }
```

    Note 1:
        Our PseudoRust syntax is quite simple. For more complex syntaxes, some new challenges start to arise.
        I recommend Nora Sandler's excellent guide on [building a compiler](https://norasandler.com/2017/11/29/Write-a-Compiler.html), so you can fnid these challenges and solve them yourself.
    Note 2: 
        Some of these fields could perhaps be dropped from the AST. 
        As an example, the equals sign token doesn't have any use here, since the types already specify that this is an Assignment.


### 2. Code Generation

Our computers really only care about machine code, a binary language that represents instructions for our CPU to execute.
Machine code is rather unsavory for our puny human minds, so, instead, we'll think about a human readable version of machine code: **Assembly**.
Turning an AST into Assembly is off the scope of this repository, but feel free to check Nora Sandler's [guide](https://norasandler.com/2017/11/29/Write-a-Compiler.html).


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

**Features**
- Sequence of types, represented as a `struct`
- One of many possible types, represented as an `enum`
- Vec<T>: for consuming multiple types or Tokens
- Option<T>: for consuming a type if it is can be consumed
- Box<T>: for consuming and heap allocating a type
- (T,P): for tuples of types
- Either<T,P>: from the [either](https://crates.io/crates/either) crate
- NonZero: from the [nonempty](https://crates.io/crates/nonempty) crate, allows you to consume at least one type  
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
