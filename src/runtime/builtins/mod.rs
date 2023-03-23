use super::environment::Environment;
use crate::{ast::*, errors::Result as OYResult};
pub mod functions;

/// The builtins functions that are available in the environment.
#[derive(Debug, Clone, Default)]
pub struct Builtins {
    pub functions: Vec<FunctionStatement>,
}

impl Builtins {
    /// Creates a new builtins, with all the builtins functions.
    pub fn new() -> Self {
        Self {
            functions: vec![
                functions::create_builtin("format", &[("format", false), ("args", true)]),
                functions::create_builtin("print", &[("values", true)]),
                functions::create_builtin("println", &[("values", true)]),
                functions::create_builtin("input", &[("prompt", false)]),
                functions::create_builtin("len", &[("value", false)]),
                functions::create_builtin("push", &[("list", false), ("value", false)]),
                functions::create_builtin("pop", &[("list", false)]),
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
}
