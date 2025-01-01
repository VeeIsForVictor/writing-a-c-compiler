use std::fmt::Debug;

use super::tokens::*;

#[derive(Debug)]
struct Tokenizer<'a> {
    remaining_chars: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            remaining_chars: src,
            tokens: Vec::new(),
        }
    }

    fn handle_identifier(&mut self) -> (usize, Token) {
        let mut matches = self.remaining_chars.matches(IDENTIFIER_PATTERN);
        if let Some(identifier) = matches.next() {
            (
                identifier.len(),
                Token::Identifier(String::from(identifier)),
            )
        } else {
            panic!("something weird happened while handling identifier")
        }
    }

    fn handle_constant(&mut self) -> (usize, Token) {
        let mut matches = self.remaining_chars.matches(CONSTANT_PATTERN);
        if let Some(constant) = matches.next() {
            (constant.len(), Token::Constant(String::from(constant)))
        } else {
            panic!("something weird happened while handling constant")
        }
    }

    fn handle_keyword(&mut self) -> (usize, Token) {
        let mut matches = self.remaining_chars.matches(KEYWORD_PATTERN);
        if let Some(keyword) = matches.next() {
            (
                keyword.len(),
                Token::Keyword(KeywordToken::try_from(keyword).unwrap()),
            )
        } else {
            panic!("something weird happened while handling keyword")
        }
    }

    fn handle_symbol(&mut self) -> (usize, Token) {
        let mut matches = self.remaining_chars.matches(SYMBOL_PATTERN);
        if let Some(sym) = matches.next() {
            (
                sym.len(),
                Token::Symbol(SymbolToken::try_from(sym).unwrap()),
            )
        } else {
            panic!("something weird happened while handling symbol")
        }
    }

    fn handle_comment(&mut self) -> (usize, Token) {
        let mut matches = self.remaining_chars.matches(COMMENT_PATTERN);
        let mut comment_type = CommentToken::PendingComment;
        if let Some(comment) = matches.next() {
            comment_type = CommentToken::try_from(comment).unwrap()
        } else {
            panic!("something weird happened while handling comment")
        }

        let comment_end = match comment_type {
            CommentToken::BlockComment => self.remaining_chars.find("*/"),
            CommentToken::LineComment => self.remaining_chars.find("\n"),
            CommentToken::PendingComment => panic!(
                "found PendingComment while handling comment, was expecting any other comment type"
            ),
        };

        match comment_end {
            Some(len) => (len, Token::Comment(comment_type)),
            None => (self.remaining_chars.len() - 1, Token::Comment(comment_type)),
        }
    }

    #[tracing::instrument]
    fn next_token(&mut self) -> Result<(usize, Token), &str> {
        let remaining_chars = self.remaining_chars;
        if remaining_chars.starts_with(IDENTIFIER_PATTERN) {
            Ok(self.handle_identifier())
        } else if remaining_chars.starts_with(CONSTANT_PATTERN) {
            Ok(self.handle_keyword())
        } else if remaining_chars.starts_with(KEYWORD_PATTERN) {
            Ok(self.handle_keyword())
        } else if remaining_chars.starts_with(SYMBOL_PATTERN) {
            Ok(self.handle_symbol())
        } else if remaining_chars.starts_with(COMMENT_PATTERN) {
            Ok(self.handle_comment())
        } else {
            Err("no more tokens left to parse in non-empty remaining_chars")
        }
    }

    fn advance_chars(&mut self, idx: usize) -> usize {
        match self.remaining_chars.get(idx..) {
            Some(new_src) => {
                self.remaining_chars = new_src;
                return self.remaining_chars.len();
            }
            None => return 0,
        }
    }

    #[tracing::instrument]
    fn tokenize(&mut self) -> Vec<Token> {
        while self.remaining_chars.len() > 0 {
            let (advance, token) = self.next_token().unwrap();
            self.tokens.push(token);
            self.advance_chars(advance);
        }
        return self.tokens.clone();
    }
}

#[tracing::instrument]
pub fn lex(code: String) -> Vec<Token> {
    return Tokenizer::new(&code).tokenize();
}
