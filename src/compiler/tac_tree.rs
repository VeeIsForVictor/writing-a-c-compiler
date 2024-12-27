use super::ast_tree::{BinaryOperatorNode, UnaryOperatorNode};

#[derive(Debug, Clone)]
pub enum TValNode {
    Constant(usize),
    Var(String),
}

#[derive(Debug)]
pub enum TInstructionNode {
    Return(TValNode),
    Unary(UnaryOperatorNode, TValNode, TValNode),
    Binary(BinaryOperatorNode, TValNode, TValNode, TValNode),
}

#[derive(Debug)]
pub enum TFunctionDefinitionNode {
    Function(String, Vec<TInstructionNode>),
}

#[derive(Debug)]
pub enum TProgramNode {
    Program(TFunctionDefinitionNode),
}
