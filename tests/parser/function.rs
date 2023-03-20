use ocypode_lang::{ast::*, errors::ErrorKind, parser::OYParser};

#[test]
fn test_params() {
    let source = "~main<argc><argv>{<func<>;>}";
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: Vec::new(),
                        span: Span::new(19, 25),
                    },
                ))],
                span: Span::new(17, 28),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 28),
        })],
        pest::Span::new(source, 0, 28).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                    ident: "arg1".to_owned(),
                                    span: Span::new(24, 28),
                                })),
                                is_unpack: false,
                                span: Span::new(24, 28),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                    ident: "arg2".to_owned(),
                                    span: Span::new(30, 34),
                                })),
                                is_unpack: false,
                                span: Span::new(30, 34),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Ident(Ident {
                                    ident: "arg3".to_owned(),
                                    span: Span::new(36, 40),
                                })),
                                is_unpack: false,
                                span: Span::new(36, 40),
                            },
                        ],
                        span: Span::new(19, 41),
                    },
                ))],
                span: Span::new(17, 44),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 44),
        })],
        pest::Span::new(source, 0, 44).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("arg1".to_owned(), Span::new(24, 30)),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 30),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("arg2".to_owned(), Span::new(32, 38)),
                                )),
                                is_unpack: false,
                                span: Span::new(32, 38),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("arg3".to_owned(), Span::new(40, 46)),
                                )),
                                is_unpack: false,
                                span: Span::new(40, 46),
                            },
                        ],
                        span: Span::new(19, 47),
                    },
                ))],
                span: Span::new(17, 50),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 50),
        })],
        pest::Span::new(source, 0, 50).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(24, 25)),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 25),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(2.into(), Span::new(27, 28)),
                                )),
                                is_unpack: false,
                                span: Span::new(27, 28),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(3.into(), Span::new(30, 31)),
                                )),
                                is_unpack: false,
                                span: Span::new(30, 31),
                            },
                        ],
                        span: Span::new(19, 32),
                    },
                ))],
                span: Span::new(17, 35),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 35),
        })],
        pest::Span::new(source, 0, 35).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(24, 27),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 27),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "2.0".parse().unwrap(),
                                        Span::new(29, 32),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(29, 32),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "3.0".parse().unwrap(),
                                        Span::new(34, 37),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(34, 37),
                            },
                        ],
                        span: Span::new(19, 38),
                    },
                ))],
                span: Span::new(17, 41),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 41),
        })],
        pest::Span::new(source, 0, 41).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Bool(true, Span::new(24, 28)),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 28),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Bool(false, Span::new(30, 35)),
                                )),
                                is_unpack: false,
                                span: Span::new(30, 35),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Bool(true, Span::new(37, 41)),
                                )),
                                is_unpack: false,
                                span: Span::new(37, 41),
                            },
                        ],
                        span: Span::new(19, 42),
                    },
                ))],
                span: Span::new(17, 45),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 45),
        })],
        pest::Span::new(source, 0, 45).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(24, 27)),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 27),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(29, 32)),
                                )),
                                is_unpack: false,
                                span: Span::new(29, 32),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(34, 37)),
                                )),
                                is_unpack: false,
                                span: Span::new(34, 37),
                            },
                        ],
                        span: Span::new(19, 38),
                    },
                ))],
                span: Span::new(17, 41),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 41),
        })],
        pest::Span::new(source, 0, 41).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::FunctionCall(
                    FunctionCallExpression {
                        callable: ValueExpression::Ident(Ident {
                            ident: "func".to_owned(),
                            span: Span::new(19, 23),
                        }),
                        args: vec![
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(1.into(), Span::new(25, 26)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(2.into(), Span::new(27, 28)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(3.into(), Span::new(29, 30)),
                                            )),
                                        ],
                                        Span::new(24, 31),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(24, 31),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(1.into(), Span::new(34, 35)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(2.into(), Span::new(36, 37)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(3.into(), Span::new(38, 39)),
                                            )),
                                        ],
                                        Span::new(33, 40),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(33, 40),
                            },
                            Arg {
                                expr: ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(1.into(), Span::new(43, 44)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(2.into(), Span::new(45, 46)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(3.into(), Span::new(47, 48)),
                                            )),
                                        ],
                                        Span::new(42, 49),
                                    ),
                                )),
                                is_unpack: false,
                                span: Span::new(42, 49),
                            },
                        ],
                        span: Span::new(19, 50),
                    },
                ))],
                span: Span::new(17, 53),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 53),
        })],
        pest::Span::new(source, 0, 53).unwrap(),
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
            ident: Some(Ident {
                ident: "foo".to_owned(),
                span: Span::new(1, 4),
            }),
            params: vec![
                Param {
                    ident: Ident {
                        ident: "p1".to_owned(),
                        span: Span::new(5, 7),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "p2".to_owned(),
                        span: Span::new(9, 11),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "p3".to_owned(),
                        span: Span::new(13, 15),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "p4".to_owned(),
                        span: Span::new(18, 20),
                    },
                    is_pack: false,
                },
                Param {
                    ident: Ident {
                        ident: "p5".to_owned(),
                        span: Span::new(23, 25),
                    },
                    is_pack: false,
                },
            ],
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(27, 31),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 31),
        })],
        pest::Span::new(source, 0, 31).unwrap(),
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
