use super::{
    parser::{ExpressionNode, FunctionDefinitionNode, StatementNode, UnaryOperatorNode},
    tacker::{TFunctionDefinitionNode, TInstructionNode, TProgramNode, TValNode},
};

#[derive(Debug)]
pub enum ARegisterNode {
    AX,
    R10,
}

#[derive(Debug)]
pub enum AOperandNode {
    Imm(usize),
    Reg(ARegisterNode),
    Pseudo(String),
    Stack(isize),
}

#[derive(Debug)]
pub enum AUnaryOperatorNode {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum AInstructionNode {
    Mov(AOperandNode, AOperandNode),
    Unary(AUnaryOperatorNode, AOperandNode),
    AllocateStack(usize),
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

fn generate_operator(operator: UnaryOperatorNode) -> AUnaryOperatorNode {}

fn generate_operand(expression: TValNode) -> AOperandNode {}

fn generate_instruction(
    instruction: TInstructionNode,
    instruction_buffer: &mut Vec<AInstructionNode>,
) {
    return match instruction {
        TInstructionNode::Return(val) => {
            instruction_buffer.push(AInstructionNode::Mov(
                generate_operand(val),
                AOperandNode::Reg(ARegisterNode::AX),
            ));
            instruction_buffer.push(AInstructionNode::Ret);
        }
        TInstructionNode::Unary(op, src, dst) => {
            instruction_buffer.push(AInstructionNode::Mov(
                generate_operand(src),
                generate_operand(dst.clone()),
            ));
            instruction_buffer.push(AInstructionNode::Unary(
                generate_operator(op),
                generate_operand(dst),
            ));
        }
    };
}

fn generate_instructions(instructions: Vec<TInstructionNode>) -> Vec<AInstructionNode> {
    let mut instruction_buffer: Vec<AInstructionNode> = vec![];
    for instruction in instructions {
        generate_instruction(instruction, &mut instruction_buffer);
    }
    return instruction_buffer;
}

fn generate_function(function: TFunctionDefinitionNode) -> AFunctionDefinitionNode {
    let TFunctionDefinitionNode::Function(name, instructions) = function;
    return AFunctionDefinitionNode::Function(name, generate_instructions(instructions));
}

pub fn generate_program(program: TProgramNode) -> AProgramNode {
    let TProgramNode::Program(function) = program;
    return AProgramNode::Program(generate_function(function));
}
