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
    CommentSymbol,
    Minus,
    Decrement,
    Tilde,
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
