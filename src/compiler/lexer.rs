use regex::{Matches, Regex};
use std::fmt::Debug;
use tracing::{debug, info};

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
        let mut matches = IDENTIFIER_PATTERN.find(self.remaining_chars);
        if let Some(identifier) = matches {
            (
                identifier.len(),
                Token::Identifier(String::from(identifier.as_str())),
            )
        } else {
            panic!("something weird happened while handling identifier")
        }
    }

    fn handle_constant(&mut self) -> (usize, Token) {
        let matches = CONSTANT_PATTERN.find(self.remaining_chars);
        if let Some(constant) = matches {
            (
                constant.len(),
                Token::Constant(String::from(constant.as_str())),
            )
        } else {
            panic!("something weird happened while handling constant")
        }
    }

    fn handle_keyword(&mut self) -> (usize, Token) {
        let matches = KEYWORD_PATTERN.find(self.remaining_chars);
        if let Some(keyword) = matches {
            (
                keyword.len(),
                Token::Keyword(KeywordToken::try_from(keyword.as_str()).unwrap()),
            )
        } else {
            panic!("something weird happened while handling keyword")
        }
    }

    #[tracing::instrument]
    fn handle_symbol(&mut self) -> (usize, Token) {
        let matches = SYMBOL_PATTERN.find(self.remaining_chars);
        if let Some(sym) = matches {
            info!("{:?}", sym);
            (
                sym.len(),
                Token::Symbol(SymbolToken::try_from(sym.as_str()).unwrap()),
            )
        } else {
            panic!("something weird happened while handling symbol")
        }
    }

    fn handle_comment(&mut self) -> (usize, Token) {
        let matches = COMMENT_PATTERN.find(self.remaining_chars);
        let mut comment_type = CommentToken::PendingComment;
        if let Some(comment) = matches {
            comment_type = CommentToken::try_from(comment.as_str()).unwrap()
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

    fn check_for_regex_at_start(&mut self, re: &str) -> bool {
        let regex = Regex::new(re).unwrap();
        match regex.find(self.remaining_chars) {
            Some(found) => found.start() == 0,
            None => false,
        }
    }

    #[tracing::instrument]
    fn next_token(&mut self) -> Result<(usize, Token), &str> {
        if self.check_for_regex_at_start(IDENTIFIER_PATTERN.as_str()) {
            Ok(self.handle_identifier())
        } else if self.check_for_regex_at_start(KEYWORD_PATTERN.as_str()) {
            Ok(self.handle_keyword())
        } else if self.check_for_regex_at_start(CONSTANT_PATTERN.as_str()) {
            Ok(self.handle_constant())
        } else if self.check_for_regex_at_start(SYMBOL_PATTERN.as_str()) {
            Ok(self.handle_symbol())
        } else if self.check_for_regex_at_start(COMMENT_PATTERN.as_str()) {
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
