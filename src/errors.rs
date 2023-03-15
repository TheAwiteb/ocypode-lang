use miette::{GraphicalReportHandler, JSONReportHandler};
use pest::error::InputLocation;
use crate::{diagnostics, parser::Rule};

/// The result type with the error.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type.
#[derive(Debug)]
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
    Parse(String),
}

/// The error type.
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: miette::SourceSpan,
}

pub trait SpanError {
    fn span(&self) -> (usize, usize);
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
        &self,
        source: impl AsRef<str>,
        source_name: impl AsRef<str>,
    ) -> diagnostics::Diagnostic<GraphicalReportHandler> {
        diagnostics::Diagnostic::<GraphicalReportHandler>::rgb(
            diagnostics::as_diagnostic::<GraphicalReportHandler>(
                self,
                source.as_ref().to_string(),
                source_name.as_ref().to_string(),
            )
            .diagnostic,
        )
    }

    /// Returns a json diagnostic for this error.
    pub fn as_json_diagnostic(
        &self,
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
