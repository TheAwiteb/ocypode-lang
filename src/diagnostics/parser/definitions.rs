use miette::{Diagnostic, NamedSource};

/// Invalid name error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(parser::definitions::invalid_name)"),
    help("{reason}. Try `{valid_name}` instead.")
)]
#[error("invalid {statement_type} name: `{name}`")]
pub struct InvalidName {
    #[source_code]
    pub(crate) src: NamedSource,
    pub(crate) name: String,
    pub(crate) statement_type: String,
    pub(crate) reason: String,
    pub(crate) valid_name: String,
    #[label("`{name}` is invalid name for {statement_type}")]
    pub(crate) span: miette::SourceSpan,
}

/// Invalid main function error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(parser::definitions::invalid_main_function)"),
    help("{help}")
)]
#[error("invalid main function")]
pub struct InvalidMainFunction {
    #[source_code]
    pub(crate) src: NamedSource,
    pub(crate) reason: String,
    pub(crate) help: String,
    #[label("{reason}")]
    pub(crate) span: miette::SourceSpan,
}
