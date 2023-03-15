use std::str::FromStr;

use crate::{
    ast::{
        AssignmentStatement, FunctionCallExpression, FunctionStatement, Ident, ObjectExpression,
        Param, ReturnStatement, ValueExpression, Visibility,
    },
    errors::{Error as OYError, Result as OYResult},
    utils,
};

use super::ast::{Block, ExpressionStatement, Program, Statement};
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
            Rule::func_def => Ok(Some(Self::parse_function(statement)?)),
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
                dbg!(visibility);
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
        Ok(FunctionCallExpression {
            ident: Self::parse_ident(inner.next().unwrap()),
            args: inner
                .next()
                .unwrap()
                .into_inner()
                .map(|arg| {
                    let expr = arg.into_inner().next().unwrap();
                    Self::parse_expression(expr)
                })
                .collect::<OYResult<_>>()?,
            span: span.into(),
        })
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
                // Remove the quotes, because they are not part of the string.
                value.as_str()[1..value.as_str().len() - 1].to_owned(),
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

    /// Parse the given source code to a function statement.
    /// Make sure that the given pair is a function statement, otherwise this will panic.
    pub fn parse_function(func: Pair<'a, Rule>) -> OYResult<Statement> {
        let span = func.as_span();
        let mut inner = func.into_inner();
        let visibility = Self::parse_visibility(inner.next().unwrap());
        let ident = utils::check_ident_case(
            Self::parse_ident(inner.next().unwrap()),
            "function",
            "the function name must be snake_case",
            utils::Case::Snake,
        )?;
        let params = inner.next().unwrap();
        let params = params
            .into_inner()
            .map(Self::parse_param)
            .collect::<OYResult<Vec<_>>>()?;
        let block = Some(Self::parse_block(inner.next().unwrap())?);
        utils::check_main_function(&ident, &params, &visibility)?;
        Ok(Statement::Function(FunctionStatement {
            ident,
            params,
            block,
            visibility,
            span: span.into(),
        }))
    }

    /// Parse the parameter of a function.
    /// Make sure that the given pair is a function parameter, otherwise this will panic.
    pub fn parse_param(param: Pair<'a, Rule>) -> OYResult<Param> {
        let ident = utils::check_ident_case(
            Self::parse_ident(param.into_inner().next().unwrap()),
            "parameter",
            "the parameter name must be snake_case",
            utils::Case::Snake,
        )?;
        Ok(Param { ident })
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
