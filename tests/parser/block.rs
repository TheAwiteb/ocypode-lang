use ocypode_lang::{ast::*, parser::OYParser};
use pest::Span;

#[test]
fn test_single_line_block() {
    let source = "~main<argc><argv>{<func<>;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(source, 1, 5).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 6, 10).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 12, 16).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        ident: Ident {
                            ident: "func".to_owned(),
                            span: Span::new(source, 19, 23).unwrap(),
                        },
                        args: Vec::new(),
                        span: Span::new(source, 19, 25).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 28).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 28).unwrap(),
        })],
        Span::new(source, 0, 28).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_multi_line_block() {
    let source = "~main<argc><argv>{<
        func<>;
    >}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(source, 1, 5).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 6, 10).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 12, 16).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        ident: Ident {
                            ident: "func".to_owned(),
                            span: Span::new(source, 28, 32).unwrap(),
                        },
                        args: Vec::new(),
                        span: Span::new(source, 28, 34).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 42).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 42).unwrap(),
        })],
        Span::new(source, 0, 42).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_invalid_block() {
    let source = "~main<argc><argv>{func<>;}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
    let source = "~main<argc><argv>{<func<>;}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
    let source = "~main<argc><argv>{func<>;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
    let source = "~main<argc><argv><func<>;>";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
}

#[test]
fn test_invalid_multi_line_block() {
    let source = "~main<argc><argv>{<
        func<>;
    }>";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
    let source = "~main<argc><argv>{<
        func<>;
    >";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
    let source = "~main<argc><argv>{<
        func<>;
    }";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_err());
}
