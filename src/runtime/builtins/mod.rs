use super::environment::Environment;
use crate::{ast::*, errors::Result as OYResult};
pub mod functions;

/// The builtins functions that are available in the environment.
#[derive(Debug, Clone, Default)]
pub struct Builtins {
    pub functions: Vec<FunctionStatement>,
}

/// Macro to create match expression for built in functions.
macro_rules! match_builtin {
    (call: $call_expr:expr; ident: $fn_ident: expr; args: $args:expr; $($builtin_ident:tt),+,) => {
        match $fn_ident {
            $(
                stringify!($builtin_ident) => functions::$builtin_ident($args, $call_expr),
            )+
            _ => unreachable!()
        }
    };
}

impl Builtins {
    /// Creates a new builtins, with all the builtins functions.
    pub fn new() -> Self {
        Self {
            functions: vec![
                create_builtin("format", &[("format", false), ("args", true)]),
                create_builtin("print", &[("values", true)]),
                create_builtin("println", &[("values", true)]),
                create_builtin("input", &[("prompt", false)]),
                create_builtin("len", &[("value", false)]),
                create_builtin("push", &[("list", false), ("value", false)]),
                create_builtin("pop", &[("list", false)]),
            ],
        }
    }

    /// Initializes the environment with the builtins functions.
    pub fn env_init(self, env: &mut Environment) -> OYResult<()> {
        for function in self.functions {
            env.add_global_function(function)?;
        }
        Ok(())
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
}

fn create_builtin(name: &str, params: &[(&str, bool)]) -> FunctionStatement {
    FunctionStatement {
        ident: Ident {
            ident: name.to_string(),
            span: Span::new(0, 0),
        },
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
