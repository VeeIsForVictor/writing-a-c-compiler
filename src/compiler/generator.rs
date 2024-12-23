use super::parser::{ExpressionNode, FunctionDefinitionNode, ProgramNode, StatementNode};

#[derive(Debug)]
pub enum AOperandNode {
    Imm(usize),
    Register,
}

#[derive(Debug)]
pub enum AInstructionNode {
    Mov(AOperandNode, AOperandNode),
    Ret,
}

#[derive(Debug)]
pub enum AFunctionDefinitionNode {
    Function(String, Vec<AInstructionNode>),
}

#[derive(Debug)]
pub enum AProgramNode {
    Program(AFunctionDefinitionNode),
}

fn generate_operand(expression: ExpressionNode) -> AOperandNode {
    let ExpressionNode::Constant(c) = expression;
    return AOperandNode::Imm(c);
}

fn generate_instructions(statement: StatementNode) -> Vec<AInstructionNode> {
    let StatementNode::Return(expression) = statement;
    return vec![
        AInstructionNode::Mov(generate_operand(expression), AOperandNode::Register),
        AInstructionNode::Ret,
    ];
}

fn generate_function(function: FunctionDefinitionNode) -> AFunctionDefinitionNode {
    let FunctionDefinitionNode::Function(name, statement) = function;
    return AFunctionDefinitionNode::Function(name, generate_instructions(statement));
}

pub fn generate_program(program: ProgramNode) -> AProgramNode {
    let ProgramNode::Program(function) = program;
    return AProgramNode::Program(generate_function(function));
}
