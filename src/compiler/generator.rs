use super::{
    parser::UnaryOperatorNode,
    tacker::{TFunctionDefinitionNode, TInstructionNode, TProgramNode, TValNode},
};

#[derive(Debug, Clone)]
pub enum ARegisterNode {
    AX,
    R10,
}

#[derive(Debug, Clone)]
pub enum AOperandNode {
    Imm(usize),
    Reg(ARegisterNode),
    Pseudo(String),
    Stack(isize),
}

#[derive(Debug, Clone)]
pub enum AUnaryOperatorNode {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
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

fn pseudoreg_to_stack(operand: &AOperandNode) -> AOperandNode {
    return match operand {
        AOperandNode::Pseudo(name) => {}
        _ => operand.clone(),
    };
}

fn replace_instruction_pseudoregs(instruction: &AInstructionNode) -> AInstructionNode {
    return match instruction {
        AInstructionNode::Mov(op1, op2) => {
            AInstructionNode::Mov(pseudoreg_to_stack(op1), pseudoreg_to_stack(op2))
        }
        AInstructionNode::Unary(operator, operand) => {
            AInstructionNode::Unary(operator.clone(), pseudoreg_to_stack(operand))
        }
        _ => instruction.clone(),
    };
}

fn replace_pseudoregs(instructions: &mut Vec<AInstructionNode>) {
    for idx in 0..instructions.len() {
        instructions[idx] = replace_instruction_pseudoregs(&instructions[idx])
    }
}

fn postprocess_assembly(program: AProgramNode) -> AProgramNode {
    let AProgramNode::Program(function) = program;
    let AFunctionDefinitionNode::Function(name, instructions) = function;
    return AProgramNode::Program(AFunctionDefinitionNode::Function(name, instructions));
}
