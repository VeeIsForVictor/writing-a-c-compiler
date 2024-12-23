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

pub fn generate_operand(expression: ExpressionNode) -> AOperandNode {}

pub fn generate_instruction(statement: StatementNode) -> AInstructionNode {}

pub fn generate_function(function: FunctionDefinitionNode) -> AFunctionDefinitionNode {}

pub fn generate_program(program: ProgramNode) -> AProgramNode {}
