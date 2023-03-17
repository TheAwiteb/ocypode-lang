pub mod functions;
pub mod idents;
pub mod types;
use miette::{Diagnostic, NamedSource};

/// Already declared error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::already_declared)"),
    help("Try renaming `{name}` or removing the previous declaration.")
)]
#[error("Identifier already declared")]
pub struct AlreadyDeclared {
    pub(crate) name: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("Identifier `{name}` already declared here")]
    pub(crate) old_decl: miette::SourceSpan,
    #[label("And you tried to declare it again here")]
    pub(crate) new_decl: miette::SourceSpan,
}

/// Format error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("Error(runtime::format)"), help("{help_message}"))]
#[error("Format error")]
pub struct FormatError {
    pub(crate) reason: String,
    pub(crate) help_message: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("{reason}")]
    pub(crate) span: miette::SourceSpan,
}

/// Runtime error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("Error(runtime::runtime)"))]
#[error("Runtime error")]
pub struct RuntimeError {
    pub(crate) reason: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("{reason}")]
    pub(crate) span: miette::SourceSpan,
}
