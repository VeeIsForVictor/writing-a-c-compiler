use tracing::error;

use super::ast_tree::*;
use super::tokens::{KeywordToken, SymbolToken, Token};

fn parse_expression<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> ExpressionNode {
    // match <int>
    let first = tokens.next().unwrap().to_owned();
    if let Token::Constant(val) = first {
        return ExpressionNode::Constant(str::parse(val).expect("Could not parse constant as int"));
    } else if let Token::Symbol(operator) = first {
        if let SymbolToken::OpenParen = operator {
            let inner = parse_expression(tokens);
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
            return ExpressionNode::Unary(operation, Box::new(parse_expression(tokens)));
        }
    } else {
        panic!("Syntax error!");
    }
}

fn parse_statement<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> StatementNode {
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

fn parse_function<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> FunctionDefinitionNode {
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
pub fn parse_program<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> ProgramNode {
    // match <function>
    let child = parse_function(tokens);

    // ensure tokens is empty after parsing
    assert!(matches!(tokens.next(), Option::None));

    return ProgramNode::Program(child);
}
