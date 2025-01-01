use regex::{Matches, Regex};
use std::fmt::Debug;
use tracing::debug;

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

    fn check_for_regex_at_start(&mut self, regex: Regex) -> bool {
        match regex.find(self.remaining_chars) {
            Some(found) => found.start() == 0,
            None => false,
        }
    }

    #[tracing::instrument]
    fn next_token(&mut self) -> Result<(usize, Token), &str> {
        if self.check_for_regex_at_start(Regex::new(IDENTIFIER_PATTERN).unwrap()) {
            Ok(self.handle_identifier())
        } else if self.check_for_regex_at_start(Regex::new(KEYWORD_PATTERN).unwrap()) {
            Ok(self.handle_keyword())
        } else if self.check_for_regex_at_start(Regex::new(CONSTANT_PATTERN).unwrap()) {
            Ok(self.handle_keyword())
        } else if self.check_for_regex_at_start(Regex::new(SYMBOL_PATTERN).unwrap()) {
            Ok(self.handle_symbol())
        } else if self.check_for_regex_at_start(Regex::new(COMMENT_PATTERN).unwrap()) {
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
