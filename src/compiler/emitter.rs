use super::generator::{AFunctionDefinitionNode, AInstructionNode, AOperandNode, AProgramNode};

pub fn emit_program(a_program: AProgramNode, output: &mut String) {
    output.push_str("   .section .note.GNU-stack,\"\",@progbits\n");
}

pub fn emit_function(a_function: AFunctionDefinitionNode, output: &mut String) {
    let AFunctionDefinitionNode::Function(name, instructions) = a_function;
    output.push_str(&format!("   .globl {name}\n"));
    for a_instruction in instructions {
        emit_instructions(a_instruction, output);
    }
}

pub fn emit_instructions(a_instruction: AInstructionNode, output: &mut String) {
    if let AInstructionNode::Mov(src, dst) = a_instruction {
        let src = direct_emit_operand(src);
        let dst = direct_emit_operand(dst);
        output.push_str(&format!("   movl    {src}, {dst}\n"));
    } else {
        assert!(matches!(a_instruction, AInstructionNode::Ret));
        output.push_str("ret\n");
    }
}

pub fn direct_emit_operand(a_operand: AOperandNode) -> String {
    if let AOperandNode::Imm(c) = a_operand {
        return format!("${c}");
    } else {
        assert!(matches!(a_operand, AOperandNode::Register));
        return format!("%eax");
    }
}
