use ocypode_lang::{ast::*, parser::OYParser};
use pest::Span;

#[test]
fn test_return_integer() {
    let source = "~main<argc><argv>{<return 2;>}";
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Int(2.into(), Span::new(source, 26, 27).unwrap()),
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
fn test_return_float() {
    let source = "~main<argc><argv>{<return 2.0;>}";
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Float(
                            "2.0".parse().unwrap(),
                            Span::new(source, 26, 29).unwrap(),
                        ),
                    )),
                    span: Span::new(source, 19, 29).unwrap(),
                })],
                span: Span::new(source, 17, 32).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 32).unwrap(),
        })],
        Span::new(source, 0, 32).unwrap(),
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::String(
                            "hello".to_owned(),
                            Span::new(source, 26, 33).unwrap(),
                        ),
                    )),
                    span: Span::new(source, 19, 33).unwrap(),
                })],
                span: Span::new(source, 17, 36).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 36).unwrap(),
        })],
        Span::new(source, 0, 36).unwrap(),
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 27, 28).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        2.into(),
                                        Span::new(source, 30, 31).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        3.into(),
                                        Span::new(source, 33, 34).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 26, 35).unwrap(),
                        ),
                    )),
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

#[test]
fn test_return_bool() {
    let source = "~main<argc><argv>{<return true;>}";
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Bool(true, Span::new(source, 26, 30).unwrap()),
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
fn test_return_nil() {
    let source = "~main<argc><argv>{<return nil;>}";
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
                statements: vec![Statement::Return(ReturnStatement {
                    value: ExpressionStatement::Value(ValueExpression::Object(
                        ObjectExpression::Nil(Span::new(source, 26, 29).unwrap()),
                    )),
                    span: Span::new(source, 19, 29).unwrap(),
                })],
                span: Span::new(source, 17, 32).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 32).unwrap(),
        })],
        Span::new(source, 0, 32).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}
