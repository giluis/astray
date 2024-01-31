# Define a set of universal rules for Astray
Astray is complex and must follow some rulessarily These are being defined (WIP) at (./universal_rules.md)

# Implement Iterator<T> for TokenIterator<T>
- Allows usage of iterator methods
- replaces `fn consume(&mut self)` with `fn next(&mut self)`, which is more intuitive in Rust land
- Prevents iterator from going backwards, of course. So it would not work on sum types.

# Functional macro for enum optimization
When an enum implements Parsable<T>, its branches can have common parsing paths!

```rust
enum MyEnum {
    Variant1(Struct1),
    Variant2(Struct2),
}

struct Struct1 {
    #[pat(Token::Plus)]
    plus: Token
    #[pat(Token::LiteralInt(_))]
    plus: TOken
}

struct Struct2 {
    #[pat(Token::Plus)]
    plus: Token
    #[pat(Token::LiteralString(_))]
    plus: TOken
}
```

Given a `TokenIterator` over `[Token:Plus, TokenLiteralString("something")]`, Astray tries to parse a Plus (for a Struct1), succeeds and tries to parse a LiteralInt(_), fails, tries to parse a Plus again (for Struct2) and then a LiteralString (succeeds).
Plus has been tried twice unnecessarilly.
The goal is to optimize this. I already have an idea for execution, which I will expose later as soon as I can.

# Parsing visualization for the command line (bonus points if also works in the browser)
An animation of how parsing is happening in "real" time, showing how the parser works.

# Documenting all functions in the code
Although they are (hopefully) rather self explanatory, great Rusty documentation is missing from each function.
I hope to add it in the future.

#  Updating nomenclature
Consumable types -> Parsable types
Partially in docs, WIP in the code.

# Generic Errors instead of ParseError
Turning ParseError into a trait allows users to specify what Error type their parsing functions to produce.


# Implement FromIterator for TokenIterator
Makes it easier to create a TokenIterator, rather.


# Update parse_if_match function
Parsers should be generic not only on type's they take, but also on their arity!
I'm investigating possible solutions for this


# Add more advanced validation functions

```rust
struct A {
    /* These should get combined into a single validation function? Or maybe multiple validation functinons... more experimentation is required*/
    #[pat(Token::LiteralInt(_))]
    #[len(> 5)]
    field1: Vec<Token>, 
}
```

Perhaps renaming `parse_if_match` to `parse_if_valid`

# Remove UnexpectedToken ParseErrorType variant
Since Tokens are to be treated as parsable types, under the specifc restrictions that Token: Parsable<Token>, then Conjunct / Disjunct branch failure error variant should be used, depending on whether the token is an enum or a struct

# Rename ParseErrorType to ParseErrorReason


