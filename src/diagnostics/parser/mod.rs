use miette::{Diagnostic, NamedSource};

pub mod definitions;
pub mod params;

/// Syntax error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("parser::syntax_error"))]
#[error("Syntax error")]
pub struct SyntaxError {
    /// The source code.
    #[source_code]
    pub src: NamedSource,
    /// The message.
    pub message: String,
    /// The span of the syntax error.
    #[label("{message}")]
    pub span: miette::SourceSpan,
}
