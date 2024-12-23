use crate::compiler::lexer::{KeywordToken, SymbolToken};

use super::lexer::Token;

#[derive(Debug)]
pub enum UnaryOperatorNode {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Constant(usize),
    Unary(UnaryOperatorNode, Box<ExpressionNode>),
}

#[derive(Debug)]
pub enum StatementNode {
    Return(ExpressionNode),
}

#[derive(Debug)]
pub enum FunctionDefinitionNode {
    Function(String, StatementNode),
}

#[derive(Debug)]
pub enum ProgramNode {
    Program(FunctionDefinitionNode),
}

fn parse_expression<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> ExpressionNode {
    // match <int>
    let constant_token = tokens.next().unwrap().to_owned();
    assert!(matches!(constant_token, Token::Constant(_)));
    if let Token::Constant(val) = constant_token {
        return ExpressionNode::Constant(str::parse(val).expect("Could not parse constant as int"));
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

pub fn parse_program<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> ProgramNode {
    // match <function>
    let child = parse_function(tokens);

    // ensure tokens is empty after parsing
    assert!(matches!(tokens.next(), Option::None));

    return ProgramNode::Program(child);
}
