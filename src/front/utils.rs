use crate::ast::{Ident, Param, Visibility};
use crate::errors::{Error as OYError, ErrorKind as OYErrorKind, Result as OYResult};

pub enum Case {
    Snake,
}

impl Case {
    pub fn apply(&self, input: &str) -> String {
        match self {
            Case::Snake => heck::AsSnakeCase(input).to_string(),
        }
    }
}

/// Check the given identifier case.
pub fn check_ident_case(
    ident: Ident,
    ident_name: impl AsRef<str>,
    reason: impl AsRef<str>,
    case: Case,
) -> OYResult<Ident> {
    let with_case = case.apply(&ident.ident);
    if with_case == ident.ident {
        Ok(ident)
    } else {
        Err(OYError::new(
            OYErrorKind::InvalidName(
                ident.ident,
                reason.as_ref().to_owned(),
                with_case,
                ident_name.as_ref().to_owned(),
            ),
            (ident.span.start(), ident.span.end()),
        ))
    }
}

/// Check the main function name.
/// There is a special case for the main function.
/// - The name of the main function must be `main`.
/// - The main function must be private.
/// - The maximum number of parameters is 2, wich are `argc` and `argv`.
pub fn check_main_function(
    ident: &Ident,
    params: &Vec<Param>,
    visibility: &Visibility,
) -> OYResult<()> {
    if ident.ident != "main" {
        Ok(())
    } else {
        let params_spans = params.iter().map(|param| param.ident.span);
        let params_span = params_spans.fold(
            (
                ident.span.start() + ident.ident.chars().count(),
                ident.span.end(),
            ),
            |acc, span| (acc.0.min(span.start() - 1), acc.1.max(span.end() + 1)),
        );

        if visibility != &Visibility::Private {
            return Err(OYError::new(
                OYErrorKind::InvalidMainFunction(
                    "The main function must be private".to_owned(),
                    "Remove the `^` to make the main function private".to_owned(),
                ),
                (ident.span.start() - 2, ident.span.start() - 2),
            ));
        }
        if params.len() > 2 || params.is_empty() {
            return Err(OYError::new(
                OYErrorKind::InvalidMainFunction(
                    "The main function must have 2 parameters, wich are `argc` and `argv`"
                        .to_owned(),
                    if params.is_empty() {
                        "Add the parameters `argc` and `argv`\n| Example: `~main<argc><argv>`"
                            .to_owned()
                    } else {
                        format!(
                            "Remove `{}` parameter{}",
                            params
                                .iter()
                                .skip(2)
                                .map(|param| format!("<{}>", param.ident.ident))
                                .collect::<String>(),
                            if params.len() - 2 > 1 { "s" } else { "" }
                        )
                    },
                ),
                params_span,
            ));
        } else {
            // Here the params length is 1 or 2.
            let argc = &params[0];
            let argv = &params.get(1);
            if argc.ident.ident != "argc" {
                return Err(OYError::new(
                    OYErrorKind::InvalidMainFunction(
                        "The first parameter of the main function must be `argc`".to_owned(),
                        "Rename the first parameter to `argc`".to_owned(),
                    ),
                    (argc.ident.span.start(), argc.ident.span.end()),
                ));
            }
            if argv.is_some() && argv.unwrap().ident.ident != "argv" {
                let argv = argv.unwrap();
                return Err(OYError::new(
                    OYErrorKind::InvalidMainFunction(
                        "The second parameter of the main function must be `argv`".to_owned(),
                        "Rename the second parameter to `argv`".to_owned(),
                    ),
                    (argv.ident.span.start(), argv.ident.span.end()),
                ));
            } else if argv.is_none() {
                return Err(OYError::new(
                    OYErrorKind::InvalidMainFunction(
                        "The main function must have 2 parameters, wich are `argc` and `argv`"
                            .to_owned(),
                        "Add the parameter `argv`\n| Example: `~main<argc><argv>`
                        "
                        .to_owned(),
                    ),
                    (argc.ident.span.end() + 1, argc.ident.span.end() + 1),
                ));
            }
        }
        Ok(())
    }
}
