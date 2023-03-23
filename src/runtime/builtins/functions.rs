use crate::{
    ast::*,
    errors::{Error as OYError, ErrorKind as OYErrorKind, Result as OYResult},
};

/// Macro to create match expression for built in functions.
macro_rules! match_builtin {
    (call: $call_expr:expr; ident: $fn_ident: expr; args: $args:expr; $($builtin_ident:tt),+,) => {
        match $fn_ident {
            $(
                stringify!($builtin_ident) => $builtin_ident($args, $call_expr),
            )+
            _ => unreachable!()
        }
    };
}

/// Format builtin function. It takes a string as first argument and a list of arguments to format.
///
/// If the number of arguments is not the same as the number of `{}` in the string, it will return an error.
///
/// # Example
/// ```oy
/// println<format<"Hello {}"><["Awiteb"]>>;
/// // Hello Awiteb
/// ```
pub fn format(args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    // Safety: The number of arguments is checked in the interpreter before calling this function.
    let format = &args[0];
    let args = &args[1];
    let format = match format {
        ObjectExpression::String(format, _) => format,
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("String".to_owned(), format.type_name().to_owned()),
                format.span(),
            ))
        }
    };
    let args: Vec<_> = match args {
        ObjectExpression::Array(args, _) => args
            .iter()
            .map(|arg| match arg {
                ExpressionStatement::Value(ValueExpression::Object(obj)) => obj,
                _ => unreachable!(
                    "The interpreter should have checked that the arguments are objects"
                ),
            })
            .collect(),
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("Array".to_owned(), args.type_name().to_owned()),
                args.span(),
            ))
        }
    };
    let mut result = String::new();
    let mut arg_index = 0;
    let mut placeholder = false;
    let mut close_placeholder = false;
    let mut placeholder_number = String::new();

    let placeholder_count = regex::Regex::new(r"\{(\d+)?\}")
        .unwrap()
        .find_iter(format)
        .count();
    let placeholder_count_without_index = regex::Regex::new(r"\{\}")
        .unwrap()
        .find_iter(format)
        .count();

    if args.len() > placeholder_count {
        return Err(OYError::new(
            OYErrorKind::FormatError(
                format!(
                    "Too many arguments for format string, expected {}, got {}",
                    placeholder_count, args.len()
                ),
                format!("The format string has {} placeholders, remove the extra arguments or add more placeholders", placeholder_count),
            ),
            call_span,
        ));
    }
    if placeholder_count_without_index > args.len() {
        return Err(OYError::new(
            OYErrorKind::FormatError(
                    "Too many placeholders without index for format string".to_owned()
                ,
                format!("| The format string has {} placeholders without index\n| and there are only {} arguments, you can add more\n| arguments or remove the extra placeholders", 
                    placeholder_count_without_index, args.len()
                ),
            ),
            call_span,
        ));
    }

    for c in format.chars() {
        if placeholder {
            if c == '}' {
                if placeholder_number.is_empty() {
                    if arg_index < args.len() {
                        result.push_str(&args[arg_index].to_string());
                        arg_index += 1;
                    }
                } else {
                    let placeholder_number: usize = placeholder_number.parse().map_err(|_| {
                        OYError::new(
                            OYErrorKind::FormatError(
                                format!("Invalid placeholder index `{}`", placeholder_number),
                                "The placeholder index must be a number".to_owned(),
                            ),
                            call_span,
                        )
                    })?;
                    if placeholder_number < args.len() {
                        result.push_str(&args[placeholder_number].to_string());
                    } else {
                        return Err(OYError::new(
                            OYErrorKind::FormatError(
                                format!(
                                    "The placeholder index `{}` is out of range.",
                                    placeholder_number
                                ),
                                format!("The maximum index is {}", placeholder_count),
                            ),
                            call_span,
                        ));
                    }
                }
                placeholder = false;
                placeholder_number.clear();
            } else if c == '{' {
                result.push('{');
                placeholder = false;
            } else if c.is_numeric() {
                placeholder_number.push(c);
            } else {
                return Err(OYError::new(
                    OYErrorKind::FormatError(
                        format!("Invalid placeholder `{}`", c),
                        "The placeholder must contain the index or nothing".to_owned(),
                    ),
                    call_span,
                ));
            }
        } else if c == '{' {
            placeholder = true;
        } else if c == '}' && !close_placeholder {
            close_placeholder = true;
        } else if c == '}' && close_placeholder {
            result.push('}');
            close_placeholder = false;
        } else if close_placeholder {
            // Unclosed placeholder
            return Err(OYError::new(
                OYErrorKind::FormatError(
                    "Unclosed placeholder".to_owned(),
                    "The placeholder must be closed with `}`".to_owned(),
                ),
                call_span,
            ));
        } else {
            result.push(c);
        }
    }
    Ok(ObjectExpression::String(result, call_span))
}

