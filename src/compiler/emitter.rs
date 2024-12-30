use super::asm_tree::{AFunctionDefinitionNode, AInstructionNode, AProgramNode};

#[tracing::instrument(skip_all)]
pub fn emit_program(a_program: AProgramNode, output: &mut String) {
    let AProgramNode::Program(a_function) = a_program;
    emit_function(a_function, output);
    output.push_str("   .section .note.GNU-stack,\"\",@progbits\n");
}

fn emit_prologue(output: &mut String) {
    output.push_str(&format!("    pushq %rbp\n"));
    output.push_str(&format!("    movq %rsp, %rbp\n"));
}

fn emit_function(a_function: AFunctionDefinitionNode, output: &mut String) {
    let AFunctionDefinitionNode::Function(name, instructions) = a_function;
    output.push_str(&format!("   .globl {name}\n"));
    output.push_str(&format!("{name}:\n"));
    emit_prologue(output);
    for a_instruction in instructions {
        emit_instructions(a_instruction, output);
    }
}

#[tracing::instrument(skip(output), level = "debug")]
fn emit_instructions(a_instruction: AInstructionNode, output: &mut String) {
    output.push_str(&format!("{a_instruction}"));
}
