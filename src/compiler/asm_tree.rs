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
