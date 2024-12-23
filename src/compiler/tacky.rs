use super::parser::{ExpressionNode, FunctionDefinitionNode, ProgramNode, StatementNode};

#[derive(Debug)]
pub enum TUnitaryOperatorNode {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum TValNode {
    Constant(usize),
    Var(String),
}

#[derive(Debug)]
pub enum TInstructionNode {
    Mov(TValNode, TValNode),
    Ret,
}

#[derive(Debug)]
pub enum TFunctionDefinitionNode {
    Function(String, Vec<TInstructionNode>),
}

#[derive(Debug)]
pub enum TProgramNode {
    Program(TFunctionDefinitionNode),
}

fn tack_val(expression: ExpressionNode) -> TValNode {
    if let ExpressionNode::Constant(c) = expression {
        return TValNode::Constant(c);
    } else {
        panic!("Not yet implemented!");
    }
}

fn tack_instructions(statement: StatementNode) -> Vec<TInstructionNode> {
    let StatementNode::Return(expression) = statement;
    return vec![];
}

fn tack_functions(function: FunctionDefinitionNode) -> TFunctionDefinitionNode {
    let FunctionDefinitionNode::Function(name, statement) = function;
    return TFunctionDefinitionNode::Function(name, tack_instructions(statement));
}

pub fn tack_program(program: ProgramNode) -> TProgramNode {
    let ProgramNode::Program(function) = program;
    return TProgramNode::Program(tack_functions(function));
}
