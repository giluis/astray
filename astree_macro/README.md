# Astree: effortless parsing

Quickly parse a linear structure of tokens into an Abstract Syntax Tree (AST), using only Rust.

**WARNING**: Astree is not ready for production yet. Many features are missing and the documentation is very incomplete. 
I am working hard on these and will deliver on them as soon as possible.

## Mental Model
An AST is in essence a tree that represents hierarchical relationships between concepts. 
Astree provides a framework for describing rules to build ASTs from any source code, using only the Rust language.

Let's call each node in an AST a Syntax Node (SN). SNs can branch to other SNs, called their children (as a metaphor to a family tree).  
Alternatively, SNs can also branch to Leafs, which are SNs that don't branch to anything else   
A Leaf will always be contain a token. Think of a token as the smallest unit of meaning in a program/language.  
SNs are represented by a Rust `struct` or `enum`.    
A Rust `struct` represents a SN that is a composition of its children.   
A Rust `enum` represents a SN that can only in pratice be a parent to only one of its children.   

Below, an example of an AST defined in these terms. In this case, it represents a computer Program, that contains a list of Functions.  
A Function has a return type, no arguments, and a body, which consists of a list of Statements.  
A return type can either be an `int` keyword or a `float` keyword.
A Statement can be either a ReturnStatement or something else we might define later.
A ReturnStatement is a `return` keyword followed by an Expression.
And then we would define Expression, and so on and so on.

At the end you'll find Leafs aka containers for tokens that are used by other SNs.

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


// we could go on and on
struct Expr {
    // ...
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

// ...
```

Now that we have defined the types that represent our AST, we need to build a parser function that takes a list of tokens and correctly assembles the tree.
So, we want to take something like this: "int func() { return 2;}" and parse it into a program.
This might sound daunting and hard: it is.

However, we don't need to go that far thanks to Astree.
By annotating Rust items that represent ASTs we can use Astree to automatically generate typesafe parsing functions for each SN!


## How to start
1. Define a `Token` type that represents each instance of smallest meaning in your progarm. They are usually symbols or small sequences of symbols.
2. Start by defining a top level struct, like we did before, and work your way down.
3. Annotate each SN that is not a leaf with `#[derive(AstNode)]`, and then `#[token(<token>)]`, where `token` is the `Token` type you defined. It can have any name.
4. Annotate Leafs with `#[derive(AstNode)]` and `#[leaf(<token>,<token_instance>)]`, where `token` is the `Token` type you defined in step 1. and `token_instance` is the specific instance of the `Token` type that you are expecting this leaf to contain.
5. Get an interator of `Token`s and call `::parse(<iter>)` on the top level type you defined. For the previous example, it would be `Program`.
6. You'll get a `Result<Program,ParseError<Program>>`. If your tokens matched the specification of AST you gave, you'll have Program struct correctly parsed.
7. Extend the syntax as much as you want, knowing you will never have to build parsing functions for anything. 


## Example
For more examples, take a look at the tests folder. In general, tests will be more accurate than any future documentation, since they are checked for errors by the compiler, contrary to markdown files.
There is much more to Astree than this! I'll document it as soon as possible. 

```rust
fn main(){
    let tokens = vec![
        Token::Identifier("var1".to_string()),
        Token::EqualSign,
        Token::LiteralInt(2),
    ]

    let result = AssignStatement::parse(&mut tokens.into_token_iter());
    match result {
        Ok(assign_statement) => println!("Assign statement was successfully parsed"),
        Ok(parse_err) => println!("There was a parsing error {err}"),
    }
}

enum Token {
    Identifier(String),
    EqualSign,
    LiteralInt(u32),
}

#[derive(AstNode)]
#[token(Token)]
struct AssignStatement {
    ident: Identifier,
    eq: EqualSign,
    literal_int: LiteralInt
}

#[derive(AstNode)]
#[leaf(Token,Token::Identifier)]
struct Identifier{
    value: Token
}

#[derive(AstNode)]
#[leaf(Token,Token::LiteralInt)]
struct LiteralInt{
    value: Token
}

#[derive(AstNode)]
#[leaf(Token,Token::EqualSign)]
struct EqualSign{
    value: Token
}



```

## TODO list
- [ ] More/more thorough examples in the documentation
- [X] Reference to other syntax structures
- [X] Product nodes (structs)
- [X] Sum nodes (enums)
- [ ] Tuple structs
- [ ] Self referencing syntax structures
- [ ] Common use cases (Delimiters, Separators, Either)
- [ ] Implementations of parse for common std types
    - [X] Option
    - [X] Vec
    - [ ] Tuple
    - [ ] Arrays
    - [ ] Box
    - [ ] Rc
    - [ ] Arc
    - [ ] RefCell
 
