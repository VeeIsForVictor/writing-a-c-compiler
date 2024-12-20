pub mod lexer {
    use regex::Regex;
    use std::{
        io::{Error, ErrorKind},
        str::Chars,
    };

    #[derive(Debug)]
    enum CommentToken {
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
        HandlingComment {
            remaining_chars: Chars<'a>,
            comment_value: CommentToken,
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
            '/' => Some(CommentSymbol),
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
        let identifier: Regex = Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap();
        if identifier.is_match(&value) {
            return Ok(Token::Identifier(value));
        }
        let constant: Regex = Regex::new(r"^[0-9]+\b$").unwrap();
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
                        Some(symbol) => match symbol {
                            SymbolToken::CommentSymbol => HandlingComment {
                                remaining_chars: remaining_chars,
                                comment_value: CommentToken::PendingComment,
                            },
                            _ => Done {
                                remaining_chars,
                                token: Symbol(symbol),
                            },
                        },
                        None => Building {
                            remaining_chars,
                            current_value: char.to_string(),
                        },
                    },
                },

                HandlingComment {
                    mut remaining_chars,
                    comment_value: comment_token,
                } => match comment_token {
                    CommentToken::PendingComment => match remaining_chars.next() {
                        Some(char) => {
                            if char == '/' {
                                HandlingComment {
                                    remaining_chars,
                                    comment_value: CommentToken::LineComment,
                                }
                            } else if char == '*' {
                                HandlingComment {
                                    remaining_chars,
                                    comment_value: CommentToken::BlockComment,
                                }
                            } else {
                                panic!("Impossible comment value");
                            }
                        }
                        None => {
                            panic!("Unexpected EOF")
                        }
                    },
                    CommentToken::LineComment => match remaining_chars.next() {
                        Some(char) => {
                            if char == '\n' {
                                Done {
                                    remaining_chars,
                                    token: Comment(CommentToken::LineComment),
                                }
                            } else {
                                HandlingComment {
                                    remaining_chars,
                                    comment_value: CommentToken::LineComment,
                                }
                            }
                        }
                        None => {
                            panic!("Unexpected EOF")
                        }
                    },
                    CommentToken::BlockComment => {
                        match remaining_chars.next().expect("Unexpected EOF") {
                            '*' => match remaining_chars.next().expect("Unexpected EOF") {
                                '/' => Done {
                                    remaining_chars,
                                    token: Comment(CommentToken::BlockComment),
                                },
                                _ => HandlingComment {
                                    remaining_chars,
                                    comment_value: CommentToken::BlockComment,
                                },
                            },
                            _ => HandlingComment {
                                remaining_chars,
                                comment_value: CommentToken::BlockComment,
                            },
                        }
                    }
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

pub mod parser {
    use std::option::Iter;

    use crate::compiler::lexer::{KeywordToken, SymbolToken};

    use super::lexer::Token;

    pub enum ExpressionNode {
        Constant(usize),
    }

    pub enum StatementNode {
        Return(ExpressionNode),
    }

    pub enum FunctionDefinitionNode {
        Function(String, StatementNode),
    }

    pub enum ProgramNode {
        Program(FunctionDefinitionNode),
    }

    fn parse_expression(tokens: &mut Iter<Token>) -> ExpressionNode {
        // match <int>
        let constant_token = tokens.next().unwrap().to_owned();
        assert!(matches!(constant_token, Token::Constant(_)));
        if let Token::Constant(val) = constant_token {
            return ExpressionNode::Constant(
                str::parse(val).expect("Could not parse constant as int"),
            );
        } else {
            panic!("Syntax error!");
        }
    }

    fn parse_statement(tokens: &mut Iter<Token>) -> StatementNode {
        // match "return"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Keyword(KeywordToken::Return)
        ));

        // match <expression>
        let expression = parse_expression(tokens);

        // match ";"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Symbol(SymbolToken::Semicolon)
        ));

        return StatementNode::Return(expression);
    }

    fn parse_function(tokens: &mut Iter<Token>) -> FunctionDefinitionNode {
        // match "int"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Keyword(KeywordToken::Int)
        ));

        // match <identifier>
        let name_token = tokens.next().unwrap().to_owned();
        assert!(matches!(name_token, Token::Identifier(_)));

        // match "("
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Symbol(SymbolToken::OpenParen)
        ));

        // match "void"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Keyword(KeywordToken::Void)
        ));

        // match ")"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Symbol(SymbolToken::CloseParen)
        ));

        // match "{"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Symbol(SymbolToken::OpenBrace)
        ));

        // match <statement>
        let statement = parse_statement(tokens);

        // match "}"
        assert!(matches!(
            tokens.next().unwrap().to_owned(),
            Token::Symbol(SymbolToken::CloseBrace)
        ));

        if let Token::Identifier(name) = name_token {
            return FunctionDefinitionNode::Function(name.to_owned(), statement);
        } else {
            panic!("Syntax error!");
        }
    }

    pub fn parse_program(tokens: &mut Iter<Token>) -> ProgramNode {
        let child = parse_function(tokens);
        return ProgramNode::Program(child);
    }
}
