use ocypode_lang::{ast::*, parser::OYParser};

#[test]
fn test_return_integer() {
    let source = "~main<argc><argv>{<return 2;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Int(2.into(), Span::new(26, 27)),
                    )),
                    span: Span::new(19, 27),
                })],
                span: Span::new(17, 30),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 30),
        })],
        pest::Span::new(source, 0, 30).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_return_float() {
    let source = "~main<argc><argv>{<return 2.0;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Float("2.0".parse().unwrap(), Span::new(26, 29)),
                    )),
                    span: Span::new(19, 29),
                })],
                span: Span::new(17, 32),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 32),
        })],
        pest::Span::new(source, 0, 32).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_return_string() {
    let source = "~main<argc><argv>{<return \"hello\";>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::String("hello".to_owned(), Span::new(26, 33)),
                    )),
                    span: Span::new(19, 33),
                })],
                span: Span::new(17, 36),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 36),
        })],
        pest::Span::new(source, 0, 36).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_return_array() {
    let source = "~main<argc><argv>{<return [1, 2, 3];>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(27, 28)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(2.into(), Span::new(30, 31)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(3.into(), Span::new(33, 34)),
                                )),
                            ],
                            Span::new(26, 35),
                        ),
                    )),
                    span: Span::new(19, 35),
                })],
                span: Span::new(17, 38),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 38),
        })],
        pest::Span::new(source, 0, 38).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_return_bool() {
    let source = "~main<argc><argv>{<return true;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Bool(true, Span::new(26, 30)),
                    )),
                    span: Span::new(19, 30),
                })],
                span: Span::new(17, 33),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 33),
        })],
        pest::Span::new(source, 0, 33).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_return_nil() {
    let source = "~main<argc><argv>{<return nil;>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Some(Ident {
                ident: "main".to_owned(),
                span: Span::new(1, 5),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(6, 10),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(12, 16),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Nil(Span::new(26, 29)),
                    )),
                    span: Span::new(19, 29),
                })],
                span: Span::new(17, 32),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 32),
        })],
        pest::Span::new(source, 0, 32).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}
