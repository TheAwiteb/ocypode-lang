use std::str::FromStr;

use crate::{
    ast::*,
    errors::{Error as OYError, Result as OYResult},
    utils::{self, check_ident_case},
};

use bigdecimal::BigDecimal;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

/// The parser for the Ocypode Language. This is generated by the pest_derive crate from the grammar in `grammar.pest`.
#[derive(Parser, Debug)]
#[grammar = "grammar.pest"]
pub struct OYParser;

impl<'a> OYParser {
    /// Parse the given source code to a program.
    pub fn parse_program(input: &'a str) -> OYResult<Program> {
        let program_pair = OYParser::parse(Rule::program, input)
            .map_err(OYError::from)?
            .next()
            .unwrap();
        let span = program_pair.as_span();
        Ok((
            program_pair
                .into_inner()
                .filter(|pair| pair.as_rule() != Rule::EOI)
                .map(Self::parse_statement)
                .collect::<OYResult<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
            span,
        ))
    }

    /// Parse the given source code to a statement.
    pub fn parse_statement(statement: Pair<'a, Rule>) -> OYResult<Option<Statement>> {
        match statement.as_rule() {
            Rule::func_def => Ok(Some(Statement::Function(Self::parse_function(statement)?))),
            Rule::class_def => Ok(Some(Statement::Class(Self::parse_class(statement)?))),
            Rule::assignment => Ok(Some(Self::parse_assignment(statement)?)),
            Rule::return_stmt => Ok(Some(Self::parse_return(statement)?)),
            Rule::expression => Ok(Some(Statement::Expression(Self::parse_expression(
                statement,
            )?))),
            Rule::statement => Self::parse_statement(statement.into_inner().next().unwrap()),
            Rule::semicolon => Ok(None),
            _ => {
                dbg!(statement.as_rule());
                unreachable!("This function only parse the statements")
            }
        }
    }

    /// Parse the given source code to a block.
    pub fn parse_block(block: Pair<'a, Rule>) -> OYResult<Block> {
        let span = block.as_span();
        Ok(Block {
            statements: block
                .into_inner()
                .map(Self::parse_statement)
                .collect::<OYResult<Vec<Option<_>>>>()?
                .into_iter()
                .flatten()
                .collect(),
            span: span.into(),
        })
    }

    /// Parse the given source code to a identifier.
    /// Make sure that the given pair is a identifier, otherwise this will panic.
    pub fn parse_ident(ident: Pair<'a, Rule>) -> Ident {
        let span = ident.as_span();
        let ident = ident.as_str().to_owned();
        Ident {
            ident: ident.as_str().to_owned(),
            span: span.into(),
        }
    }

    /// Parse the visibility.
    /// Make sure that the given pair is a visibility, otherwise this will panic.
    pub fn parse_visibility(visibility: Pair<'a, Rule>) -> Visibility {
        match visibility.as_rule() {
            Rule::PUBLIC => Visibility::Public,
            Rule::PRIVATE => Visibility::Private,
            Rule::visibility => Self::parse_visibility(visibility.into_inner().next().unwrap()),
            _ => {
                dbg!(visibility.as_rule());
                unreachable!("This function only parse the visibility")
            }
        }
    }

    /// Parse the given source code to a expression.
    pub fn parse_expression(expr: Pair<'a, Rule>) -> OYResult<ExpressionStatement> {
        match expr.as_rule() {
            Rule::func_call => Ok(ExpressionStatement::FunctionCall(
                Self::parse_function_call(expr)?,
            )),
            Rule::value => Ok(ExpressionStatement::Value(Self::parse_value(expr)?)),
            Rule::expression => Self::parse_expression(expr.into_inner().next().unwrap()),
            Rule::anonymous_function => {
                let anonymous_function = Self::parse_anonymous_function(expr)?;
                Ok(ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::Function(anonymous_function),
                )))
            }
            _ => {
                dbg!(expr.as_rule());
                unreachable!("This function only parse the expressions")
            }
        }
    }

    /// Parse the given source code to a function call expression.
    /// Make sure that the given pair is a function call expression, otherwise this will panic.
    pub fn parse_function_call(func: Pair<'a, Rule>) -> OYResult<FunctionCallExpression> {
        let span = func.as_span();
        let mut inner = func.into_inner();
        let callable = inner.next().unwrap();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(Self::parse_arg)
            .collect::<OYResult<Vec<_>>>()?;
        match callable.as_rule() {
            Rule::IDENT => Ok(FunctionCallExpression {
                callable: ValueExpression::Ident(Self::parse_ident(callable)),
                args,
                span: span.into(),
            }),
            Rule::anonymous_function => {
                let anonymous_function = Self::parse_anonymous_function(callable)?;

                Ok(FunctionCallExpression {
                    callable: ValueExpression::Object(ObjectExpression::Function(
                        anonymous_function,
                    )),
                    args,
                    span: span.into(),
                })
            }
            _ => {
                dbg!(callable.as_rule());
                unreachable!("This function only parse the function call expressions")
            }
        }
    }

    /// Parse the given source code to a return statement.
    /// Make sure that the given pair is a return statement, otherwise this will panic.
    pub fn parse_return(return_stmt: Pair<'a, Rule>) -> OYResult<Statement> {
        let span = return_stmt.as_span();
        let mut inner = return_stmt.into_inner();
        Ok(Statement::Return(ReturnStatement {
            value: Self::parse_expression(inner.next().unwrap())?,
            span: span.into(),
        }))
    }

    /// Parse the given source code to a value expression.
    /// Make sure that the given pair is a value expression, otherwise this will panic.
    pub fn parse_value(value: Pair<'a, Rule>) -> OYResult<ValueExpression> {
        let span = value.as_span();
        Ok(match value.as_rule() {
            Rule::IDENT => ValueExpression::Ident(Self::parse_ident(value)),
            Rule::string => ValueExpression::Object(ObjectExpression::String(
                value.as_str()[1..value.as_str().len() - 1]
                    .replace(r#"\\"#, r#"\"#)
                    .replace(r#"\""#, r#"""#)
                    .replace(r#"\n"#, "\n")
                    .replace(r#"\t"#, "\t")
                    .replace(r#"\r"#, "\r"),
                span.into(),
            )),
            Rule::float => ValueExpression::Object(ObjectExpression::Float(
                BigDecimal::from_str(value.as_str()).unwrap(),
                span.into(),
            )),
            Rule::integer => ValueExpression::Object(ObjectExpression::Int(
                BigDecimal::from_str(value.as_str()).unwrap(),
                span.into(),
            )),
            Rule::boolean => ValueExpression::Object(ObjectExpression::Bool(
                value.as_str().parse().unwrap(),
                span.into(),
            )),
            Rule::array => ValueExpression::Object(ObjectExpression::Array(
                value
                    .into_inner()
                    .map(Self::parse_expression)
                    .collect::<OYResult<_>>()?,
                span.into(),
            )),
            Rule::nil => ValueExpression::Object(ObjectExpression::Nil(span.into())),
            Rule::value => Self::parse_value(value.into_inner().next().unwrap())?,
            _ => {
                dbg!(value.as_rule());
                unreachable!("This function only parse the values")
            }
        })
    }

    /// Parse the given source code to an anonymous function.
    /// Make sure that the given pair is an anonymous function, otherwise this will panic.
    pub fn parse_anonymous_function(anonymous_func: Pair<'a, Rule>) -> OYResult<FunctionStatement> {
        let span = anonymous_func.as_span();
        let mut anonymous_inner = anonymous_func.into_inner();
        let params = anonymous_inner
            .next()
            .unwrap()
            .into_inner()
            .map(Self::parse_param)
            .collect::<OYResult<_>>()?;
        let block = Self::parse_block(anonymous_inner.next().unwrap())?;
        Ok(FunctionStatement {
            ident: None,
            params,
            block: Some(block),
            span: span.into(),
            visibility: Visibility::Private,
        })
    }

    /// Parse the given source code to a function statement.
    /// Make sure that the given pair is a function statement, otherwise this will panic.
    pub fn parse_function(func: Pair<'a, Rule>) -> OYResult<FunctionStatement> {
        let span = func.as_span();
        let mut inner = func.into_inner();
        let visibility = Self::parse_visibility(inner.next().unwrap());
        let ident = utils::check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "function",
            "the function name must be snake_case",
            utils::Case::Snake,
        )?;
        let params = inner
            .next()
            .unwrap()
            .into_inner()
            .map(Self::parse_param)
            .collect::<OYResult<Vec<_>>>()?;
        let params = utils::cheeck_params(params, &ident)?;
        let block = Some(Self::parse_block(inner.next().unwrap())?);
        utils::check_main_function(&ident, &params, &visibility)?;
        Ok(FunctionStatement {
            ident: Some(ident),
            params,
            block,
            visibility,
            span: span.into(),
        })
    }

    /// Parse the given source code to a class member.
    /// Make sure that the given pair is a class member, otherwise this will panic.
    pub fn parse_class_member(member: Pair<'a, Rule>) -> OYResult<Member> {
        let span = member.as_span();
        let mut inner = member.into_inner();
        let visibility = Self::parse_visibility(inner.next().unwrap());
        let ident = check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "class member",
            "the class member must be snake_case",
            utils::Case::Snake,
        )?;
        Ok(Member {
            ident,
            visibility,
            span: span.into(),
        })
    }

    /// Parse the given source code to a class statement.
    /// Make sure that the given pair is a class statement, otherwise this will panic.
    pub fn parse_class(class_def: Pair<'a, Rule>) -> OYResult<ClassStatement> {
        let class_span = class_def.as_span();
        let mut inner = class_def.into_inner();
        let visibility = Self::parse_visibility(inner.next().unwrap());
        let ident = utils::check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "class",
            "the class name must be PascalCase",
            utils::Case::Pascal,
        )?;
        let mut block = inner.next().unwrap().into_inner();
        let members = block
            .next()
            .unwrap()
            .into_inner()
            .map(Self::parse_class_member)
            .collect::<OYResult<Vec<_>>>()?;
        let methods = block
            .next()
            .unwrap()
            .into_inner()
            .map(Self::parse_function)
            .collect::<OYResult<Vec<_>>>()?;
        Ok(ClassStatement {
            ident,
            members,
            methods,
            visibility,
            span: class_span.into(),
        })
    }

    /// Parse the argument of a function call.
    /// Make sure that the given pair is a function argument, otherwise this will panic.
    pub fn parse_arg(arg: Pair<'a, Rule>) -> OYResult<Arg> {
        let mut inner = arg.into_inner();
        let is_unpack = inner.next().unwrap().as_str() == "...";
        let expr = Self::parse_expression(inner.next().unwrap())?;
        let span = expr.span();
        Ok(Arg {
            expr,
            is_unpack,
            span,
        })
    }

    /// Parse the parameter of a function.
    /// Make sure that the given pair is a function parameter, otherwise this will panic.
    pub fn parse_param(param: Pair<'a, Rule>) -> OYResult<Param> {
        let mut inner = param.into_inner();
        let is_pack = inner.next().unwrap().as_str() == "*";
        let ident = utils::check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "parameter",
            "the parameter name must be snake_case",
            utils::Case::Snake,
        )?;
        Ok(Param { ident, is_pack })
    }

    /// Parse the given source code to a assignment statement.
    /// Make sure that the given pair is a assignment statement, otherwise this will panic.
    pub fn parse_assignment(assignment: Pair<'a, Rule>) -> OYResult<Statement> {
        let span = assignment.as_span();
        let mut inner = assignment.into_inner();
        let ident = utils::check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "variable",
            "the variable name must be snake_case",
            utils::Case::Snake,
        )?;
        let expression = Self::parse_expression(inner.next().unwrap())?;
        Ok(Statement::Assignment(AssignmentStatement {
            ident,
            expression,
            span: span.into(),
        }))
    }
}
