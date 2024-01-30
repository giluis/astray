# Errors

As a general rule,  <P as Parsable<T>>::parse(...) and <P as Parsable<T>>::parse_if_match(...) both produce a Result<P, ParseError<T>>, where T: Parsable<T>. 

This means that all parsing is fallible. If it does fail, a ParseError<T> is produced.
Check its definition below:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType<T>
where
    T: ConsumableToken,
{
    /* Since Tokens are just parsable types, this might be removed in the future*/
    UnexpectedToken { expected: T, found: T },
    /* When you run out of tokens mid parsing a type */
    NoMoreTokens,
    /* When a type can be parsed from the TokenIterator but it does not match the pattern that was applied to it */
    ParsedButUnmatching { err_msg: String }, 
    /**
     * Failed to parse a branch from a conjunct type
     *  This will happen for:
     * - fields /elements in a struct / tuple struct
     * - elements in a tuple
     * - the first element in a NonEmpty vec
     */
    ConjunctBranchParsingFailure { err_source: Box<ParseError<T>> },
    /**
     * Failed to parse a branch from a conjunct type
     *  This will happen for:
     * - variants in an enum
     * - fields in Either
     */
    DisjunctBranchParsingFailure { err_source: Vec<ParseError<T>> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError<T>
where
    T: ConsumableToken,
{
    type_name: &'static str,
    failed_at: usize,
    pub failure_type: ParseErrorType<T>,
}
```

As you can see, a ParseError can have 5 differnent causes.
Check the comments in each for further details.