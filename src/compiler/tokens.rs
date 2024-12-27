#[derive(Debug)]
pub enum CommentToken {
    LineComment,
    BlockComment,
    PendingComment,
}

#[derive(Debug)]
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

impl TryFrom<char> for SymbolToken {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use SymbolToken::*;
        match value {
            '(' => Ok(OpenParen),
            ')' => Ok(CloseParen),
            '{' => Ok(OpenBrace),
            '}' => Ok(CloseBrace),
            ';' => Ok(Semicolon),
            '\"' => Ok(Quote),
            '\n' | ' ' | '\t' => Ok(Whitespace),
            '+' => Ok(Plus),
            '-' => Ok(Minus),
            '*' => Ok(Asterisk),
            '/' => Ok(ForwardSlash),
            '%' => Ok(Percent),
            '~' => Ok(Tilde),
            _ => Err("coercion from char to SymbolToken failed"),
        }
    }
}

#[derive(Debug)]
pub enum KeywordToken {
    Int,
    Void,
    Return,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(KeywordToken),
    Symbol(SymbolToken),
    Comment(CommentToken),
}
