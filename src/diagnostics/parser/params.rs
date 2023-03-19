use miette::{Diagnostic, NamedSource};

/// Multiple packed parameters diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("parser::params::multiple_packed"))]
#[error("Multiple packed parameters")]
pub struct MultiplePackedParams {
    /// The source code.
    #[source_code]
    pub src: NamedSource,
    /// The name of the function.
    pub func_name: String,
    /// The span of the syntax error.
    #[label("multiple packed parameters in function `{func_name}`")]
    pub span: miette::SourceSpan,
}

/// Packed parameter is not last diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("parser::params::packed_not_last"))]
#[error("Packed parameter is not last")]
pub struct PackedParamNotLast {
    /// The source code.
    #[source_code]
    pub src: NamedSource,
    /// The name of the packed parameter.
    pub param_name: String,
    /// The span of the syntax error.
    #[label("packed parameter `{param_name}` is not the last parameter")]
    pub span: miette::SourceSpan,
}

/// Multiple parameters with the same name diagnostic.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code("parser::params::multiple_same_name"))]
#[error("Multiple parameters with the same name")]
pub struct MultipleParamsWithTheSameName {
    /// The source code.
    #[source_code]
    pub src: NamedSource,
    /// The name of the parameter.
    pub param_name: String,
    /// The name of the function.
    pub func_name: String,
    /// The span of the syntax error.
    #[label("multiple parameters with the same name `{param_name}` in function `{func_name}`")]
    pub span: miette::SourceSpan,
}
