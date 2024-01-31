# Enums

Much like structs, enums can be used as consumable types.
Astray will try to parse each of the enum's variants from top to bottom.
**If all variants fail to parse, the enum fails to parse**
**If at least one variant does parse, the enum parsing succeeds**
All enums derive SN, as long as they follow the guidelines in the notes below:

## Enum Note 1: Unit variants must have a #[pat] attribute annotation

```rust
/*valid*/
#[derive(SN)]
enum Sign {
    #[pat(Token::Plus)]
    Plus,
    #[pat(Token::Minus)]
    Minus,
}

/*invalid, fails to compile*/
#[derive(SN)]
enum Sign {
    Plus,
    Minus,
}
```

## Enum Note 2: Single Tuple variants only

Use single element tuple variants that contain a tuple instead of tuple variants with many elements.

```rust

enum Sign {
    #[pat(Token::Plus)]
    Plus(Token),
    // valid 
    #[pat(Token::Slash)]
    Div((Token)),
    // invalid, fails to compile 
    IntegerDiv(Token, Token),
    // valid, applies pattern to tuple
    #[pat((Token::Slash, Token::Slash))]
    IntegerDiv((Token, Token)),
}
```

**struct variants are not supported *yet***

TODO: Add support for this and mention tracking issue here