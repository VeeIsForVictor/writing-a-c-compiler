use std::iter::Peekable;

use tracing::error;

use super::ast_tree::*;
use super::tokens::{KeywordToken, SymbolToken, Token};

#[tracing::instrument(skip_all)]
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
                SymbolToken::Exclamation => UnaryOperatorNode::Not,
                _ => {
                    error!("unrecognized symbol {:?} used as unary operator", operator);
                    panic!("Syntax error!");
                }
            };
            return ExpressionNode::Unary(operation, Box::new(parse_factor(tokens)));
        }
    } else if let Token::Identifier(name) = first {
        return ExpressionNode::Var(name);
    } else {
        error!("first token {:?} not a valid start", first);
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
        SymbolToken::LeftAngleBracket => 35,
        SymbolToken::LeftABEqual => 35,
        SymbolToken::RightAngleBracket => 35,
        SymbolToken::RightABEqual => 35,
        SymbolToken::DoubleEqual => 30,
        SymbolToken::ExclamationEqual => 30,
        SymbolToken::DoubleAmpersand => 10,
        SymbolToken::DoubleBar => 5,
        SymbolToken::Equal => 1,
        _ => panic!("finding precedence for unrecognized operator!"),
    }
}

#[tracing::instrument(skip_all)]
fn parse_expression<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    min_precedence: isize,
) -> ExpressionNode {
    let mut left = parse_factor(tokens);
    loop {
        let next = tokens.peek().unwrap();
        if let Token::Symbol(sym) = next {
            use SymbolToken::*;
            // handle the case of an assignment operation
            if matches!(sym, Equal) {
                // discard the '=' operator
                tokens.next();
                let right = parse_expression(tokens, operator_precedence(sym));
                left = ExpressionNode::Assignment(Box::new(left), Box::new(right));
                continue;
            }
            match sym {
                Plus | Minus | Asterisk | ForwardSlash | Percent | DoubleAmpersand | DoubleBar
                | DoubleEqual | ExclamationEqual | LeftAngleBracket | LeftABEqual
                | RightAngleBracket | RightABEqual => {
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
                        Token::Symbol(SymbolToken::DoubleAmpersand) => BinaryOperatorNode::And,
                        Token::Symbol(SymbolToken::DoubleBar) => BinaryOperatorNode::Or,
                        Token::Symbol(SymbolToken::DoubleEqual) => BinaryOperatorNode::Equal,
                        Token::Symbol(SymbolToken::ExclamationEqual) => {
                            BinaryOperatorNode::NotEqual
                        }
                        Token::Symbol(SymbolToken::LeftAngleBracket) => {
                            BinaryOperatorNode::LessThan
                        }
                        Token::Symbol(SymbolToken::LeftABEqual) => BinaryOperatorNode::LessOrEqual,
                        Token::Symbol(SymbolToken::RightAngleBracket) => {
                            BinaryOperatorNode::GreaterThan
                        }
                        Token::Symbol(SymbolToken::RightABEqual) => {
                            BinaryOperatorNode::GreaterOrEqual
                        }
                        _ => panic!("unrecognized symbol used as operator in binop"),
                    };

                    let right = parse_expression(tokens, precedence + 1);
                    left = ExpressionNode::Binary(operator, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        } else {
            break;
        }
    }
    return left;
}

#[tracing::instrument(skip_all)]
fn parse_statement<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> StatementNode {
    let statement: StatementNode = match tokens.peek().unwrap() {
        Token::Keyword(KeywordToken::Return) => {
            // match "return"
            assert!(matches!(
                tokens.next().unwrap().to_owned(),
                Token::Keyword(KeywordToken::Return)
            ));

            // match <expression>
            let expression = parse_expression(tokens, 0);

            StatementNode::Return(expression)
        }
        Token::Identifier(_) => {
            let expression = parse_expression(tokens, 0);

            StatementNode::Expression(expression)
        }
        Token::Symbol(SymbolToken::Semicolon) => StatementNode::Null,
        _ => panic!("unexpected token beginning statement"),
    };

    // ensure statement is closed properly
    assert!(matches!(
        tokens.next().unwrap().to_owned(),
        Token::Symbol(SymbolToken::Semicolon)
    ));

    return statement;
}

#[tracing::instrument(skip_all)]
fn parse_declaration<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> DeclarationNode {
    // match "int"
    assert!(matches!(
        tokens.next().unwrap().to_owned(),
        Token::Keyword(KeywordToken::Int)
    ));

    // match <identifier>
    let identifier_token = tokens.next().unwrap().to_owned();
    assert!(matches!(identifier_token, Token::Identifier(_)));

    // match next symbol as ";" or "="

    let expression = match tokens.next().unwrap().to_owned() {
        Token::Symbol(SymbolToken::Semicolon) => None,
        Token::Symbol(SymbolToken::Equal) => {
            let ret = Some(parse_expression(tokens, 0)); // match ";"
            assert!(matches!(
                tokens.next().unwrap().to_owned(),
                Token::Symbol(SymbolToken::Semicolon)
            ));
            ret
        }
        _ => panic!("syntax error!"),
    };

    if let Token::Identifier(name) = identifier_token {
        return DeclarationNode::Declaration(name.to_owned(), expression);
    } else {
        panic!("syntax error!");
    }
}

fn parse_block_item<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> BlockItemNode {
    let next = tokens.peek().unwrap();
    match next {
        Token::Keyword(KeywordToken::Return) => {
            BlockItemNode::StatementItem(parse_statement(tokens))
        }
        Token::Keyword(KeywordToken::Int) => {
            BlockItemNode::DeclarationItem(parse_declaration(tokens))
        }
        Token::Identifier(_) => BlockItemNode::StatementItem(parse_statement(tokens)),
        _ => {
            error!("unexpected token {:?} in block item", next);
            panic!("syntax error!");
        }
    }
}

fn parse_function_definition<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Vec<BlockItemNode> {
    let mut items = vec![];

    while (!matches!(
        tokens.peek().unwrap().to_owned(),
        Token::Symbol(SymbolToken::CloseBrace)
    )) {
        items.push(parse_block_item(tokens));
    }

    return items;
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
    let definition = parse_function_definition(tokens);

    // match "}"
    assert!(matches!(
        tokens.next().unwrap().to_owned(),
        Token::Symbol(SymbolToken::CloseBrace)
    ));

    if let Token::Identifier(name) = name_token {
        return FunctionDefinitionNode::Function(name.to_owned(), definition);
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
