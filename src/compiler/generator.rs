use super::{
    parser::UnaryOperatorNode,
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

fn generate_operator(operator: UnaryOperatorNode) -> AUnaryOperatorNode {
    return match operator {
        UnaryOperatorNode::Complement => AUnaryOperatorNode::Not,
        UnaryOperatorNode::Negate => AUnaryOperatorNode::Neg,
    };
}

fn generate_operand(operand: TValNode) -> AOperandNode {
    return match operand {
        TValNode::Constant(c) => AOperandNode::Imm(c),
        TValNode::Var(id) => AOperandNode::Pseudo(id),
    };
}

fn generate_instruction(instruction: TInstructionNode) -> Vec<AInstructionNode> {
    return match instruction {
        TInstructionNode::Return(val) => {
            vec![
                AInstructionNode::Mov(generate_operand(val), AOperandNode::Reg(ARegisterNode::AX)),
                AInstructionNode::Ret,
            ]
        }
        TInstructionNode::Unary(op, src, dst) => {
            vec![
                AInstructionNode::Mov(generate_operand(src), generate_operand(dst.clone())),
                AInstructionNode::Unary(generate_operator(op), generate_operand(dst)),
            ]
        }
    };
}

fn generate_instructions(instructions: Vec<TInstructionNode>) -> Vec<AInstructionNode> {
    let mut instruction_buffer: Vec<AInstructionNode> = vec![];
    for instruction in instructions {
        instruction_buffer.append(&mut generate_instruction(instruction));
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

fn validate_moves(instructions: &mut Vec<AInstructionNode>) {}

fn replace_pseudoregs(instructions: &mut Vec<AInstructionNode>) {}

fn postprocess_assembly(program: AProgramNode) -> AProgramNode {
    let AProgramNode::Program(function) = program;
    let AFunctionDefinitionNode::Function(name, instructions) = function;
    return AProgramNode::Program(AFunctionDefinitionNode::Function(name, instructions));
}
