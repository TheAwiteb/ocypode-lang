pub mod parser;
pub mod runtime;
use crate::errors::{Error, ErrorKind};
use miette::{GraphicalReportHandler, JSONReportHandler};
use std::{env, fmt::Display};

/// A diagnostic struct, which is used to create a diagnostic.
#[derive(Debug)]
pub struct Diagnostic<T> {
    pub diagnostic: Box<dyn miette::Diagnostic>,
    pub handler: T,
}

impl<T: Default> Diagnostic<T> {
    /// Create a new diagnostic.
    pub fn new(diagnostic: Box<dyn miette::Diagnostic>) -> Self {
        Self {
            diagnostic,
            handler: T::default(),
        }
    }

    /// Create a RGB diagnostic.
    pub fn rgb(self) -> Diagnostic<GraphicalReportHandler> {
        Diagnostic {
            diagnostic: self.diagnostic,
            handler: GraphicalReportHandler::default()
                .with_context_lines(3)
                .with_theme(miette::GraphicalTheme {
                    characters: miette::ThemeCharacters::emoji(),
                    styles: matches!(env::var("NO_COLOR").as_deref(), Ok("1") | Ok("true"))
                        .then(miette::ThemeStyles::none)
                        .unwrap_or_else(miette::ThemeStyles::rgb),
                }),
        }
    }
}

impl Diagnostic<GraphicalReportHandler> {
    /// Display the diagnostic as a ASCII string
    pub fn as_ascii(mut self) -> Self {
        self.handler = self.handler.with_theme(miette::GraphicalTheme {
            characters: miette::ThemeCharacters::ascii(),
            styles: miette::ThemeStyles::ansi(),
        });
        self
    }

    /// Add a lines of context to the diagnostic. (the lines before and after the diagnostic)
    pub fn with_context_lines(mut self, lines: usize) -> Self {
        self.handler = self.handler.with_context_lines(lines);
        self
    }
}

impl Display for Diagnostic<GraphicalReportHandler> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.handler.render_report(f, self.diagnostic.as_ref())
    }
}

impl Display for Diagnostic<JSONReportHandler> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.handler.render_report(f, self.diagnostic.as_ref())
    }
}

pub fn as_diagnostic<T: Default>(err: Error, source: String, source_name: String) -> Diagnostic<T> {
    match err.kind {
        ErrorKind::InvalidName(ref name, ref reason, ref valid_name, ref statement_type) => {
            Diagnostic::new(Box::new(parser::definitions::InvalidName {
                src: miette::NamedSource::new(source_name, source),
                name: name.clone(),
                statement_type: statement_type.clone(),
                reason: reason.clone(),
                valid_name: valid_name.clone(),
                span: err.span,
            }))
        }
        ErrorKind::Parse(message) => Diagnostic::new(Box::new(parser::SyntaxError {
            src: miette::NamedSource::new(source_name, source),
            message,
            span: err.span,
        })),
        ErrorKind::InvalidMainFunction(ref reason, ref help) => {
            Diagnostic::new(Box::new(parser::definitions::InvalidMainFunction {
                src: miette::NamedSource::new(source_name, source),
                reason: reason.clone(),
                help: help.clone(),
                span: err.span,
            }))
        }
        ErrorKind::UnDeclaredIdent(name) => {
            Diagnostic::new(Box::new(runtime::idents::UnDeclaredIdent {
                src: miette::NamedSource::new(source_name, source),
                name,
                span: err.span,
            }))
        }
        ErrorKind::AlreadyDeclared(name, old_decl) => {
            Diagnostic::new(Box::new(runtime::AlreadyDeclared {
                src: miette::NamedSource::new(source_name, source),
                name,
                old_decl: old_decl.into(),
                new_decl: err.span,
            }))
        }
        ErrorKind::MissingMainFunction => {
            Diagnostic::new(Box::new(runtime::functions::MissingMain {
                src: miette::NamedSource::new(source_name, source),
            }))
        }
        ErrorKind::InvalidExitCode(exit_code) => {
            Diagnostic::new(Box::new(runtime::functions::InvalidExitCode {
                src: miette::NamedSource::new(source_name, source),
                code: exit_code,
                span: err.span,
            }))
        }
        ErrorKind::NotCallable(call_span) => {
            Diagnostic::new(Box::new(runtime::idents::NotCallable {
                src: miette::NamedSource::new(source_name, source),
                call_span: call_span.into(),
                span: err.span,
            }))
        }
        ErrorKind::UncorrectArguments(args_count, func_span, params, func_name) => {
            Diagnostic::new(Box::new(runtime::functions::UncorrectArguments {
                src: miette::NamedSource::new(source_name, source),
                args_count,
                params: if !params.is_empty() {
                    format!(
                        "{} arguments, which are `{}`",
                        params.len(),
                        params
                            .into_iter()
                            .map(|p| p.ident.ident)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    "no arguments".to_owned()
                },
                func_span: func_span.into(),
                func_name,
                span: err.span,
            }))
        }
        ErrorKind::UnexpectedType(expected, actual) => {
            Diagnostic::new(Box::new(runtime::types::UnexpectedType {
                src: miette::NamedSource::new(source_name, source),
                expected,
                actual,
                span: err.span,
            }))
        }
        ErrorKind::MultiplePackedParams(func_name) => {
            Diagnostic::new(Box::new(parser::params::MultiplePackedParams {
                src: miette::NamedSource::new(source_name, source),
                func_name,
                span: err.span,
            }))
        }
        ErrorKind::PackedParamNotLast(param_name) => {
            Diagnostic::new(Box::new(parser::params::PackedParamNotLast {
                src: miette::NamedSource::new(source_name, source),
                param_name,
                span: err.span,
            }))
        }
        ErrorKind::MultipleParamsWithTheSameName(param_name, func_name) => {
            Diagnostic::new(Box::new(parser::params::MultipleParamsWithTheSameName {
                src: miette::NamedSource::new(source_name, source),
                param_name,
                func_name,
                span: err.span,
            }))
        }
        ErrorKind::InvalidUnpackArg(type_name) => {
            Diagnostic::new(Box::new(runtime::InvalidUnpackedArgument {
                src: miette::NamedSource::new(source_name, source),
                type_name,
                span: err.span,
            }))
        }
        ErrorKind::FormatError(reason, help_message) => {
            Diagnostic::new(Box::new(runtime::FormatError {
                src: miette::NamedSource::new(source_name, source),
                reason,
                help_message,
                span: err.span,
            }))
        }
        ErrorKind::Runtime(reason) => Diagnostic::new(Box::new(runtime::RuntimeError {
            src: miette::NamedSource::new(source_name, source),
            reason,
            span: err.span,
        })),
    }
}
