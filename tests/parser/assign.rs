use ocypode_lang::{ast::*, parser::OYParser};

#[test]
fn test_assign_to_int() {
    let source = "~main<argc><argv>{<a = 1;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Int(1.into(), Span::new(23, 24)),
                    )),
                    span: Span::new(19, 24),
                })],
                span: Span::new(17, 27),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 27),
        })],
        pest::Span::new(source, 0, 27).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_ident() {
    let source = "~main<argc><argv>{<a = b;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Ident(Ident {
                        ident: "b".to_owned(),
                        span: Span::new(23, 24),
                    })),
                    span: Span::new(19, 24),
                })],
                span: Span::new(17, 27),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 27),
        })],
        pest::Span::new(source, 0, 27).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_float() {
    let source = "~main<argc><argv>{<a = 1.0;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Float("1.0".parse().unwrap(), Span::new(23, 26)),
                    )),
                    span: Span::new(19, 26),
                })],
                span: Span::new(17, 29),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 29),
        })],
        pest::Span::new(source, 0, 29).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_string() {
    let source = "~main<argc><argv>{<a = \"hello\";>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::String("hello".to_owned(), Span::new(23, 30)),
                    )),
                    span: Span::new(19, 30),
                })],
                span: Span::new(17, 33),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 33),
        })],
        pest::Span::new(source, 0, 33).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_bool() {
    let source = "~main<argc><argv>{<a = true;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Bool(true, Span::new(23, 27)),
                    )),
                    span: Span::new(19, 27),
                })],
                span: Span::new(17, 30),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 30),
        })],
        pest::Span::new(source, 0, 30).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_array() {
    let source = "~main<argc><argv>{<a = [1, 2, 3];>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int("1".parse().unwrap(), Span::new(24, 25)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int("2".parse().unwrap(), Span::new(27, 28)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int("3".parse().unwrap(), Span::new(30, 31)),
                                )),
                            ],
                            Span::new(23, 32),
                        ),
                    )),
                    span: Span::new(19, 32),
                })],
                span: Span::new(17, 35),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 35),
        })],
        pest::Span::new(source, 0, 35).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_nil() {
    let source = "~main<argc><argv>{<a = nil;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Nil(Span::new(23, 26)),
                    )),
                    span: Span::new(19, 26),
                })],
                span: Span::new(17, 29),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 29),
        })],
        pest::Span::new(source, 0, 29).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_fun_call() {
    let source = "~main<argc><argv>{<a = fun<>;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::FunctionCall(FunctionCallExpression {
                        ident: Ident {
                            ident: "fun".to_owned(),
                            span: Span::new(23, 26),
                        },
                        args: vec![],
                        span: Span::new(23, 28),
                    }),
                    span: Span::new(19, 28),
                })],
                span: Span::new(17, 31),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 31),
        })],
        pest::Span::new(source, 0, 31).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_assign_to_fun_call_with_args() {
    let source = "~main<argc><argv>{<a = fun<1><2><3>;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(19, 20),
                    },
                    expression: ExpressionStatement::FunctionCall(FunctionCallExpression {
                        ident: Ident {
                            ident: "fun".to_owned(),
                            span: Span::new(23, 26),
                        },
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int("1".parse().unwrap(), Span::new(27, 28)),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int("2".parse().unwrap(), Span::new(30, 31)),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int("3".parse().unwrap(), Span::new(33, 34)),
                            )),
                        ],
                        span: Span::new(23, 35),
                    }),
                    span: Span::new(19, 35),
                })],
                span: Span::new(17, 38),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 38),
        })],
        pest::Span::new(source, 0, 38).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}
