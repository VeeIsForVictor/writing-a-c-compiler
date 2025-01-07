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
fn make_label_name(op: &BinaryOperatorNode) -> String {
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
            format!("{label_name}.{temp}")
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
    let conclude_label = TInstructionNode::Label(make_label_name(&operator));
    let jump_op = match operator {
        BinaryOperatorNode::And => TInstructionNode::JumpIfZero,
        BinaryOperatorNode::Or => TInstructionNode::JumpIfNotZero,
        _ => unimplemented!(),
    };
    let v1 = tack_exp(*op1, instruction_buffer);
    let v2 = tack_exp(*op2, instruction_buffer);
    let dst_name = make_temporary_var();
    let dst = TValNode::Var(dst_name);
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

fn tack_functions(function: FunctionDefinitionNode) -> TFunctionDefinitionNode {
    let FunctionDefinitionNode::Function(name, statement) = function;
    return TFunctionDefinitionNode::Function(name, tack_instructions(statement));
}

#[tracing::instrument(skip_all)]
pub fn tack_program(program: ProgramNode) -> TProgramNode {
    let ProgramNode::Program(function) = program;
    return TProgramNode::Program(tack_functions(function));
}
