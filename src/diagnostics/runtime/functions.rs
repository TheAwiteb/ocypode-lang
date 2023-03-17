use miette::{Diagnostic, NamedSource};

/// Missing main function error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::functions::missing_main)"),
    help("Try adding a main function.")
)]
#[error("Missing main function")]
pub struct MissingMain {
    #[source_code]
    pub(crate) src: NamedSource,
}

/// Invalid exit code error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::functions::invalid_exit_code)"),
    help("Try using a number between 0 and 255.")
)]
#[error("Invalid exit code")]
pub struct InvalidExitCode {
    #[source_code]
    pub(crate) src: NamedSource,
    pub(crate) code: bigdecimal::BigDecimal,
    #[label("Invalid exit code `{code}`")]
    pub(crate) span: miette::SourceSpan,
}

/// Uncorrect arguments error diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(
    code("Error(runtime::functions::uncorrect_arguments)"),
    help("`{func_name}` function takes {params}. But you are passed {args_count} arguments.")
)]
#[error("Uncorrect arguments")]
pub struct UncorrectArguments {
    pub(crate) func_name: String,
    pub(crate) args_count: usize,
    pub(crate) params: String,
    #[source_code]
    pub(crate) src: NamedSource,
    #[label("Function `{func_name}` is defined here")]
    pub(crate) func_span: miette::SourceSpan,
    #[label("Uncorrect arguments for function `{func_name}`")]
    pub(crate) span: miette::SourceSpan,
}
