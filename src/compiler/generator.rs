use super::{
    asm_tree::*,
    ast_tree::UnaryOperatorNode,
    tac_tree::{TFunctionDefinitionNode, TInstructionNode, TProgramNode, TValNode},
};

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
        _ => unimplemented!(),
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

#[tracing::instrument(skip_all)]
pub fn generate_program(program: TProgramNode) -> AProgramNode {
    let TProgramNode::Program(function) = program;
    let postprocessed = postprocess_assembly(AProgramNode::Program(generate_function(function)));
    return postprocessed;
}

fn map_pseudoregister_name(identifier: &String, max_allocation: &mut isize) -> isize {
    let split: Vec<&str> = identifier.split(".").collect();
    // assume that all pseudoregisters are identified as "temp.{n}"
    assert_eq!(split.len(), 2);
    let count: isize = str::parse(split[1]).expect("Could not parse pseudoregister number");
    let current_max = *max_allocation;
    *max_allocation = std::cmp::max(current_max, (count + 1) * 4);
    return -((count + 1) * 4);
}

fn pseudoreg_to_stack(operand: &AOperandNode, mut max_allocation: &mut isize) -> AOperandNode {
    return match operand {
        AOperandNode::Pseudo(name) => {
            AOperandNode::Stack(map_pseudoregister_name(name, &mut max_allocation))
        }
        _ => operand.clone(),
    };
}

fn replace_instruction_pseudoregs(
    instruction: &AInstructionNode,
    mut max_allocation: &mut isize,
) -> AInstructionNode {
    return match instruction {
        AInstructionNode::Mov(op1, op2) => AInstructionNode::Mov(
            pseudoreg_to_stack(op1, &mut max_allocation),
            pseudoreg_to_stack(op2, &mut max_allocation),
        ),
        AInstructionNode::Unary(operator, operand) => AInstructionNode::Unary(
            operator.clone(),
            pseudoreg_to_stack(operand, max_allocation),
        ),
        _ => instruction.clone(),
    };
}

fn replace_pseudoregs(instructions: &mut Vec<AInstructionNode>, mut max_allocation: &mut isize) {
    for idx in 0..instructions.len() {
        instructions[idx] = replace_instruction_pseudoregs(&instructions[idx], &mut max_allocation)
    }
}

fn replace_invalid_moves(instruction: &AInstructionNode) -> Vec<AInstructionNode> {
    use AOperandNode::*;
    return match instruction {
        AInstructionNode::Mov(Stack(src), Stack(dst)) => vec![
            AInstructionNode::Mov(Stack(*src), Reg(ARegisterNode::R10)),
            AInstructionNode::Mov(Reg(ARegisterNode::R10), Stack(*dst)),
        ],
        _ => vec![instruction.clone()],
    };
}

fn validate_moves(
    instructions: &mut Vec<AInstructionNode>,
    max_allocation: isize,
) -> Vec<AInstructionNode> {
    let mut new_instructions: Vec<AInstructionNode> = vec![];
    new_instructions.push(AInstructionNode::AllocateStack(
        usize::try_from(max_allocation).expect("failed to convert max allocation to isize"),
    ));
    for idx in 0..instructions.len() {
        let instruction = &instructions[idx];
        let replaced_moves = replace_invalid_moves(instruction);
        let mut to_insert = replaced_moves.iter();
        while let Some(new_instruction) = to_insert.next() {
            new_instructions.push(new_instruction.clone());
        }
    }
    return new_instructions;
}

#[tracing::instrument(skip_all)]
fn postprocess_assembly(program: AProgramNode) -> AProgramNode {
    let AProgramNode::Program(function) = program;
    let AFunctionDefinitionNode::Function(name, mut instructions) = function;
    let mut max_allocation: isize = 0;
    replace_pseudoregs(&mut instructions, &mut max_allocation);
    let instructions = validate_moves(&mut instructions, max_allocation);
    return AProgramNode::Program(AFunctionDefinitionNode::Function(name, instructions));
}
