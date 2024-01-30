# A primer on compilers

Let's imagine a programming language called PseudoRust where the only valid statement is:

```rust
let <i> = <x> <sign> <y>;
```
Where:
```rust
<i>    :=  [a-z]([1-9] | [a-z])*
<x>    :=  [0-9]
<y>    :=  [0-9]
<sign> :=  + | *
```

If confusing, see [here](https://medium.com/factory-mind/regex-tutorial-a-simple-cheatsheet-by-examples-649dc1c3f285)

Our goal is to write a compiler in Rust that takes PseudoRust text and turns it into machine code for a computer to run.
A compiler can be divided into (at least) 3 steps:

1. Lexing / Tokenization
2. Parsing
3. Code Generation

Real world compilers include other steps and features like type-checking and optimizations.  

## 1. Lexing / Tokenization

Tokens (a.k.a lexems) are the smallest meaningful units in a programming language.

E.g. in PseudoRust, the following would be tokens:

- `Let`: let keyword
- `+`: plus sign
- `123`: an integer literal
- `abc`: an identifier 
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

/* Example of storing additional data
struct TokenStruct {
    index_in_source: usize,
    token_len: usize,
    token_type: Token
}*/


fn lex(text: &str) -> Vec<Token> {
    /* Loop through the text, find tokens. Record additional data if needed  */
}

```

## 2. Parsing

Lexing gives us a list of meaningful building blocks. Out compiler should now check that these building blocks are arranged in accordance with the language's syntax.
A way to do this is by parsing the Tokens into an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree), which asserts meaningful logical relationships between tokens according to syntax rules.

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
... and, given the following AST definition:
```rust
    struct AST {
        // Token::LetKw
        let_kw: Token
        var_ident: String,
        // Token::EqualSign
        equals_sign: Token,
        body: Expr,
        // Token::SemiColon
        semicolon: Token,
    }


    struct Expr {
        // Token::LiteralInt(_)
        left: Token
        sign: Sign,
        // Token::LiteralInt(_)
        right: u32
    }

    // Token::Plus | Token::Asterisk
    enum Sign {
        Add
        Mult
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
        let_kw: Token::LetKw,
        var_ident: "a".to_owned(),
        equals_sign: Token::EqualsSign,
        body: Expr {
            left: Token::LiteralInt(1),
            sign: Sign::Add,
            right: Token::LiteralInt(3),
        },
        semicolon: Token::Semicolon,
    }
```
This AST let's us know that this item is an assignment, using the `let` keyword of the result of `(1 + 3)` to the identifier "a". 

Note 1:
    Our PseudoRust syntax is quite simple. For more complex syntaxes, some new challenges start to arise.
    I recommend Nora Sandler's excellent guide on [building a compiler](https://norasandler.com/2017/11/29/Write-a-Compiler.html), so you can understand these challenges.
Note 2: 
    Some of these fields could perhaps be dropped from the AST. 
    As an example, the equals sign token doesn't have any use here, since we already typed this statement as being an Assignment.

Note 3:
    Sometimes, storing the whole token might not be necessary, and maybe we'll just include type it contains in the AST, like we see in `var_ident` field `Assignment`. 

## 2.5 Error Handling
In practice, any step of our compiler might fail:
When Lexing, maybe some unrecognized tokens are present in the source text:
    `let a = 1 👍 3;`
    According to our grammar, this statement is un-lexable and so lexing should fail

Even if lexing succeeds, maybe parsing will fail if there are no syntax rules to explain the tokens that were produced by the lexer:
`let a let a = 1 + 3;`  
Though all tokens were valid `let a let a` has no meaning according to our syntax, so parsing should fail.

Code generation from an AST is more straightforward than the previous steps and would, in this case, maybe only fail if there was some compatibility issue with the target architecture, or something like that.

So, in practice, all our steps should produce `Result`s rather than just values.

## 3. Code Generation

### 3.1 Generating Assembly

Our computers really only care about machine code, a binary language that represents instructions for our CPU to execute.
Machine code is rather unsavory for our simple human minds, so, instead, we'll think about a human readable version of machine code: **Assembly**.
Turning an AST into Assembly is off the scope of this project and repository, but feel free to check Nora Sandler's [guide](https://norasandler.com/2017/11/29/Write-a-Compiler.html).


In the end, our compiler would look something like this:

```rust
fn compiler(text: &str) -> Result<String,CompileTimeError> {
    let tokens: Vec<Token> = lex(text)?;
    let ast: AST  = parse(tokens)?;
    generate_assembly(ast)
}

fn generate_assembly(ast: AST) -> Result<String, ParseError > {
    //...
}
```

### 3.2 Assembling Assembly into Machine Code

Assembling is the process of turning Assembly into machine code. It's a _relativelly_ straightforward process, where each Assembly instruction is turned into 1 or more machine code instrutions. 
This process is very well studied, highly optimized and, once again, off the scope of this project.
Important note: very often, compilers will transform an AST directly into machine code, skipping 3.1 entirelly. This makes sense, since likely no one will look at whatever the output of this phase is, hence no need for human readable output.




