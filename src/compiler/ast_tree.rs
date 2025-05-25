#[derive(Debug)]
pub enum UnaryOperatorNode {
    Complement,
    Negate,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperatorNode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Constant(usize),
    Var(String),
    Unary(UnaryOperatorNode, Box<ExpressionNode>),
    Binary(BinaryOperatorNode, Box<ExpressionNode>, Box<ExpressionNode>),
    Assignment(Box<ExpressionNode>, Box<ExpressionNode>)
}

#[derive(Debug)]
pub enum StatementNode {
    Return(ExpressionNode),
    Expression(ExpressionNode),
    Null,
}

#[derive(Debug)]
pub enum DeclarationNode {
    Declaration(String, Option<ExpressionNode>),
}

#[derive(Debug)]
pub enum FunctionDefinitionNode {
    Function(String, StatementNode),
}

#[derive(Debug)]
pub enum ProgramNode {
    Program(FunctionDefinitionNode),
}
