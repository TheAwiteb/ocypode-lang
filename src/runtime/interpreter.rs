use bigdecimal::ToPrimitive;

use super::{builtins::Builtins, environment::Environment};
use crate::{
    ast::*,
    errors::{Error as OYError, ErrorKind, Result as OYResult, SpanError},
};

/// The interpreter. This will execute the AST of Ocypode and return the result. check the AST in `src/front/ast.rs`.
#[derive(Debug, Default)]
pub struct Interpreter {
    /// The current environment.
    environment: Environment,
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    /// Interprets the given program. This will return the exit code of the program.
    pub fn interpret(mut self, program: Program, argc: usize, argv: Vec<String>) -> OYResult<u8> {
        let mut exit_code = 0;
        // The program contains only functions. So we need to add them to the environment.
        for function in program.0 {
            match function {
                Statement::Function(function) => {
                    self.environment.add_global_function(function)?;
                }
                _ => unreachable!("The program only contains functions"),
            }
        }

        // Then we need to find the main function.
        if let Some(main_function) = self.environment.get_global_function("main") {
            let args = vec![
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::Int(
                    argc.to_string().parse().unwrap(),
                    Span::new(0, 0),
                ))),
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::Array(
                    argv.into_iter()
                        .map(|v| {
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::String(v, Span::new(0, 0)),
                            ))
                        })
                        .collect(),
                    Span::new(0, 0),
                ))),
            ];
            self.environment
                .new_for_function(&main_function.params, args)?;
            exit_code = match self.execute_function(main_function.clone())? {
                ObjectExpression::Int(int, span) => int
                    .to_u8()
                    .ok_or_else(|| OYError::new(ErrorKind::InvalidExitCode(int), span))?,
                _ => exit_code,
            };
            Ok(exit_code)
        } else {
            Err(OYError::new(ErrorKind::MissingMainFunction, (0, 0)))
        }
    }

    /// Executes the given function with the given arguments.
    /// This will return the result of the function. If the function does not return anything, it will return `nil`.
    ///
    /// Note: The environment should contain a fream for the function (Will removed after the function is executed)
    pub fn execute_function(&mut self, function: FunctionStatement) -> OYResult<ObjectExpression> {
        let mut result = ObjectExpression::Nil(function.span);
        if let Some(block) = function.block {
            for statement in block.statements {
                if let Some(return_value) = self.execute_statement(statement)? {
                    result = return_value;
                    break;
                }
            }
        } else {
            unreachable!("The builtin function should call in the call expression")
        }
        self.environment.exit_frame();
        Ok(result)
    }

    /// Executes the given statement.
    /// This will return the result of the value of return statement. if there is no return statement, it will return `nil`.
    pub fn execute_statement(
        &mut self,
        statement: Statement,
    ) -> OYResult<Option<ObjectExpression>> {
        match statement {
            Statement::Function(function) => {
                self.environment.add_local_function(function).map(|_| None)
            }
            Statement::Assignment(assign) => self.execute_assign(assign).map(|_| None),
            Statement::Return(return_stmt) => {
                let return_value = self.execute_expression(return_stmt.value)?;
                Ok(Some(return_value))
            }
            Statement::Expression(expr) => {
                self.execute_expression(expr)?;
                Ok(None)
            }
        }
    }

    /// Executes the given assign statement.
    /// This will execute the expr and save the varibal with the object
    pub fn execute_assign(&mut self, assign: AssignmentStatement) -> OYResult<()> {
        let object = self.execute_expression(assign.expression)?;
        let assign = AssignmentStatement {
            ident: assign.ident,
            expression: ExpressionStatement::Value(ValueExpression::Object(object)),
            span: assign.span,
        };
        self.environment.add_variable(assign)?;
        Ok(())
    }

    /// Executes the given expression.
    /// This will return the result of the expression.
    pub fn execute_expression(&mut self, expr: ExpressionStatement) -> OYResult<ObjectExpression> {
        match expr {
            ExpressionStatement::FunctionCall(func_call) => self.execute_function_call(func_call),
            ExpressionStatement::Value(value) => self.execute_value(value),
        }
    }

    /// Executes the given function call.
    /// This will return the result of the function call.
    pub fn execute_function_call(
        &mut self,
        func_call: FunctionCallExpression,
    ) -> OYResult<ObjectExpression> {
        let function = self
            .environment
            .take(&func_call.ident.ident, func_call.span)?;
        let function = match function {
            Statement::Assignment(assign) => match assign.expression {
                ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::Function(function),
                )) => function,
                _ => {
                    return Err(OYError::new(
                        ErrorKind::NotCallable(func_call.span.span()),
                        assign.span,
                    ))
                }
            },
            Statement::Function(function) => function,
            _ => {
                return Err(OYError::new(
                    ErrorKind::NotCallable(func_call.span.span()),
                    function.span(),
                ))
            }
        };
        if function.params.len() != func_call.args.len() {
            return Err(OYError::new(
                ErrorKind::UncorrectArguments(
                    func_call.args.len(),
                    function.ident.span.span(),
                    function.params,
                    function.ident.ident,
                ),
                func_call.span,
            ));
        }
        if function.block.is_none() {
            Builtins::execute_builtin_funtion(
                &function.ident.ident,
                func_call.span,
                func_call
                    .args
                    .into_iter()
                    .map(|arg| {
                        let arg_span = arg.span();
                        let mut expr = self.execute_expression(arg)?;
                        *expr.span_mut() = arg_span;
                        Ok(expr)
                    })
                    .collect::<OYResult<Vec<ObjectExpression>>>()?,
            )
        } else {
            self.environment
                .new_for_function(&function.params, func_call.args)?;
            self.execute_function(function)
        }
    }

    /// Executes the given value.
    /// This will return the result of the value.
    pub fn execute_value(&mut self, value: ValueExpression) -> OYResult<ObjectExpression> {
        match value {
            ValueExpression::Object(ObjectExpression::Array(arr, _)) => {
                let mut result = Vec::new();
                for expr in arr {
                    result.push(ExpressionStatement::Value(ValueExpression::Object(
                        self.execute_expression(expr)?,
                    )));
                }
                Ok(ObjectExpression::Array(result, Span::new(0, 0)))
            }
            ValueExpression::Object(obj) => Ok(obj),
            ValueExpression::Ident(ident) => {
                match self.environment.take(&ident.ident, ident.span)? {
                    Statement::Assignment(assign) => self.execute_expression(assign.expression),
                    Statement::Function(function) => Ok(ObjectExpression::Function(function)),
                    _ => unreachable!(),
                }
            }
        }
    }
}
