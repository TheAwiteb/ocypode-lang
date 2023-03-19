use crate::ast::*;
use crate::errors::{Error as OYError, ErrorKind as OYErrorKind, Result as OYResult};
use crate::runtime::interpreter::Interpreter;

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
            ident.span,
        ))
    }
}

/// Check if the given parameters are valid.
/// This will check if there are multiple parameters with the same name.
/// This will also check if there are multiple packed parameters.
/// This will also check if the packed parameter is not the last parameter.
pub fn cheeck_params(params: Vec<Param>, func_ident: &Ident) -> OYResult<Vec<Param>> {
    let params_span = span_of_spans(
        &params.iter().map(|p| p.span()).collect::<Vec<_>>(),
        Span::new(func_ident.span.end, func_ident.span.end),
    );
    let packs_count = params.iter().filter(|p| p.is_pack).count();

    // Check if there are multiple parameters with the same name.
    for param in &params {
        if params
            .iter()
            .filter(|p| p.ident.ident == param.ident.ident)
            .count()
            > 1
        {
            return Err(OYError::new(
                OYErrorKind::MultipleParamsWithTheSameName(
                    param.ident.ident.clone(),
                    func_ident.ident.clone(),
                ),
                params_span,
            ));
        }
    }
    // Check if there are multiple packed parameters.
    if packs_count > 1 {
        return Err(OYError::new(
            OYErrorKind::MultiplePackedParams(func_ident.ident.clone()),
            params_span,
        ));
    }
    // Check if the packed parameter is not the last parameter.
    if packs_count == 1 && !params.last().unwrap().is_pack {
        let packed_param = params.iter().find(|p| p.is_pack).unwrap();
        return Err(OYError::new(
            OYErrorKind::PackedParamNotLast(packed_param.ident.ident.clone()),
            packed_param.ident.span,
        ));
    }

    Ok(params)
}

/// Returns the span of the given spans.
pub fn span_of_spans(spans: &[Span], default_span: Span) -> (usize, usize) {
    let last_param = spans
        .last()
        .map(|s| s.end + 1)
        .unwrap_or(default_span.start);
    let first_param = spans
        .first()
        .map(|s| s.start - 1)
        .unwrap_or(default_span.end);
    (first_param, last_param)
}

/// Unpack the given arguments.
pub fn unpack_args(interpreter: &mut Interpreter, call_args: Vec<Arg>) -> OYResult<Vec<Arg>> {
    let mut args = Vec::new();
    for arg in call_args {
        if arg.is_unpack {
            let obj = interpreter.execute_expression(arg.expr)?;
            let unpacked_args = match obj {
                ObjectExpression::Array(array, _) => array
                    .into_iter()
                    .map(|expr| {
                        let span = expr.span();
                        Ok(Arg {
                            expr,
                            is_unpack: false,
                            span,
                        })
                    })
                    .collect::<OYResult<Vec<Arg>>>()?,
                _ => {
                    return Err(OYError::new(
                        OYErrorKind::InvalidUnpackArg(obj.type_name().to_owned()),
                        arg.span,
                    ))
                }
            };
            args.extend(unpacked_args);
        } else {
            args.push(arg);
        }
    }
    Ok(args)
}

/// Pack the rest of the given arguments.
pub fn pack_args(
    interpreter: &mut Interpreter,
    function: &FunctionStatement,
    call_args: Vec<Arg>,
) -> OYResult<Vec<Arg>> {
    // Collect the arguments.
    let mut args = call_args
        .iter()
        .enumerate()
        .take_while(|(idx, _)| *idx < function.params.len() - 1)
        .map(|(_, arg)| arg.clone())
        .collect::<Vec<_>>();
    // Collect the rest of the arguments.
    let rest_args = call_args
        .iter()
        .skip(function.params.len() - 1)
        .cloned()
        .collect::<Vec<_>>();
    // The default span will not be used, because the rest arguments are not empty.
    let rests_span = span_of_spans(
        &rest_args.iter().map(|a| a.span).collect::<Vec<_>>(),
        Span::new(0, 0),
    );
    // Pack the rest of the arguments.
    args.push(Arg {
        expr: ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::Array(
            rest_args
                .into_iter()
                .map(|arg| {
                    Ok(ExpressionStatement::Value(ValueExpression::Object(
                        interpreter.execute_expression(arg.expr)?,
                    )))
                })
                .collect::<OYResult<_>>()?,
            Span::new(rests_span.0, rests_span.1),
        ))),
        is_unpack: false,
        span: Span::new(0, 0),
    });
    Ok(args)
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
        let params_span = span_of_spans(
            &params.iter().map(|p| p.span()).collect::<Vec<_>>(),
            Span::new(ident.span.end, ident.span.end),
        );

        if visibility != &Visibility::Private {
            return Err(OYError::new(
                OYErrorKind::InvalidMainFunction(
                    "The main function must be private".to_owned(),
                    "Remove the `^` to make the main function private".to_owned(),
                ),
                (ident.span.start - 2, ident.span.start - 2),
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
                    argc.ident.span,
                ));
            }
            if argv.is_some() && argv.unwrap().ident.ident != "argv" {
                let argv = argv.unwrap();
                return Err(OYError::new(
                    OYErrorKind::InvalidMainFunction(
                        "The second parameter of the main function must be `argv`".to_owned(),
                        "Rename the second parameter to `argv`".to_owned(),
                    ),
                    argv.ident.span,
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
                    (argc.ident.span.end + 1, argc.ident.span.end + 1),
                ));
            }
        }
        Ok(())
    }
}
