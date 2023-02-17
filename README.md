# Astray: effortless parsing

Automatically generate type safe [Recursive Descent Parsers (RDP)](https://en.wikipedia.org/wiki/Recursive_descent_parser) from Rust structures representing an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

**WARNING**: Astray is not ready for production yet. Some features are missing and the documentation is very incomplete.
I am working hard on these and will deliver on them as soon as possible.

## Mental Model

An AST is, in essence, a tree that represents hierarchical relationships between concepts.
An AST has a root, which represents the most encompassing structure in a syntax definition. The root is connected to nodes, and the nodes to other nodes, and so on.
The end-nodes that do not link to any nodes but their parents are often called leafs.

- Let's call each node in an AST a Syntax Node (SN)
- SNs can branch to other SNs, which are called their children.
- An SN that does not branch to another SN is called a Leaf
- Leaves will always be reference Tokens.
- Tokens are the smallest units of meaning in a program/language (usually symbols or small sets of symbols).
- With Astray, SNs are represented by a `struct` or an `enum`
  - A Rust `struct` represents a SN that is a composition of its children.
  - A Rust `enum` represents a SN that could in practice branch to any one of its children

Below, an example of an AST defined in these terms. It defines the structure of a computer program:

- A Program contains many Functions. Program is the root of our AST.
- A Function has a return type, no arguments, and a function body, which consists of a many of Statements.
- A return type can either be an `int` keyword or a `float` keyword.
- A Statement can be either a ReturnStatement or something else we might define later.
- A ReturnStatement is a `return` keyword followed by an Expression.
- An Expression is just a literal integer.
- The `int`, `return` and `float` keywords, as well as Identifier are leafs. They do not branch to other SN, but rather contain a Token.
- In the end, we define the Token enum, which represents the smallest parsable unit in our syntax. They are the building blocks of the AST.

```rust
struct Program {
    function: Vec<Function>
}

struct Function {
    return_type: Type,
    identifier: Identifier,
    parens: (LParen, RParen)
    body: Vec<Statement>
}

enum Type {
    Int(KwInt),
    Float(KwFloat),
}

enum Statement {
    ReturnStatement(ReturnStatement),
    // ...
}

struct ReturnStatement {
    kw_return: KwReturn, // keyword return
    expr: Expr,
}

struct Expr {
    expr: LiteralInt
}

// Identifier, KwInt, KwFloat and KwReturn are all Leafs.
// They are the bottom of the item hierarchy
struct Identifier {
    value: Token
}

struct KwReturn{
    value: Token
}

struct KwInt{
    value: Token
}

struct KwFloat{
    value: Token
}

struct LiteralInt {
    value: Token
}


enum Token {
    KwReturn,
    KwInt,
    KwFloat,
    LiteralInt(u32),
    Identifier(String)
}

// ...
```

Now that we have defined the types that represent our AST, we need to build a parser function that takes a list of tokens and correctly assembles the AST.
So, we want to take something like "int func() { return 2;}", use a lexer to build a `Vec<Token>` and then parse that into a Program.
Astray does not deal with the lexing part. You'll have to use an [external crate](https://crates.io/crates/logos) or [build your own](https://mohitkarekar.com/posts/pl/lexer/)

Moving on to the parsing: traditionally, you'd have to build a RDP to parse each struct and enum you have. This includes at least as many functions as SN you have defined (or less functions, but larger). It sounds complex and error-prone: that's because it is.

However, we don't need to go that far thanks to Astray.
By annotating Rust items that represent SNs we can use Astray to automatically generate typesafe parsing functions for each SN!

## How to start

1. Define a `Token` type that represents each building block of your AST.
2. Start by defining a top level struct (root), like we did before, and work your way down.
3. Annotate each SN that is not a leaf with `#[SN(<token>)]`, where `<token>` is the `Token` type you defined. It can have any name.
4. Annotate Leafs with `#[leaf(<token>,<token_instance>)]`, where `token` is the `Token` type you defined in step 1 and `token_instance` is the specific instance of the `Token` type that you are expecting this leaf to contain.
5. Alternatively, just annotate the Token type with `#[derive(leaf)]`. This will automatically generate the leaf SNs for each enum variant.
6. Get an interator of `Token`s and call `::parse(&mut iter)` on the top level type you defined. For the previous example, it would be `Program::parse(&mut iter)`.
7. You'll get a `Result<Program,ParseError<Program>>`. If your tokens matched the specification of AST you gave, you'll have Program struct correctly parsed.
8. Extend the syntax as much as you want, knowing you will never have to build parsing functions for anything.

## Example

For more examples, take a look at the tests folder. In general, tests will be more accurate than any future documentation, since they are checked for errors by the compiler, contrary to markdown files.

There is much more to Astray than this! I'll document it as soon as possible.

```rust
use astray::{SN, Leaf};

#[SN(Token)]
struct Program {
    function: Vec<Function>
}

#[SN(Token)]
struct Function {
    return_type: Type,
    identifier: Identifier,
    parens: (LParen, RParen)
    body: Vec<Statement>
}

#[SN(Token)]
enum Type {
    Int(KwInt),
    Float(KwFloat),
}

#[SN(Token)]
enum Statement {
    ReturnStatement(ReturnStatement),
}

#[SN(Token)]
struct ReturnStatement {
    kw_return: KwReturn, 
    expr: Expr,
}

#[SN(Token)]
struct Expr {
    expr: LiteralInt
}

#[derive(Leaf)] // auto generates leaf SNs for each node in this Enum
enum Token {
    KwReturn,
    KwInt,
    KwFloat,
    EqualSign,
    LeftParen,
    RightParen,
    LiteralInt(u32),
    Identifier(String)
}

fn main(){
    // result of lexing "int function1() return 2"
    let tokens = vec![
        Token::KwInt,
        Token::Identifier("function1".to_string())
        Token::LeftParen
        Token::RightParen
        Token::KwReturn,
        Token::LiteralInt(2),
    ]

    let result = Program::parse(&mut tokens.into_token_iter());
    let expected_program = Program {
        function: vec![
            Function {
                return_type: Type::Int(KwInt{token: Token::KwInt}),
                identifier: Identifier {
                    token: Token::Identifier("function1".to_string())
                },
                parens: (LParen {token: Token::LParen}, RParen {token: Token::RParen})
                body: vec![
                    Statement::ReturnStatement(ReturnStatement{
                        kw_return: KwReturn {token: Token::KwReturn}
                        expr: Expr {
                            expr: LiteralInt {token: Token::LiteralInt(2)}
                        }
                    })
                ]
            }
        ]
    }
    match result {
        Ok(result_program) => assert_eq!(result, expected_program),
        Err(parse_err) => println!("There was a parsing error: {err}"),
    }
}
```
