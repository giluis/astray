# Custom types

Sometimes, the type implementations for Parsable<T> that Astray already provides are inssuficcient for certain usecases. 
It's possible to implement Parsable<T> for your own custom types!
Let's take practical example


# Comma Delimited Array
Let's say we have a TokenIterator<Token> as defined here TODO: add link to syntax definition here.
We want to parse a comma separated array, right-angle bracket delimited array where the last element must not have a comma between it and teh closing bracket. So, no trailling commas are allowed, like in Python 2 lists. 

```python
[1,2,3,4] # should parse
[1,2,3,4,] # should NOT parse
```

Some attempts at defining our AST with Astray that actually will not work: 

### Vec<(Token, Token)>
```rust
/**
 * Does not work because it would not parse the correct version... 
 * [1,2,3,5]
 * ... but would parse this incorrect version:
 * [1,2,3,4,]
 * 
*/
#[derive(SN)]
struct Python2List {
    #[pat(Token::LBracket)]
    l_bracket: Token

    #[pat((Token::IntLiteral(_), Token::Comma))]
    elements: Vec<(Token,Token)>

    #[pat(Token::RBracket)]
    r_bracket: Token

}
```

### Optional comma
```rust
/**
 * Does not work because, while it would allow the correct version...
 * [1,2,3,4]
 * ... it would also allow this incorrect version without commas:
 * [1 2,3 4,]
 * 
*/
#[derive(SN)]
struct Python2List {
    #[pat(Token::LBracket)]
    l_bracket: Token

    elements: Vec<CommaValuePair>

    #[pat(Token::RBracket)]
    r_bracket: Token
}

#[derive(SN)]
struct CommaValuePair {
    #[pat(Token::IntLiteral(_))]
    value: Token,
    #[pat(Token::Comma)]
    comma: Option<Token>,
}

```

### Vec<( element, comma )> and last element
```rust
/**
 * This attempt parses pairs of (element, comma), until a pair cannot be parsed. So, it will parse (element, comma) all the way through until the end, since the last element is the only one that does not have a comma after it. 
 * Then, we parse that last element. 
 * While this approach does work, it's a little clunky to have the last element as a separate field
 * 
*/
#[derive(SN)]
struct Python2List {
    #[pat(Token::LBracket)]
    l_bracket: Token


    #[pat((Token::IntLiteral(_), Token::Comma))]
    elements: Vec<(Token, Token)>

    #[pat(Token::IntLiteral(_))]
    last_element: Token

    #[pat(Token::RBracket)]
    r_bracket: Token
}

```


### Implementing a custom type

DelSep stands for Delimited and Separated.


```rust
/**
 * LDel: Left delimiter
 * RDel: Right delimiter
 * Sep: Separator
 * Elem: Type that lies between each separator, after the left delimiter and before the right delimiter 
 * 
 * [1,2,3,4]
 * 
 * elems_and_seps[i].1 == Some(sep), for  0 < i < elems_and_seps.len() - 1
 * elems_and_seps[elem_and_seps.len() - 1].1 == None 
 *
*/
struct DelSep<T, LDel, RDel, Sep, Elem> 
where 
    T: ConsumableToken
    LDel: Parsable<T>
    RDel: Parsable<T>
    Sep: Parsable<T>
    Elem : Parsable<T>
{
    delimiters: (LDel,RDel),
    elems_and_seps: Vec<(Elem, Option<Sep>)>,
}

impl <T, LDel, RDel, Sep, Elem> Parsable<T> for DelSep<T, LDel, RDel, Sep, Elem> 
where 
    T: ConsumableToken
    LDel: Parsable<T>
    RDel: Parsable<T>
    Sep: Parsable<T>
    Elem : Parsable<T>
{
    
    fn parse(&mut TokenIterator) -> Result<Self, ParseError<T>> {

    }

}

```
struct 

