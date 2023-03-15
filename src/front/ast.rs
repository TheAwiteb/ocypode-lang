use bigdecimal::BigDecimal;

/// The program. Is only contains a lables and functions.
pub type Program<'a> = (Vec<Statement>, pest::Span<'a>);

/// Returns the span of the given AST node.
pub trait ASTNodeSpan {
    fn span(&self) -> Span;
}

/// A span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The start of the span.
    pub start: usize,
    /// The length of the span.
    pub end: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span<'_>) -> Self {
        Self::new(span.start(), span.end())
    }
}

impl PartialEq<pest::Span<'_>> for Span {
    fn eq(&self, other: &pest::Span<'_>) -> bool {
        self.start == other.start() && self.end == other.end()
    }
}

/// A Ident.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    /// The ident name.
    pub ident: String,
    /// The span of the ident.
    pub span: Span,
}

/// A Param.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    /// The param name.
    pub ident: Ident,
}

/// A visibility, which is public or private.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    /// Public, can be used in other files.
    Public,
    /// Private, can only be used in this file.
    Private,
}

/// A Block.
/// A block is a list of statements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The statements in the block.
    pub statements: Vec<Statement>,
    /// The span of the block.
    pub span: Span,
}

/// A statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// The function statement.
    Function(FunctionStatement),
    /// The assignment statement.
    Assignment(AssignmentStatement),
    /// The return statement.
    Return(ReturnStatement),
    /// The expression statement.
    Expression(ExpressionStatement),
}

/// A Expression statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionStatement {
    /// A function call.
    FunctionCall(FunctionCallExpression),
    /// A value, which is a ident or a object.
    Value(ValueExpression),
}

/// A function statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionStatement {
    /// The function name.
    pub ident: Ident,
    /// Parameters
    pub params: Vec<Param>,
    /// The function block
    /// If the block is None, the function is builtin function.
    pub block: Option<Block>,
    /// The function visibility.
    pub visibility: Visibility,
    /// The span of the function statement.
    pub span: Span,
}

/// A assignment statement.
/// This is a assignment of a value to a ident.
/// The value is a expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentStatement {
    /// The ident to assign to.
    pub ident: Ident,
    /// The value to assign.
    pub expression: ExpressionStatement,
    /// The span of the assignment statement.
    pub span: Span,
}

/// A return statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
    /// The span of the return statement.
    pub span: Span,
    /// The value to return.
    pub value: ExpressionStatement,
}

/// A function call expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCallExpression {
    /// The function name.
    pub ident: Ident,
    /// The arguments to the function.
    pub args: Vec<ExpressionStatement>,
    /// The span of the function call expression.
    pub span: Span,
}

/// A value expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueExpression {
    /// A ident.
    Ident(Ident),
    /// A object.
    Object(ObjectExpression),
}

/// A object expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectExpression {
    /// A function
    Function(FunctionStatement),
    /// A string.
    String(String, Span),
    /// A integer number.
    Int(BigDecimal, Span),
    /// A float number.
    Float(BigDecimal, Span),
    /// A boolean.
    Bool(bool, Span),
    /// A array.
    Array(Vec<ExpressionStatement>, Span),
    /// A nil.
    Nil(Span),
}

impl ASTNodeSpan for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for Param {
    fn span(&self) -> Span {
        self.ident.span()
    }
}

impl ASTNodeSpan for Block {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for FunctionStatement {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for AssignmentStatement {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for ReturnStatement {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for FunctionCallExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl ASTNodeSpan for ValueExpression {
    fn span(&self) -> Span {
        match self {
            ValueExpression::Ident(ident) => ident.span(),
            ValueExpression::Object(object) => object.span(),
        }
    }
}

impl ASTNodeSpan for ObjectExpression {
    fn span(&self) -> Span {
        match self {
            ObjectExpression::Function(function) => function.span(),
            ObjectExpression::String(_, span) => *span,
            ObjectExpression::Int(_, span) => *span,
            ObjectExpression::Float(_, span) => *span,
            ObjectExpression::Bool(_, span) => *span,
            ObjectExpression::Array(_, span) => *span,
            ObjectExpression::Nil(span) => *span,
        }
    }
}

impl ASTNodeSpan for ExpressionStatement {
    fn span(&self) -> Span {
        match self {
            ExpressionStatement::FunctionCall(function_call) => function_call.span(),
            ExpressionStatement::Value(value) => value.span(),
        }
    }
}

impl ASTNodeSpan for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Function(function) => function.span(),
            Statement::Assignment(assignment) => assignment.span(),
            Statement::Return(return_statement) => return_statement.span(),
            Statement::Expression(expression) => expression.span(),
        }
    }
}
