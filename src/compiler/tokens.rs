#[derive(Debug, Clone)]
pub enum CommentToken {
    LineComment,
    BlockComment,
    PendingComment,
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

pub static IDENTIFIER_PATTERN: &str = r"^[a-zA-Z_]\w*\b$";
pub static CONSTANT_PATTERN: &str = r"^[0-9]+\b$";
pub static KEYWORD_PATTERN: &str = r"int|void|return";
pub static SYMBOL_PATTERN: &str = r#"(|)|{|}|;|\"|\n|+|-|*|/|%|~|\s|--"#;
pub static COMMENT_PATTERN: &str = r"\/\/|\/\*";
