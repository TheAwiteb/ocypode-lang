use crate::ast::*;
use crate::errors::{Error as OYError, ErrorKind as OYErrorKind, Result as OYResult, SpanError};

use super::builtins::Builtins;

#[derive(Debug, Clone)]
pub struct Frame {
    /// The local functions that are available in the frame.
    local_functions: Vec<FunctionStatement>,
    /// The variables that are available in the frame.
    variables: Vec<AssignmentStatement>,
}

#[derive(Debug, Clone, Default)]
pub struct Environment {
    /// The global functions that are available in the environment.
    global_functions: Vec<FunctionStatement>,
    /// The frames, new frame are created when entering a function, and are removed when exiting a function.
    frames: Vec<Frame>,
}

impl Environment {
    /// Creates a new environment.
    pub fn new() -> Self {
        let mut env = Self::default();
        Builtins::new().env_init(&mut env).unwrap();
        env
    }

    /// Creates a new environment with the variables. Is used when passing arguments to a function.
    /// The arguments are added to the environment and are available in the function.
    /// The number of arguments must be equal to the number of parameter or it will panic.
    pub fn new_for_function(&mut self, params: Vec<Param>, args: Vec<Arg>) -> OYResult<()> {
        let get_ident = |expr: &ExpressionStatement| match &expr {
            ExpressionStatement::Value(ValueExpression::Ident(ident)) => Some(ident.ident.clone()),
            _ => None,
        };

        let mut variables = Vec::new();
        let mut local_functions = Vec::new();

        for (param, arg) in params.into_iter().zip(args) {
            if let Some(ident) = get_ident(&arg.expr) {
                match self.take(&ident, arg.span)? {
                    Statement::Assignment(assignment) => variables.push({
                        AssignmentStatement {
                            ident: param.ident,
                            expression: assignment.expression,
                            span: arg.span,
                        }
                    }),
                    Statement::Function(function) => local_functions.push({
                        FunctionStatement {
                            ident: Some(param.ident),
                            params: function.params,
                            block: function.block,
                            visibility: Visibility::Private,
                            span: arg.span,
                        }
                    }),
                    _ => unreachable!(),
                }
            } else {
                variables.push({
                    AssignmentStatement {
                        ident: param.ident,
                        expression: arg.expr,
                        span: arg.span,
                    }
                });
            }
        }

        self.enter_frame(local_functions, variables);
        Ok(())
    }

    /// Adds a function to the environment.
    pub fn add_global_function(&mut self, new_function: FunctionStatement) -> OYResult<()> {
        let new_function_ident = new_function.ident.clone().unwrap();
        if let Some(FunctionStatement {
            ident: Some(ident), ..
        }) = self
            .global_functions
            .iter()
            // Global functions must have an identifier.
            .find(|f| f.ident.as_ref().unwrap().ident == new_function_ident.ident)
        {
            Err(OYError::new(
                OYErrorKind::AlreadyDeclared(new_function_ident.ident, ident.span.span()),
                new_function_ident.span,
            ))
        } else {
            self.global_functions.push(new_function);
            Ok(())
        }
    }

    /// Adds a local function to the environment.
    pub fn add_local_function(&mut self, new_function: FunctionStatement) -> OYResult<()> {
        let new_function_ident = new_function.ident.clone().unwrap();
        if let Some(FunctionStatement {
            ident: Some(ident), ..
        }) = self
            .frame()
            .local_functions
            .iter()
            // Local functions must have an identifier.
            .find(|f| f.ident.as_ref().unwrap().ident == new_function_ident.ident)
        {
            Err(OYError::new(
                OYErrorKind::AlreadyDeclared(new_function_ident.ident, ident.span.span()),
                new_function_ident.span,
            ))
        } else {
            self.frame().local_functions.push(new_function);
            Ok(())
        }
    }

    /// Adds a variable to the environment.
    pub fn add_variable(&mut self, new_variable: AssignmentStatement) -> OYResult<()> {
        if let Some(old_variable) = self
            .frame()
            .variables
            .iter()
            .find(|v| v.ident.ident == new_variable.ident.ident)
        {
            Err(OYError::new(
                OYErrorKind::AlreadyDeclared(new_variable.ident.ident, old_variable.span.span()),
                new_variable.span,
            ))
        } else {
            self.frame().variables.push(new_variable);
            Ok(())
        }
    }

    /// Adds a frame to the environment.
    pub fn enter_frame(
        &mut self,
        local_functions: Vec<FunctionStatement>,
        variables: Vec<AssignmentStatement>,
    ) {
        self.frames.push(Frame {
            local_functions,
            variables,
        });
    }

    /// Returns the last frame.
    pub fn frame(&mut self) -> &mut Frame {
        self.frames
            .last_mut()
            .expect("There must be at least one frame.")
    }

    /// Exit the current frame.
    /// This is used to remove the frame from the environment.
    pub fn exit_frame(&mut self) {
        self.frames.pop();
    }

    /// Return the global function by ident
    pub fn get_global_function(&self, ident: &str) -> Option<FunctionStatement> {
        self.global_functions
            .iter()
            // Global functions must have an identifier.
            .find(|f| f.ident.as_ref().unwrap().ident == ident)
            .cloned()
    }

    /// Returns the value from the environment, and removes it from the environment.
    /// This is used when you want to move the value to another owner.
    /// If the ident is a global function, its will not be removed from the environment.
    pub fn take(&mut self, ident: &str, span: impl SpanError) -> OYResult<Statement> {
        if let Some(var) = self
            .frame()
            .variables
            .iter()
            .position(|v| v.ident.ident == ident)
        {
            Ok(Statement::Assignment(self.frame().variables.remove(var)))
        } else if let Some(local_func) = self
            .frame()
            .local_functions
            .iter()
            // Local functions must have an identifier.
            .find(|f| f.ident.as_ref().unwrap().ident == ident)
        {
            Ok(Statement::Function(local_func.clone()))
        } else if let Some(func) = self.get_global_function(ident) {
            // Not removing the global function from the environment.
            Ok(Statement::Function(func))
        } else {
            Err(OYError::new(
                OYErrorKind::UnDeclaredIdent(ident.to_owned()),
                span,
            ))
        }
    }
}
