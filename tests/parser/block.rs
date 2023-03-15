use ocypode_lang::{ast::*, parser::OYParser};

#[test]
fn test_single_line_block() {
    let source = "~main<argc><argv>{<func<>;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                },
            ],
            block: Block {
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        ident: Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        },
                        args: Vec::new(),
                        span: Span::new(19, 25),
                    },
                ))],
                span: Span::new(17, 28),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 28),
        })],
        pest::Span::new(source, 0, 28).unwrap(),
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
                span: Span::new(1, 5),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                },
            ],
            block: Block {
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        ident: Ident {
                            ident: "func".to_owned(),
                            span: Span::new(28, 32),
                        },
                        args: Vec::new(),
                        span: Span::new(28, 34),
                    },
                ))],
                span: Span::new(17, 42),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 42),
        })],
        pest::Span::new(source, 0, 42).unwrap(),
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
