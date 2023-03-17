use miette::{Diagnostic, NamedSource};

/// Undeclared identifier error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::idents::undeclared_ident)"),
    help("Try declaring `{name}` before using it.")
)]
#[error("Use of undeclared identifier")]
pub struct UnDeclaredIdent {
    pub(crate) name: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("Undeclared identifier `{name}`")]
    pub(crate) span: miette::SourceSpan,
}

/// Not callable error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("Error(runtime::idents::not_callable)"))]
#[error("Calling a non-function")]
pub struct NotCallable {
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("This is not a function")]
    pub(crate) span: miette::SourceSpan,
    #[label("And you tried to call it here")]
    pub(crate) call_span: miette::SourceSpan,
}
