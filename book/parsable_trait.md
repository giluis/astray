### The Parsable<T> trait

At the heart of Astray lies the `Parsable<T>` trait. Check its definition [here](https://github.com/giluis/astray_core/main/blob/src/parsable.rs). 
`Parsable<T>` marks a type as consumable type. This means that given a `TokenIterator<T>`, any struct implementing `Parsable<T>` may be parsed from those tokens.

Its definition:

```rust


pub trait Parsable<T>: std::fmt::Debug
where
    T: Parsable<T>,
    Self: Sized,
    T: ConsumableToken

{
    type ApplyMatchTo: Parsable<T> = Self;

    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>;

    fn parse_if_match<F: Fn(&Self::ApplyMatchTo) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized {
            todo!("parse_if_match not yet implemented for {:?}", Self::identifier());
        }
    

    fn identifier() -> &'static str {
        std::any::type_name::<Self>()
    }
}
```

Let's go step by step

### Trait declaration
```rust 
// Any type that implements Parsable<T> must implement std::fmt::Debug
// This is necessary for building nice ParseErrors
pub trait Parsable<T>: std::fmt::Debug
where
    // T: Parsable<T>, meaning T is a Token as per Astray Rule # 1
    T: Parsable<T>,
    // Self is Sized is required, since parse and parse_if_match associated functions return Self
    Self: Sized,
    // This is just a marker trait, that might be removed in the future
    T: ConsumableToken
```

### Associated Type
```rust 
{
    type ApplyMatchTo: Parsable<T> = Self
}
```

This is the type that patterns will be applied to when `#[pat(<pattern>)]` is used.
Generally, it will be Self. However, for [container types](./additional_types.md#option), ApplyMatchTo might be the contained type.
ApplyMatchTo may be any type that makes sense for each specific implementor of Parsable.
Check this page on [implementing Parsable<T> by hand](./custom_types.md) for an example.


### parse function
```rust
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>;
```

Parse takes `&mut TokenIterator<T>`, which must be mut since the inner pointer in TokenIterator will be moved depending on what the parsing function does.
`parse` will always return a Result, meaning it is always fallible.


### parse function

```rust
fn parse_if_match<F: Fn(&Self::ApplyMatchTo) -> bool>(
    _iter: &mut TokenIter<T>,
    _matches: F,
    _pattern: Option<&'static str>
) -> Result<Self, ParseError<T>>
where
    Self: Sized {
        todo!("parse_if_match not yet implemented for {:?}", Self::identifier());
    }
```

The `parse_if_match` function will allow an implementor to restrict which types can be parsed according to a validating function, here named `matches` (TODO: might be renamed in the future). 
Ideally, we would be able to pass a pattern directly to this function, but Rust doesn't really have first class support for patterns, so a `Fn(&Self::ApplyMatchTo) -> bool` does the trick. In practice, the function that actually passed to `parse_if_match` is `|ty|matches!(ty, <pattern>)`.

`parse_if_match` requires a `pattern` string which is a stringified version of a pattern.
Since Rust doesn't really have first class support for patterns, a `matches` which would very useful. So
 
TODO: A default implementation is on the way.


Given a `token_iterator: TokenIterator<T>` and `P: Parsable<T>`:
1. `P` shall called a parsable type 
2. `P` may be parsed from token iterator with `P::parse(&mut token_iterator)`
3. `P::parse(&mut token_iterator)` always produces:
    - `Ok(P {/*fields*/})` if parsing succeeds. The iterator is left at the position pointing to the token after the last token that was consumed for parsing `P`
    - `Err(ParseError<T> /*different errors exist*/)`. In this case, the iterator is reset to the position it was before parsing was attempted
4. `T: Parsable<T>` ([with some caveats](./features.md#notes))
    - Calling `<Token as Parsable<Token>>::parse(&mut token_iterator)` just consumes the next token


