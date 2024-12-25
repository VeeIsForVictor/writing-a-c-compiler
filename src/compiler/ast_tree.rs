#[derive(Debug)]
pub enum UnaryOperatorNode {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Constant(usize),
    Unary(UnaryOperatorNode, Box<ExpressionNode>),
}

#[derive(Debug)]
pub enum StatementNode {
    Return(ExpressionNode),
}

#[derive(Debug)]
pub enum FunctionDefinitionNode {
    Function(String, StatementNode),
}

#[derive(Debug)]
pub enum ProgramNode {
    Program(FunctionDefinitionNode),
}
