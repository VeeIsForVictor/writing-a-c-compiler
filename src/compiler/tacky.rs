use std::sync::Mutex;

use tracing::error;

use super::parser::{ExpressionNode, FunctionDefinitionNode, ProgramNode, StatementNode};

static TEMPORARY_COUNTER: Mutex<usize> = Mutex::new(0);

#[derive(Debug)]
pub enum TUnitaryOperatorNode {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum TValNode {
    Constant(usize),
    Var(String),
}

#[derive(Debug)]
pub enum TInstructionNode {
    Mov(TValNode, TValNode),
    Ret,
}

#[derive(Debug)]
pub enum TFunctionDefinitionNode {
    Function(String, Vec<TInstructionNode>),
}

#[derive(Debug)]
pub enum TProgramNode {
    Program(TFunctionDefinitionNode),
}

fn make_temporary_var() -> String {
    match TEMPORARY_COUNTER.lock() {
        Ok(mut counter) => {
            let temp = *counter;
            *counter += 1;
            format!("tmp.{temp}")
        }
        Err(e) => {
            error!("temporary variable counter mutex was poisoned: {e:?}");
            panic!("Concurrency panic!");
        }
    }
}

fn tack_exp(expression: ExpressionNode) -> TValNode {
    match expression {
        ExpressionNode::Constant(c) => TValNode::Constant(c),
        ExpressionNode::Unary(operator, exp) => {
            let src = tack_exp(*exp);
            let dst_name = make_temporary_var();
        }
    }
}

fn tack_instructions(statement: StatementNode) -> Vec<TInstructionNode> {
    let StatementNode::Return(expression) = statement;
    return vec![];
}

fn tack_functions(function: FunctionDefinitionNode) -> TFunctionDefinitionNode {
    let FunctionDefinitionNode::Function(name, statement) = function;
    return TFunctionDefinitionNode::Function(name, tack_instructions(statement));
}

pub fn tack_program(program: ProgramNode) -> TProgramNode {
    let ProgramNode::Program(function) = program;
    return TProgramNode::Program(tack_functions(function));
}
