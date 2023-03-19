use crate::{ast, diagnostics, parser::Rule};
use miette::{GraphicalReportHandler, JSONReportHandler};
use pest::error::InputLocation;

/// The result type with the error.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The name of the ident is invalid.
    /// - The first argument is the invalid name.
    /// - The second argument is the reason.
    /// - The third argument is the valid name.
    /// - The fourth argument is the statement type.
    InvalidName(String, String, String, String),
    /// The main function is invalid.
    /// - The first argument is the reason.
    /// - The second argument is the help message.
    InvalidMainFunction(String, String),
    /// The ident is undeclared.
    /// - The first argument is the name of the ident.
    UnDeclaredIdent(String),
    /// Alreade decleared function.
    /// - The first argument is the name of the ident.
    /// - The second argument is the span of the old declaration.
    /// (The span of the new declaration is stored in the error.)
    AlreadyDeclared(String, (usize, usize)),
    /// Missing main function.
    /// (The span of the error will not be used.)
    MissingMainFunction,
    /// The exit code is invalid.
    /// - The first argument is the exit code.
    InvalidExitCode(bigdecimal::BigDecimal),
    /// The ident is not callable.
    /// - The first argument is the span of the call
    /// (The span of the error is the not callable ident.)
    NotCallable((usize, usize)),
    /// Uncorrect arguments.
    /// - The first argument is the number of arguments.
    /// - The second argument is the span of the called function.
    /// - The third argument is the parametets of the called function.
    /// - The forth argument is the name of the called function.
    /// (The span of the error is the function call expression.)
    UncorrectArguments(usize, (usize, usize), Vec<ast::Param>, String),
    /// Unexpected type.
    /// - The first argument is the expected type.
    /// - The second argument is the actual type.
    /// (The span of the error is the expression.)
    UnexpectedType(String, String),
    /// Multiple packed params.
    /// - The first argument is the name of the function.
    /// (The span of the error is the parametets of the function.)
    MultiplePackedParams(String),
    /// Packed param is not last.
    /// - The first argument is the name of the packed param.
    /// (The span of the error is the packed param.)
    PackedParamNotLast(String),
    /// Multiple params with the same name.
    /// - The first argument is the name of the param.
    /// - The second argument is the name of the function.
    /// (The span of the error is the parametets of the function.)
    MultipleParamsWithTheSameName(String, String),
    /// Invalid unpacked argument.
    /// - The first argument is the name of the argument type.
    InvalidUnpackArg(String),
    /// Format error.
    /// - The first argument is the reason.
    /// - The second argument is the help message.
    FormatError(String, String),
    /// Runtime error. (The error is not a bug in the interpreter.)
    /// - The first argument is the reason.
    Runtime(String),
    /// The parser error.
    Parse(String),
}

/// The error type.
#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: miette::SourceSpan,
}

pub trait SpanError {
    fn span(&self) -> (usize, usize);
}

impl SpanError for ast::Span {
    fn span(&self) -> (usize, usize) {
        let (start, end) = (self.start, self.end);
        let length = end - start;
        (start, length)
    }
}

impl SpanError for pest::Span<'_> {
    fn span(&self) -> (usize, usize) {
        let (start, end) = (self.start(), self.end());
        let length = end - start;
        (start, length)
    }
}

impl SpanError for InputLocation {
    fn span(&self) -> (usize, usize) {
        match self {
            InputLocation::Pos(pos) => (*pos, 1),
            InputLocation::Span((start, end)) => (*start, end - start),
        }
    }
}

impl SpanError for (usize, usize) {
    fn span(&self) -> (usize, usize) {
        let (start, end) = self;
        let length = end - start;
        (*start, length)
    }
}

impl Error {
    /// Create a new error.
    pub fn new(kind: ErrorKind, span: impl SpanError) -> Self {
        Self {
            kind,
            span: span.span().into(),
        }
    }

    /// Returns a diagnostic for this error.
    pub fn as_diagnostic(
        self,
        source: impl AsRef<str>,
        source_name: impl AsRef<str>,
    ) -> diagnostics::Diagnostic<GraphicalReportHandler> {
        diagnostics::as_diagnostic::<GraphicalReportHandler>(
            self,
            source.as_ref().to_string(),
            source_name.as_ref().to_string(),
        )
        .rgb()
    }

    /// Returns a json diagnostic for this error.
    pub fn as_json_diagnostic(
        self,
        source: impl AsRef<str>,
        source_name: impl AsRef<str>,
    ) -> diagnostics::Diagnostic<JSONReportHandler> {
        diagnostics::as_diagnostic::<JSONReportHandler>(
            self,
            source.as_ref().to_string(),
            source_name.as_ref().to_string(),
        )
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::new(
            ErrorKind::Parse(err.variant.message().to_string()),
            err.location,
        )
    }
}