/// Print builtin function. It takes a array of objects as argument and print them.
///
/// Note: The function will not print a newline at the end of the value. To print a newline, use `println`.
pub fn print(args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    print!("{}", print_result(args)?);
    Ok(ObjectExpression::Nil(call_span))
}

/// Print builtin function. It takes a array of objects as argument and print them with a newline.
pub fn println(args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    println!("{}", print_result(args)?);
    Ok(ObjectExpression::Nil(call_span))
}

/// Input builtin function. It takes a prompt as argument and read a line from stdin.
/// The function returns a string.
pub fn input(args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    let prompt = match &args[0] {
        ObjectExpression::String(prompt, _) => prompt,
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("String".to_owned(), args[0].type_name().to_owned()),
                args[0].span(),
            ))
        }
    };
    let input = rustyline::DefaultEditor::new()
        .map_err(|_| {
            OYError::new(
                OYErrorKind::Runtime("Failed to read a line from stdin".to_owned()),
                call_span,
            )
        })?
        .readline(prompt)
        .map_err(|_| {
            OYError::new(
                OYErrorKind::Runtime("Failed to read a line from stdin".to_owned()),
                call_span,
            )
        })?;
    Ok(ObjectExpression::String(input, call_span))
}

/// Len builtin function. It takes a array/string as argument and returns the length of the array/string.
pub fn len(args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    let len = match &args[0] {
        ObjectExpression::Array(array, _) => array.len().to_string(),
        ObjectExpression::String(string, _) => string.chars().count().to_string(),
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType(
                    "Array or String".to_owned(),
                    args[0].type_name().to_owned(),
                ),
                args[0].span(),
            ))
        }
    };
    Ok(ObjectExpression::Int(
        len.parse().map_err(|err| {
            OYError::new(
                OYErrorKind::Runtime(format!(
                    "Failed to parse the length of the array/string: {}",
                    err
                )),
                call_span,
            )
        })?,
        call_span,
    ))
}

/// Push builtin function. It takes a array and an object as argument and returns the array with the object pushed.
pub fn push(mut args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    let object = args.pop().unwrap();
    let array = args.pop().unwrap();
    let mut array: Vec<_> = match array {
        ObjectExpression::Array(array, _) => array,
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("Array".to_owned(), args[0].type_name().to_owned()),
                args[0].span(),
            ))
        }
    };
    array.push(ExpressionStatement::Value(ValueExpression::Object(object)));
    Ok(ObjectExpression::Array(array, call_span))
}

/// Pop builtin function. It takes a array as argument and returns the array with the last element popped
pub fn pop(mut args: Vec<ObjectExpression>, call_span: Span) -> OYResult<ObjectExpression> {
    let array = args.pop().unwrap();
    let mut array: Vec<_> = match array {
        ObjectExpression::Array(array, _) => array,
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("Array".to_owned(), args[0].type_name().to_owned()),
                args[0].span(),
            ))
        }
    };
    array.pop();
    Ok(ObjectExpression::Array(array, call_span))
}

/// Executes the builtin function.
pub fn execute_builtin_funtion(
    fn_ident: &str,
    call_span: Span,
    args: Vec<ObjectExpression>,
) -> OYResult<ObjectExpression> {
    match_builtin!(
        call: call_span; ident: fn_ident; args: args;
        format, print, println, input, len, push, pop,
    )
}

/// Create a new builtin function.
pub(super) fn create_builtin(name: &str, params: &[(&str, bool)]) -> FunctionStatement {
    FunctionStatement {
        ident: Some(Ident {
            ident: name.to_string(),
            span: Span::new(0, 0),
        }),
        params: params
            .iter()
            .map(|param| Param {
                ident: Ident {
                    ident: param.0.to_string(),
                    span: Span::new(0, 0),
                },
                is_pack: param.1,
            })
            .collect(),
        block: None,
        visibility: Visibility::Public,
        span: Span::new(0, 0),
    }
}

fn print_result(mut args: Vec<ObjectExpression>) -> OYResult<String> {
    let args = args
        .pop()
        .expect("The interpreter should have checked that the arguments are not empty");
    Ok(match args {
        ObjectExpression::Array(args, _) => args
            .into_iter()
            .map(|arg| match arg {
                ExpressionStatement::Value(ValueExpression::Object(obj)) => obj.to_string(),
                _ => unreachable!(
                    "The interpreter should have checked that the arguments are objects, {:?}",
                    arg
                ),
            })
            .collect(),
        _ => {
            return Err(OYError::new(
                OYErrorKind::UnexpectedType("Array".to_owned(), args.type_name().to_owned()),
                args.span(),
            ))
        }
    })
}
