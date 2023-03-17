use miette::{Diagnostic, NamedSource};

/// Unexpected type error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::types::unexpected_type)"),
    help("Try using a `{expected}` type.")
)]
#[error("Unexpected type")]
pub struct UnexpectedType {
    pub(crate) expected: String,
    pub(crate) actual: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("Unexpected type `{actual}`")]
    pub(crate) span: miette::SourceSpan,
}
