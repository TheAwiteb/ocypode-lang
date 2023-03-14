pub mod parser;
use std::{env, fmt::Display};

use miette::{GraphicalReportHandler, JSONReportHandler};

use crate::errors::{Error, ErrorKind};

/// A diagnostic struct, which is used to create a diagnostic.
#[derive(Debug)]
pub struct Diagnostic<T> {
    pub diagnostic: Box<dyn miette::Diagnostic>,
    handler: T,
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
    pub fn rgb(diagnostic: Box<dyn miette::Diagnostic>) -> Diagnostic<GraphicalReportHandler> {
        Diagnostic {
            diagnostic,
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

pub fn as_diagnostic<T: Default>(
    err: &Error,
    source: String,
    source_name: String,
) -> Diagnostic<T> {
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
        ErrorKind::Parse(ref message) => Diagnostic::new(Box::new(parser::SyntaxError {
            src: miette::NamedSource::new(source_name, source),
            message: message.clone(),
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
    }
}
