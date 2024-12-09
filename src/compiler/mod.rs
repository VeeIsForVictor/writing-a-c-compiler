pub mod lexer {
    use regex::Regex;
    use std::{
        io::{Error, ErrorKind},
        str::Chars,
    };

    #[derive(Debug)]
    enum SymbolToken {
        OpenParen,
        CloseParen,
        OpenBrace,
        CloseBrace,
        Semicolon,
        Quote,
        Whitespace,
    }

    #[derive(Debug)]
    enum KeywordToken {
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
    }

    #[derive(Debug)]
    enum ReadState<'a> {
        Ready {
            remaining_chars: Chars<'a>,
        },
        Building {
            remaining_chars: Chars<'a>,
            current_value: String,
        },
        Done {
            remaining_chars: Chars<'a>,
            token: Token,
        },
        Exit,
    }

    fn check_for_symbol(ch: char) -> Option<SymbolToken> {
        use SymbolToken::*;
        match ch {
            '(' => Some(OpenParen),
            ')' => Some(CloseParen),
            '{' => Some(OpenBrace),
            '}' => Some(CloseBrace),
            ';' => Some(Semicolon),
            '\"' => Some(Quote),
            '\n' | ' ' | '\t' => Some(Whitespace),
            _ => None,
        }
    }

    fn match_non_symbol_token(value: String) -> Result<Token, Error> {
        use KeywordToken::*;
        use Token::*;
        match value.as_str() {
            "int" => Ok(Keyword(Int)),
            "void" => Ok(Keyword(Void)),
            "return" => Ok(Keyword(Return)),
            _ => match_identifier_or_constant(value),
        }
    }

    fn match_identifier_or_constant(value: String) -> Result<Token, Error> {
        // catching the weird inclusion of @ in [a-zA-Z_]
        if value.chars().next().unwrap() == '@' {
            return Err(Error::new(ErrorKind::Other, format!("{value} contains @")));
        }
        let identifier: Regex = Regex::new(r"[a-zA-Z_]\w*\b").unwrap();
        if identifier.is_match(&value) {
            return Ok(Token::Identifier(value));
        }
        let constant: Regex = Regex::new(r"[0-9]+\b").unwrap();
        if constant.is_match(&value) {
            return Ok(Token::Constant(value));
        }
        Err(Error::new(
            ErrorKind::Other,
            format!("{value} did not match an identifier or a constant"),
        ))
    }

    fn consume<'a>(chars: Chars, mut vec: Vec<Token>) -> Vec<Token> {
        use ReadState::*;
        use Token::*;
        let mut state = Ready {
            remaining_chars: chars,
        };
        loop {
            state = match state {
                Ready {
                    mut remaining_chars,
                } => match remaining_chars.next() {
                    None => Exit,
                    Some(char) => match check_for_symbol(char) {
                        Some(symbol) => Done {
                            remaining_chars,
                            token: Symbol(symbol),
                        },
                        None => Building {
                            remaining_chars,
                            current_value: char.to_string(),
                        },
                    },
                },

                Building {
                    mut remaining_chars,
                    mut current_value,
                } => match remaining_chars.clone().peekable().peek() {
                    None => Done {
                        remaining_chars,
                        token: match_non_symbol_token(current_value)
                            .expect("Non-symbol token matching raised an error"),
                    },
                    Some(char) => {
                        if let Some(_) = check_for_symbol(char.to_owned()) {
                            Done {
                                remaining_chars: remaining_chars,
                                token: match_non_symbol_token(current_value)
                                    .expect("Non-symbol token matching raised an error"),
                            }
                        } else if char.to_owned() == ' ' {
                            Done {
                                remaining_chars: remaining_chars,
                                token: match_non_symbol_token(current_value)
                                    .expect("Non-symbol token matching raised an error"),
                            }
                        } else {
                            let new_char =
                                remaining_chars.next().expect("Something weird happened");
                            current_value.push(new_char);
                            Building {
                                remaining_chars: remaining_chars,
                                current_value: current_value,
                            }
                        }
                    }
                },

                Done {
                    remaining_chars,
                    token,
                } => {
                    vec.push(token);
                    Ready {
                        remaining_chars: remaining_chars.to_owned(),
                    }
                }
                Exit => return vec,
            };
        }
    }

    pub fn lex(code: String) -> Vec<Token> {
        let chars = code.chars();
        let vec = vec![];
        return consume(chars, vec);
    }
}
