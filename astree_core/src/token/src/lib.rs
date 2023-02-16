#[derive(PartialEq, Default, Debug, Clone)]
pub struct LiteralStringValue {
    value: String,
}

impl From<String> for LiteralStringValue {
    fn from(s: String) -> Self {
        LiteralStringValue { value: s }
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct LiteralIntValue {
    value: String,
}

impl From<String> for LiteralIntValue {
    fn from(s: String) -> Self {
        LiteralIntValue { value: s }
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct IdentifierValue {
    value: String,
}

impl From<String> for IdentifierValue {
    fn from(s: String) -> Self {
        IdentifierValue { value: s }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Operations
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    // KeyWords and Literals
    KInt,
    KFloat,
    KReturn,
    // Literals
    LiteralString(String),
    LiteralInt(u32),
    //Identifier
    Identifier(String),
    // Delimeters
    RCurly,
    LCurly,
    RBracket,
    LBracket,
    RParen,
    LParen,
    Comma,
    //Punctuation
    SemiColon,
    // meta
    INVALID,
    EMPTY,
}

impl Token {
    pub fn from_regex_result(&self, input: String) -> (Token, usize) {
        let token = match *self {
            Token::LiteralString(_) => Token::LiteralString(input[1..input.len() - 1].to_string()), // remove  quotes around string
            Token::Identifier(_) => Token::Identifier(input.clone()), // remove  quotes around string
            Token::LiteralInt(_) => {
                let s: u32 = input.parse().unwrap();
                Token::LiteralInt(s)
            }
            _ => (*self).clone(),
        };
        (token, input.len())
    }

    pub fn stateless_equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::LiteralString(_), Token::LiteralString(_)) => true,
            (Token::LiteralInt(_), Token::LiteralInt(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            _ => self == other,
        }
    }
}

#[macro_export]
macro_rules! t {
    (,) => {
        Token::Comma
    };
    (, def) => {
        ","
    };
    (;) => {
        Token::SemiColon
    };
    (; def) => {
        ";"
    };
    (=) => {
        Token::Assign
    };
    (= def) => {
        "="
    };
    (+) => {
        Token::Plus
    };
    (+ def) => {
        "+"
    };
    (-) => {
        Token::Minus
    };
    (- def) => {
        "-"
    };
    (*) => {
        Token::Mult
    };
    (* def) => {
        "*"
    };
    (/) => {
        Token::Div
    };
    (/ def) => {
        "/"
    };
    (int) => {
        Token::KInt
    };
    (float) => {
        Token::KFloat
    };
    (int def) => {
        "int"
    };
    (return) => {
        Token::KReturn
    };
    (return def) => {
        "return"
    };

    (litstr) => {
        Token::LiteralString("DEFAULT_LITERAL_STRING")
    };
    (litint) => {
        Token::LiteralInt(0)
    };
    (ident) => {
        Token::Identifier("DEFAULT_IDENTIFIER".to_string())
    };

    (litstr $value:expr) => {
        Token::LiteralString($value.to_string())
    };
    (litint $value:expr) => {
        Token::LiteralInt($value)
    };
    (ident $value:expr) => {
        Token::Identifier($value.to_string())
    };

    (r_paren) => {
        Token::RParen
    };
    (r_paren def) => {
        ")"
    };
    (l_paren) => {
        Token::LParen
    };
    (l_paren def) => {
        "("
    };
    (r_curly) => {
        Token::RCurly
    };
    (r_curly def) => {
        "}"
    };
    (l_curly) => {
        Token::LCurly
    };
    (l_curly def) => {
        "{"
    };
    (r_bracket) => {
        Token::RBracket
    };
    (r_bracket def) => {
        "]"
    };
    (l_bracket) => {
        Token::LBracket
    };
    (l_bracket def) => {
        "["
    };
    (empty) => {
        Token::EMPTY
    };
    (invalid) => {
        Token::INVALID
    };
}
