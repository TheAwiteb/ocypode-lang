use pest::Span;

use ocypode_lang::{ast::*, errors::ErrorKind, parser::OYParser};

#[test]
fn test_params() {
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
fn test_more_function_args_as_ident() {
    let source = "~main<argc><argv>{<func<arg1><arg2><arg3>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                ident: "arg1".to_owned(),
                                span: Span::new(source, 24, 28).unwrap(),
                            })),
                            ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                ident: "arg2".to_owned(),
                                span: Span::new(source, 30, 34).unwrap(),
                            })),
                            ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                ident: "arg3".to_owned(),
                                span: Span::new(source, 36, 40).unwrap(),
                            })),
                        ],
                        span: Span::new(source, 19, 41).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 44).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 44).unwrap(),
        })],
        Span::new(source, 0, 44).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_args_as_string() {
    let source = "~main<argc><argv>{<func<\"arg1\"><\"arg2\"><\"arg3\">;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::String(
                                    "arg1".to_owned(),
                                    Span::new(source, 24, 30).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::String(
                                    "arg2".to_owned(),
                                    Span::new(source, 32, 38).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::String(
                                    "arg3".to_owned(),
                                    Span::new(source, 40, 46).unwrap(),
                                ),
                            )),
                        ],
                        span: Span::new(source, 19, 47).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 50).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 50).unwrap(),
        })],
        Span::new(source, 0, 50).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_args_as_int() {
    let source = "~main<argc><argv>{<func<1><2><3>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(1.into(), Span::new(source, 24, 25).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(2.into(), Span::new(source, 27, 28).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(3.into(), Span::new(source, 30, 31).unwrap()),
                            )),
                        ],
                        span: Span::new(source, 19, 32).unwrap(),
                    },
                ))],
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
fn test_more_function_args_as_float() {
    let source = "~main<argc><argv>{<func<1.0><2.0><3.0>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Float(
                                    "1.0".parse().unwrap(),
                                    Span::new(source, 24, 27).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Float(
                                    "2.0".parse().unwrap(),
                                    Span::new(source, 29, 32).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Float(
                                    "3.0".parse().unwrap(),
                                    Span::new(source, 34, 37).unwrap(),
                                ),
                            )),
                        ],
                        span: Span::new(source, 19, 38).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 41).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 41).unwrap(),
        })],
        Span::new(source, 0, 41).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_args_as_bool() {
    let source = "~main<argc><argv>{<func<true><false><true>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Bool(true, Span::new(source, 24, 28).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Bool(false, Span::new(source, 30, 35).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Bool(true, Span::new(source, 37, 41).unwrap()),
                            )),
                        ],
                        span: Span::new(source, 19, 42).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 45).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 45).unwrap(),
        })],
        Span::new(source, 0, 45).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_args_as_nil() {
    let source = "~main<argc><argv>{<func<nil><nil><nil>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Nil(Span::new(source, 24, 27).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Nil(Span::new(source, 29, 32).unwrap()),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Nil(Span::new(source, 34, 37).unwrap()),
                            )),
                        ],
                        span: Span::new(source, 19, 38).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 41).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 41).unwrap(),
        })],
        Span::new(source, 0, 41).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_args_as_array() {
    let source = "~main<argc><argv>{<func<[1,2,3]><[1,2,3]><[1,2,3]>;>}";
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
                        args: vec![
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Array(
                                    vec![
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                1.into(),
                                                Span::new(source, 25, 26).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                2.into(),
                                                Span::new(source, 27, 28).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                3.into(),
                                                Span::new(source, 29, 30).unwrap(),
                                            ),
                                        )),
                                    ],
                                    Span::new(source, 24, 31).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Array(
                                    vec![
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                1.into(),
                                                Span::new(source, 34, 35).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                2.into(),
                                                Span::new(source, 36, 37).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                3.into(),
                                                Span::new(source, 38, 39).unwrap(),
                                            ),
                                        )),
                                    ],
                                    Span::new(source, 33, 40).unwrap(),
                                ),
                            )),
                            ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Array(
                                    vec![
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                1.into(),
                                                Span::new(source, 43, 44).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                2.into(),
                                                Span::new(source, 45, 46).unwrap(),
                                            ),
                                        )),
                                        ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Int(
                                                3.into(),
                                                Span::new(source, 47, 48).unwrap(),
                                            ),
                                        )),
                                    ],
                                    Span::new(source, 42, 49).unwrap(),
                                ),
                            )),
                        ],
                        span: Span::new(source, 19, 50).unwrap(),
                    },
                ))],
                span: Span::new(source, 17, 53).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 53).unwrap(),
        })],
        Span::new(source, 0, 53).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_more_function_params() {
    let source = "~foo<p1><p2><p3> <p4> <p5> {<>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "foo".to_owned(),
                span: Span::new(source, 1, 4).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "p1".to_owned(),
                        span: Span::new(source, 5, 7).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "p2".to_owned(),
                        span: Span::new(source, 9, 11).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "p3".to_owned(),
                        span: Span::new(source, 13, 15).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "p4".to_owned(),
                        span: Span::new(source, 18, 20).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "p5".to_owned(),
                        span: Span::new(source, 23, 25).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: Vec::new(),
                span: Span::new(source, 27, 31).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 31).unwrap(),
        })],
        Span::new(source, 0, 31).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_invalid_function_params() {
    let source = "~foo<p1, p2, p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    let source = "~foo<p1 p2 p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    let source = "~foo<<p1> <p2> <p3>>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_args() {
    let source = "~foo<p1><p2><p3>{<func<arg1, arg2, arg3>;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    let source = "~foo<p1><p2><p3>{<func<arg1 arg2 arg3>;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    let source = "~foo<p1><p2><p3>{<func<<arg1><arg2><arg3>>;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_with_double_function_tag() {
    let source = "~~main<argc><argv>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_camal_case() {
    let source = "~fooBar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::InvalidName(..)));
}

#[test]
fn test_invalid_function_name_with_kebab_case() {
    let source = "~foo-bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_pascal_case() {
    let source = "~FooBar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::InvalidName(..)));
}

#[test]
fn test_invalid_function_name_with_start_with_digit() {
    let source = "~1foo<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_empty() {
    let source = "~<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_space() {
    let source = "~foo bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_dot() {
    let source = "~foo.bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_colon() {
    let source = "~foo:bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_slash() {
    let source = "~foo/bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_function_name_with_plus() {
    let source = "~foo+bar<p1><p2><p3>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}
