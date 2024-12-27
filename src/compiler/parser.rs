use std::iter::Peekable;

use tracing::error;

use super::ast_tree::*;
use super::tokens::{KeywordToken, SymbolToken, Token};

fn parse_factor<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> ExpressionNode {
    // match <int>
    let first = tokens.next().unwrap().to_owned();
    if let Token::Constant(val) = first {
        return ExpressionNode::Constant(
            str::parse(&val).expect("Could not parse constant as int"),
        );
    } else if let Token::Symbol(operator) = first {
        if let SymbolToken::OpenParen = operator {
            let inner = parse_expression(tokens, 0);
            assert!(matches!(
                tokens.next().unwrap(),
                Token::Symbol(SymbolToken::CloseParen)
            ));
            return inner;
        } else {
            let operation: UnaryOperatorNode = match operator {
                SymbolToken::Minus => UnaryOperatorNode::Negate,
                SymbolToken::Tilde => UnaryOperatorNode::Complement,
                _ => {
                    error!("unrecognized symbol {:?} used as unary operator", operator);
                    panic!("Syntax error!");
                }
            };
            return ExpressionNode::Unary(operation, Box::new(parse_factor(tokens)));
        }
    } else {
        panic!("Syntax error!");
    }
}

fn operator_precedence(operator: &SymbolToken) -> isize {
    match operator {
        SymbolToken::Plus => 45,
        SymbolToken::Minus => 45,
        SymbolToken::Asterisk => 50,
        SymbolToken::ForwardSlash => 50,
        SymbolToken::Percent => 50,
        _ => panic!("finding precedence for unrecognized operator!"),
    }
}

fn parse_expression<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    min_precedence: isize,
) -> ExpressionNode {
    let mut left = parse_factor(tokens);
    loop {
        let next = tokens.peek().unwrap();
        if let Token::Symbol(sym) = next {
            use SymbolToken::*;
            match sym {
                Plus | Minus | Asterisk | ForwardSlash | Percent => {
                    let precedence = operator_precedence(sym);
                    if precedence < min_precedence {
                        break;
                    }
                    let operator = match tokens.next().unwrap() {
                        Token::Symbol(SymbolToken::Plus) => BinaryOperatorNode::Add,
                        Token::Symbol(SymbolToken::Minus) => BinaryOperatorNode::Subtract,
                        Token::Symbol(SymbolToken::Asterisk) => BinaryOperatorNode::Multiply,
                        Token::Symbol(SymbolToken::ForwardSlash) => BinaryOperatorNode::Divide,
                        Token::Symbol(SymbolToken::Percent) => BinaryOperatorNode::Remainder,
                        _ => panic!("unrecognized symbol used as operator in binop"),
                    };

                    let right = parse_expression(tokens, precedence + 1);
                    left = ExpressionNode::Binary(operator, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
    }
    return left;
}

fn parse_statement<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> StatementNode {
    // match "return"
    assert!(matches!(
        tokens.next().unwrap().to_owned(),
        Token::Keyword(KeywordToken::Return)
    ));

    // match <expression>
    let expression = parse_expression(tokens, 0);

    // match ";"
    assert!(matches!(
        tokens.next().unwrap().to_owned(),
        Token::Symbol(SymbolToken::Semicolon)
    ));

    return StatementNode::Return(expression);
}

fn parse_function<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> FunctionDefinitionNode {
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

#[tracing::instrument(skip_all)]
pub fn parse_program<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> ProgramNode {
    // match <function>
    let child = parse_function(tokens);

    // ensure tokens is empty after parsing
    assert!(matches!(tokens.next(), Option::None));

    return ProgramNode::Program(child);
}
