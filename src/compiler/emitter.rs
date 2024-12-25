use super::generator::{
    AFunctionDefinitionNode, AInstructionNode, AOperandNode, AProgramNode, ARegisterNode,
    AUnaryOperatorNode,
};

pub fn emit_program(a_program: AProgramNode, output: &mut String) {
    let AProgramNode::Program(a_function) = a_program;
    emit_function(a_function, output);
    output.push_str("   .section .note.GNU-stack,\"\",@progbits\n");
}

pub fn emit_function(a_function: AFunctionDefinitionNode, output: &mut String) {
    let AFunctionDefinitionNode::Function(name, instructions) = a_function;
    output.push_str(&format!("   .globl {name}\n"));
    output.push_str(&format!("{name}:\n"));
    for a_instruction in instructions {
        emit_instructions(a_instruction, output);
    }
}

pub fn emit_instructions(a_instruction: AInstructionNode, output: &mut String) {
    match a_instruction {
        AInstructionNode::Mov(src, dst) => {
            let src = direct_emit_operand(src);
            let dst = direct_emit_operand(dst);
            output.push_str(&format!("   movl    {src}, {dst}\n"));
        }
        AInstructionNode::Ret => {
            output.push_str("   ret\n");
        }
        AInstructionNode::Unary(operator, operand) => {}
        AInstructionNode::AllocateStack(size) => {}
    }
}

pub fn direct_emit_operand(a_operand: AOperandNode) -> String {
    match a_operand {
        AOperandNode::Imm(c) => format!("${c}"),
        AOperandNode::Reg(ARegisterNode::AX) => format!("%eax"),
        AOperandNode::Stack(addr) => format!(""),
        _ => panic!("invalid operand found in emitter stage"),
    }
}

pub fn direct_emit_operator(a_operator: AUnaryOperatorNode) -> String {
    match a_operator {
        AUnaryOperatorNode::Neg => String::from("negl"),
        AUnaryOperatorNode::Not => String::from("notl"),
    }
}
