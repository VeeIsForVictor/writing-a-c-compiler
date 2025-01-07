use std::fmt::{self, Display, Error};

#[derive(Debug, Clone)]
pub enum ARegisterNode {
    AX,
    DX,
    R10,
    R11,
}

impl Display for ARegisterNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ARegisterNode::AX => write!(f, "eax"),
            ARegisterNode::DX => write!(f, "edx"),
            ARegisterNode::R10 => write!(f, "r10d"),
            ARegisterNode::R11 => write!(f, "r11d"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AConditionCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
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
pub enum ABinaryOperatorNode {
    Add,
    Sub,
    Mult,
}

impl Display for ABinaryOperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABinaryOperatorNode::Add => write!(f, "addl"),
            ABinaryOperatorNode::Sub => write!(f, "subl"),
            ABinaryOperatorNode::Mult => write!(f, "imull"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AInstructionNode {
    Mov(AOperandNode, AOperandNode),
    Unary(AUnaryOperatorNode, AOperandNode),
    Binary(ABinaryOperatorNode, AOperandNode, AOperandNode),
    Cmp(AOperandNode, AOperandNode),
    Idiv(AOperandNode),
    Cdq,
    Jmp(String),
    JmpCC(AConditionCode, String),
    SetCC(AConditionCode, AOperandNode),
    Label(String),
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
            AInstructionNode::Binary(operator, src, dst) => write!(f, "{operator}\t{src}, {dst}"),
            AInstructionNode::Idiv(operand) => write!(f, "idivl\t{operand}"),
            AInstructionNode::Cdq => write!(f, "cdq"),
            AInstructionNode::Cmp(op1, op2) => write!(f, "cmpl\t{op1}, {op2}"),
            AInstructionNode::Jmp(target) => write!(f, "j\t.L{target}"),
            AInstructionNode::JmpCC(cc, target) => write!(f, "j{cc}\t.L{target}"),
            AInstructionNode::SetCC(cc, operand) => write!(f, "set{cc}\t{operand}"),
            AInstructionNode::Label(label) => write!(f, "\r   \r.L{label}"),
            _ => unimplemented!(),
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
