use std::sync::Mutex;

use tracing::{debug, error};

use super::{ast_tree::*, tac_tree::*};

static TEMPORARY_COUNTER: Mutex<usize> = Mutex::new(0);
static LABEL_COUNTER: Mutex<usize> = Mutex::new(0);

#[tracing::instrument]
fn make_temporary_var() -> String {
    debug!("temporary variable creation called");
    match TEMPORARY_COUNTER.lock() {
        Ok(mut counter) => {
            let temp = *counter;
            *counter += 1;
            debug!("temporary variable {temp} created");
            format!("tmp.{temp}")
        }
        Err(e) => {
            error!("temporary variable counter mutex was poisoned: {e:?}");
            panic!("Concurrency panic!");
        }
    }
}

#[tracing::instrument]
fn make_label_name(op: &BinaryOperatorNode, suffix: &str) -> String {
    debug!("label name creation called");
    let label_name = match op {
        BinaryOperatorNode::And => "false_label",
        BinaryOperatorNode::Or => "true_label",
        _ => unimplemented!(),
    };
    match LABEL_COUNTER.lock() {
        Ok(mut counter) => {
            let temp = *counter;
            *counter += 1;
            debug!("label anme {temp} created");
            format!("{label_name}_{temp}{suffix}")
        }
        Err(e) => {
            error!("temporary variable counter mutex was poisoned: {e:?}");
            panic!("Concurrency panic!");
        }
    }
}

fn handle_regular_operation(
    operator: BinaryOperatorNode,
    op1: Box<ExpressionNode>,
    op2: Box<ExpressionNode>,
    instruction_buffer: &mut Vec<TInstructionNode>,
) -> TValNode {
    let v1 = tack_exp(*op1, instruction_buffer);
    let v2 = tack_exp(*op2, instruction_buffer);
    let dst_name = make_temporary_var();
    let dst = TValNode::Var(dst_name);
    let ret = dst.clone();
    instruction_buffer.push(TInstructionNode::Binary(operator, v1, v2, dst));
    return ret;
}

fn handle_shortcircuiting_operation(
    operator: BinaryOperatorNode,
    op1: Box<ExpressionNode>,
    op2: Box<ExpressionNode>,
    instruction_buffer: &mut Vec<TInstructionNode>,
) -> TValNode {
    let sclabel_name = make_label_name(&operator, "");
    let end_label_name = make_label_name(&operator, "_end");
    let shortcircuit_label = TInstructionNode::Label(sclabel_name.clone());
    let end_label = TInstructionNode::Label(end_label_name.clone());
    let jump_op = match operator {
        BinaryOperatorNode::And => TInstructionNode::JumpIfZero,
        BinaryOperatorNode::Or => TInstructionNode::JumpIfNotZero,
        _ => unimplemented!(),
    };
    let v1 = tack_exp(*op1, instruction_buffer);
    instruction_buffer.push(jump_op(v1, sclabel_name.clone()));
    let v2 = tack_exp(*op2, instruction_buffer);
    instruction_buffer.push(jump_op(v2, sclabel_name));
    let jump_val = match operator {
        BinaryOperatorNode::And => 0,
        BinaryOperatorNode::Or => 1,
        _ => unimplemented!(),
    };
    let no_jump_val = match operator {
        BinaryOperatorNode::And => 1,
        BinaryOperatorNode::Or => 0,
        _ => unimplemented!(),
    };
    let dst_name = make_temporary_var();
    let dst = TValNode::Var(dst_name);
    instruction_buffer.push(TInstructionNode::Copy(
        TValNode::Constant(no_jump_val),
        dst.clone(),
    ));
    instruction_buffer.push(TInstructionNode::Jump(end_label_name.clone()));
    instruction_buffer.push(shortcircuit_label);
    instruction_buffer.push(TInstructionNode::Copy(
        TValNode::Constant(jump_val),
        dst.clone(),
    ));
    instruction_buffer.push(end_label);
    let ret = dst.clone();
    return ret;
}

fn tack_exp(
    expression: ExpressionNode,
    instruction_buffer: &mut Vec<TInstructionNode>,
) -> TValNode {
    match expression {
        ExpressionNode::Constant(c) => TValNode::Constant(c),
        ExpressionNode::Unary(operator, exp) => {
            let src = tack_exp(*exp, instruction_buffer);
            let dst_name = make_temporary_var();
            let dst = TValNode::Var(dst_name);
            let ret = dst.clone();
            instruction_buffer.push(TInstructionNode::Unary(operator, src, dst));
            return ret;
        }
        ExpressionNode::Binary(operator, op1, op2) => {
            use BinaryOperatorNode::*;
            match operator {
                Add | Subtract | Multiply | Divide | Remainder | Equal | NotEqual | GreaterThan
                | GreaterOrEqual | LessThan | LessOrEqual => {
                    handle_regular_operation(operator, op1, op2, instruction_buffer)
                }
                _ => handle_shortcircuiting_operation(operator, op1, op2, instruction_buffer),
            }
        }
        _ => unimplemented!(),
    }
}

fn tack_instructions(statement: StatementNode) -> Vec<TInstructionNode> {
    let mut instruction_buffer: Vec<TInstructionNode> = vec![];
    let StatementNode::Return(expression) = statement;
    let final_return = TInstructionNode::Return(tack_exp(expression, &mut instruction_buffer));
    instruction_buffer.push(final_return);
    return instruction_buffer;
}

fn tack_block_items(block_items: Vec<BlockItemNode>) -> Vec<TInstructionNode> {
    let mut nodes = vec![];
    for block_item in block_items {
        let mut instruction_nodes: Vec<TInstructionNode> = match block_item {
            BlockItemNode::DeclarationItem(_) => todo!("implement tacking for block"),
            BlockItemNode::StatementItem(statement) => tack_instructions(statement),
        };
        nodes.append(&mut instruction_nodes);
    }
    return nodes;
}

fn tack_functions(function: FunctionDefinitionNode) -> TFunctionDefinitionNode {
    let FunctionDefinitionNode::Function(name, block_items) = function;
    return TFunctionDefinitionNode::Function(name, tack_block_items(block_items));
}

#[tracing::instrument(skip_all)]
pub fn tack_program(program: ProgramNode) -> TProgramNode {
    let ProgramNode::Program(function) = program;
    return TProgramNode::Program(tack_functions(function));
}
