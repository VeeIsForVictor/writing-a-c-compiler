use regex::Regex;

#[derive(Debug, Clone)]
pub enum CommentToken {
    LineComment,
    BlockComment,
    PendingComment,
}

impl TryFrom<&str> for CommentToken {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use CommentToken::*;
        match value {
            "//" => Ok(LineComment),
            "/*" => Ok(BlockComment),
            _ => Err("coercion from char to SymbolToken failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SymbolToken {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Quote,
    Whitespace,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Percent,
    Decrement,
    Tilde,
}

impl TryFrom<&str> for SymbolToken {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SymbolToken::*;
        match value {
            "(" => Ok(OpenParen),
            ")" => Ok(CloseParen),
            "{" => Ok(OpenBrace),
            "}" => Ok(CloseBrace),
            ";" => Ok(Semicolon),
            "\"" => Ok(Quote),
            "\n" | " " | "\t" => Ok(Whitespace),
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Asterisk),
            "/" => Ok(ForwardSlash),
            "%" => Ok(Percent),
            "--" => Ok(Decrement),
            "~" => Ok(Tilde),
            _ => Err("coercion from char to SymbolToken failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum KeywordToken {
    Int,
    Void,
    Return,
}

impl TryFrom<&str> for KeywordToken {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use KeywordToken::*;
        match value {
            "int" => Ok(Int),
            "void" => Ok(Void),
            "return" => Ok(Return),
            _ => Err("coercion from str to KeywordToken failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(KeywordToken),
    Symbol(SymbolToken),
    Comment(CommentToken),
}

lazy_static! {
    pub static ref IDENTIFIER_PATTERN: Regex = Regex::new(r"[a-zA-Z_]\w*\b").unwrap();
    pub static ref CONSTANT_PATTERN: Regex = Regex::new(r"[0-9]+\b").unwrap();
    pub static ref KEYWORD_PATTERN: Regex = Regex::new(r"int|void|return").unwrap();
    pub static ref SYMBOL_PATTERN: Regex =
        Regex::new(r#"\(|\)|\{|\}|;|\"|\n|\+|-|\*|\/|%|~| |\t|\n|--"#).unwrap();
    pub static ref COMMENT_PATTERN: Regex = Regex::new(r"\/\/|\/\*").unwrap();
}
