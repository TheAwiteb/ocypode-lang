use ocypode_lang::{ast::*, parser::OYParser};
use pest::Span;

#[test]
fn test_assign_to_int() {
    let source = "~main<argc><argv>{<a = 1;>}";
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Int(1.into(), Span::new(source, 23, 24).unwrap()),
                    )),
                    span: Span::new(source, 19, 24).unwrap(),
                })],
                span: Span::new(source, 17, 27).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 27).unwrap(),
        })],
        Span::new(source, 0, 27).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Ident(Ident {
                        ident: "b".to_owned(),
                        span: Span::new(source, 23, 24).unwrap(),
                    })),
                    span: Span::new(source, 19, 24).unwrap(),
                })],
                span: Span::new(source, 17, 27).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 27).unwrap(),
        })],
        Span::new(source, 0, 27).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Float(
                            "1.0".parse().unwrap(),
                            Span::new(source, 23, 26).unwrap(),
                        ),
                    )),
                    span: Span::new(source, 19, 26).unwrap(),
                })],
                span: Span::new(source, 17, 29).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 29).unwrap(),
        })],
        Span::new(source, 0, 29).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::String(
                            "hello".to_owned(),
                            Span::new(source, 23, 30).unwrap(),
                        ),
                    )),
                    span: Span::new(source, 19, 30).unwrap(),
                })],
                span: Span::new(source, 17, 33).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 33).unwrap(),
        })],
        Span::new(source, 0, 33).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Bool(true, Span::new(source, 23, 27).unwrap()),
                    )),
                    span: Span::new(source, 19, 27).unwrap(),
                })],
                span: Span::new(source, 17, 30).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 30).unwrap(),
        })],
        Span::new(source, 0, 30).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        "1".parse().unwrap(),
                                        Span::new(source, 24, 25).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        "2".parse().unwrap(),
                                        Span::new(source, 27, 28).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        "3".parse().unwrap(),
                                        Span::new(source, 30, 31).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 23, 32).unwrap(),
                        ),
                    )),
                    span: Span::new(source, 19, 32).unwrap(),
                })],
                span: Span::new(source, 17, 35).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 35).unwrap(),
        })],
        Span::new(source, 0, 35).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Nil(Span::new(source, 23, 26).unwrap()),
                    )),
                    span: Span::new(source, 19, 26).unwrap(),
                })],
                span: Span::new(source, 17, 29).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 29).unwrap(),
        })],
        Span::new(source, 0, 29).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::FunctionCall(FunctionCallExpression {
                        ident: Ident {
                            ident: "fun".to_owned(),
                            span: Span::new(source, 23, 26).unwrap(),
                        },
                        args: vec![],
                        span: Span::new(source, 23, 28).unwrap(),
                    }),
                    span: Span::new(source, 19, 28).unwrap(),
                })],
                span: Span::new(source, 17, 31).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 31).unwrap(),
        })],
        Span::new(source, 0, 31).unwrap(),
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
                statements: vec![Statement::Assignment(AssignmentStatement {
                    ident: Ident {
                        ident: "a".to_owned(),
                        span: Span::new(source, 19, 20).unwrap(),
                    },
                    expression: ExpressionStatement::FunctionCall(FunctionCallExpression {
                        ident: Ident {
                            ident: "fun".to_owned(),
                            span: Span::new(source, 23, 26).unwrap(),
                        },
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(
                                    "1".parse().unwrap(),
                                    Span::new(source, 27, 28).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(
                                    "2".parse().unwrap(),
                                    Span::new(source, 30, 31).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(
                                    "3".parse().unwrap(),
                                    Span::new(source, 33, 34).unwrap(),
                                ),
                            )),
                        ],
                        span: Span::new(source, 23, 35).unwrap(),
                    }),
                    span: Span::new(source, 19, 35).unwrap(),
                })],
                span: Span::new(source, 17, 38).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 38).unwrap(),
        })],
        Span::new(source, 0, 38).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}
