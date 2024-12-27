use std::fmt::{self, Display, Error};

#[derive(Debug, Clone)]
pub enum ARegisterNode {
    AX,
    R10,
}

impl Display for ARegisterNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ARegisterNode::AX => write!(f, "eax"),
            ARegisterNode::R10 => write!(f, "r10d"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AOperandNode {
    Imm(usize),
    Reg(ARegisterNode),
    Pseudo(String),
    Stack(isize),
}

impl Display for AOperandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AOperandNode::Imm(c) => write!(f, "${c}"),
            AOperandNode::Reg(reg) => write!(f, "%{reg}"),
            AOperandNode::Stack(addr) => write!(f, "{addr}(%rbp)"),
            _ => Err(Error),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AUnaryOperatorNode {
    Neg,
    Not,
}

impl Display for AUnaryOperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AUnaryOperatorNode::Neg => write!(f, "negl"),
            AUnaryOperatorNode::Not => write!(f, "notl"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AInstructionNode {
    Mov(AOperandNode, AOperandNode),
    Unary(AUnaryOperatorNode, AOperandNode),
    AllocateStack(usize),
    Ret,
}

impl Display for AInstructionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t")?;
        match self {
            AInstructionNode::Mov(src, dst) => write!(f, "movl\t{src}, {dst}"),
            AInstructionNode::Unary(operator, operand) => write!(f, "{operator}\t{operand}"),
            AInstructionNode::AllocateStack(size) => write!(f, "subq\t$({size}), %rsp"),
            AInstructionNode::Ret => {
                write!(f, "movq\t%rbp, %rsp\n")?;
                write!(f, "\tpopq\t%rbp\n")?;
                write!(f, "\tret")
            }
        }?;
        write!(f, "\n")
    }
}

#[derive(Debug)]
pub enum AFunctionDefinitionNode {
    Function(String, Vec<AInstructionNode>),
}

#[derive(Debug)]
pub enum AProgramNode {
    Program(AFunctionDefinitionNode),
}
