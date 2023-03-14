use bigdecimal::BigDecimal;
use pest::Span;

/// The program. Is only contains a lables and functions.
pub type Program<'a> = (Vec<Statement<'a>>, Span<'a>);

/// A Ident.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a> {
    /// The ident name.
    pub ident: String,
    /// The span of the ident.
    pub span: Span<'a>,
}

/// A Param.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param<'a> {
    /// The param name.
    pub ident: Ident<'a>,
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
pub struct Block<'a> {
    /// The statements in the block.
    pub statements: Vec<Statement<'a>>,
    /// The span of the block.
    pub span: Span<'a>,
}

/// A statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    /// The function statement.
    Function(FunctionStatement<'a>),
    /// The assignment statement.
    Assignment(AssignmentStatement<'a>),
    /// The return statement.
    Return(ReturnStatement<'a>),
    /// The expression statement.
    Expression(ExpressionStatement<'a>),
}

/// A Expression statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionStatement<'a> {
    /// A function call.
    FunctionCall(FunctionCallExpression<'a>),
    /// A value, which is a ident or a object.
    Value(ValueExpression<'a>),
}

/// A function statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionStatement<'a> {
    /// The function name.
    pub ident: Ident<'a>,
    /// Parameters
    pub params: Vec<Param<'a>>,
    /// The function block
    pub block: Block<'a>,
    /// The function visibility.
    pub visibility: Visibility,
    /// The span of the function statement.
    pub span: Span<'a>,
}

/// A assignment statement.
/// This is a assignment of a value to a ident.
/// The value is a expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentStatement<'a> {
    /// The ident to assign to.
    pub ident: Ident<'a>,
    /// The value to assign.
    pub expression: ExpressionStatement<'a>,
    /// The span of the assignment statement.
    pub span: Span<'a>,
}

/// A return statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<'a> {
    /// The span of the return statement.
    pub span: Span<'a>,
    /// The value to return.
    pub value: ExpressionStatement<'a>,
}

/// A function call expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCallExpression<'a> {
    /// The function name.
    pub ident: Ident<'a>,
    /// The arguments to the function.
    pub args: Vec<ExpressionStatement<'a>>,
    /// The span of the function call expression.
    pub span: Span<'a>,
}

/// A value expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueExpression<'a> {
    /// A ident.
    Ident(Ident<'a>),
    /// A object.
    Object(ObjectExpression<'a>),
}

/// A object expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectExpression<'a> {
    /// A string.
    String(String, Span<'a>),
    /// A integer number.
    Int(BigDecimal, Span<'a>),
    /// A float number.
    Float(BigDecimal, Span<'a>),
    /// A boolean.
    Bool(bool, Span<'a>),
    /// A array.
    Array(Vec<ExpressionStatement<'a>>, Span<'a>),
    /// A nil.
    Nil(Span<'a>),
}
