use ocypode_lang::ast::*;
use ocypode_lang::errors::ErrorKind;
use ocypode_lang::parser::OYParser;
use pest::Span;

#[cfg(test)]
mod nil {
    use super::*;

    #[test]
    fn test_nil() {
        let source = "~main<argc><argv>{<nil;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Nil(
                            Span::new(source, 19, 22).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 25).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 25).unwrap(),
            })],
            Span::new(source, 0, 25).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_nil_as_prefix() {
        let source = "~main<argc><argv>{<nilsome;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Ident(Ident {
                            ident: "nilsome".to_owned(),
                            span: Span::new(source, 19, 26).unwrap(),
                        }),
                    ))],
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
    fn test_nil_as_suffix() {
        let source = "~main<argc><argv>{<somnil;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Ident(Ident {
                            ident: "somnil".to_owned(),
                            span: Span::new(source, 19, 25).unwrap(),
                        }),
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
}

#[cfg(test)]
mod bool {
    use super::*;

    #[test]
    fn test_bool() {
        let source = "~main<argc><argv>{<true;false;>}";
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Bool(true, Span::new(source, 19, 23).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Bool(false, Span::new(source, 24, 29).unwrap()),
                        ))),
                    ],
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
    fn test_bool_as_prefix() {
        let source = "~main<argc><argv>{<truesome;falsesome;>}";
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "truesome".to_owned(),
                                span: Span::new(source, 19, 27).unwrap(),
                            },
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "falsesome".to_owned(),
                                span: Span::new(source, 28, 37).unwrap(),
                            },
                        ))),
                    ],
                    span: Span::new(source, 17, 40).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 40).unwrap(),
            })],
            Span::new(source, 0, 40).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_bool_as_suffix() {
        let source = "~main<argc><argv>{<sometrue;somefalse;>}";
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "sometrue".to_owned(),
                                span: Span::new(source, 19, 27).unwrap(),
                            },
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "somefalse".to_owned(),
                                span: Span::new(source, 28, 37).unwrap(),
                            },
                        ))),
                    ],
                    span: Span::new(source, 17, 40).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 40).unwrap(),
            })],
            Span::new(source, 0, 40).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }
}

#[cfg(test)]
mod int {
    use super::*;

    #[test]
    fn test_int() {
        let source = "~main<argc><argv>{<0;1;2;3;4;5;6;7;8;9;>}";
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(0.into(), Span::new(source, 19, 20).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(1.into(), Span::new(source, 21, 22).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(2.into(), Span::new(source, 23, 24).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(3.into(), Span::new(source, 25, 26).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(4.into(), Span::new(source, 27, 28).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(5.into(), Span::new(source, 29, 30).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(6.into(), Span::new(source, 31, 32).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(7.into(), Span::new(source, 33, 34).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(8.into(), Span::new(source, 35, 36).unwrap()),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(9.into(), Span::new(source, 37, 38).unwrap()),
                        ))),
                    ],
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
    fn test_int_as_prefix() {
        let source =
            "~main<argc><argv>{<0some;1some;2some;3some;4some;5some;6some;7some;8some;9some;>}";
        let ast = OYParser::parse_program(source);
        assert!(ast.is_err());
    }

    #[test]
    fn test_long_int() {
        let source = "~main<argc><argv>{<1234567890;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Int(
                            1234567890.into(),
                            Span::new(source, 19, 29).unwrap(),
                        )),
                    ))],
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
    fn test_very_long_int() {
        let source = "~main<argc><argv>{<123456789012345678901234567890;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Int(
                            "123456789012345678901234567890".parse().unwrap(),
                            Span::new(source, 19, 49).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 52).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 52).unwrap(),
            })],
            Span::new(source, 0, 52).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_very_long_int_as_prefix() {
        let source = "~main<argc><argv>{<123456789012345678901234567890some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }
}

#[cfg(test)]
mod float {
    use super::*;

    #[test]
    fn test_float() {
        let source = "~main<argc><argv>{<0.0;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "0.0".parse().unwrap(),
                            Span::new(source, 19, 22).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 25).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 25).unwrap(),
            })],
            Span::new(source, 0, 25).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_float_as_prefix() {
        let source = "~main<argc><argv>{<0.0some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }

    #[test]
    fn test_long_float() {
        let source = "~main<argc><argv>{<1234567890.1234567890;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "1234567890.1234567890".parse().unwrap(),
                            Span::new(source, 19, 40).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 43).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 43).unwrap(),
            })],
            Span::new(source, 0, 43).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_long_float_as_prefix() {
        let source = "~main<argc><argv>{<1234567890.1234567890some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }

    #[test]
    fn test_very_long_float() {
        let source =
            "~main<argc><argv>{<123456789012345678901234567890.123456789012345678901234567890;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "123456789012345678901234567890.123456789012345678901234567890"
                                .parse()
                                .unwrap(),
                            Span::new(source, 19, 80).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 83).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 83).unwrap(),
            })],
            Span::new(source, 0, 83).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_very_long_float_as_prefix() {
        let source =
        "~main<argc><argv>{<123456789012345678901234567890.123456789012345678901234567890some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }
}

#[cfg(test)]
mod string {
    use super::*;
    #[test]
    fn test_empty_string() {
        let source = r#"~main<argc><argv>{<"";>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "".to_owned(),
                            Span::new(source, 19, 21).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 24).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 24).unwrap(),
            })],
            Span::new(source, 0, 24).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_empty_string_as_prefix() {
        let source = "~main<argc><argv>{<\"\"some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }

    #[test]
    fn test_single_character_string() {
        let source = "~main<argc><argv>{<\"a\";>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "a".to_owned(),
                            Span::new(source, 19, 22).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 25).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 25).unwrap(),
            })],
            Span::new(source, 0, 25).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_one_word_string() {
        let source = "~main<argc><argv>{<\"some\";>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "some".to_owned(),
                            Span::new(source, 19, 25).unwrap(),
                        )),
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
    fn test_one_word_string_as_prefix() {
        let source = "~main<argc><argv>{<\"some\"some;>}";
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }

    #[test]
    fn test_one_word_string_with_escape() {
        let source = r#"~main<argc><argv>{<"some\"";>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            r#"some\""#.to_owned(),
                            Span::new(source, 19, 27).unwrap(),
                        )),
                    ))],
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
    fn test_one_word_string_with_escape_as_prefix() {
        let source = r#"~main<argc><argv>{<"some\""some;>}"#;
        let ast = OYParser::parse_program(source);
        assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
    }

    #[test]
    fn test_multiline_string() {
        let source = "~main<argc><argv>{<\"some\nmultiline\";>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "some\nmultiline".to_owned(),
                            Span::new(source, 19, 35).unwrap(),
                        )),
                    ))],
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
    fn test_multiline_string_with_escape() {
        let source = r#"~main<argc><argv>{<"some\nmultiline\"";>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            r#"some\nmultiline\""#.to_owned(),
                            Span::new(source, 19, 38).unwrap(),
                        )),
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
    fn test_long_string() {
        let source = "~main<argc><argv>{<\"Consequat aliquip reprehenderit ex dolore reprehenderit ut sunt cupidatat aute.Incididunt ex nisi id et.Deserunt aute est sit dolor tempor.\";>}";
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
                statements: vec![Statement::Expression(ExpressionStatement::Value(
                    ValueExpression::Object(ObjectExpression::String(
                        "Consequat aliquip reprehenderit ex dolore reprehenderit ut sunt cupidatat aute.Incididunt ex nisi id et.Deserunt aute est sit dolor tempor.".to_owned(),
                        Span::new(source, 19, 160).unwrap(),
                    )),
                ))],
                span: Span::new(source, 17, 163).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 163).unwrap(),
        })],
        Span::new(source, 0, 163).unwrap(),
    );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_long_string_with_escape() {
        let lorem = r#"Consequat aliquip reprehenderit ex dolore reprehenderit ut sunt cupidatat aute.Incididunt ex nisi id et.Deserunt aute est sit dolor tempor.\""#;
        let source = format!(r#"~main<argc><argv>{{<"{}";>}}"#, lorem);
        let ast = OYParser::parse_program(&source);
        assert!(ast.is_ok());
        let program = (
            vec![Statement::Function(FunctionStatement {
                ident: Ident {
                    ident: "main".to_owned(),
                    span: Span::new(&source, 1, 5).unwrap(),
                },
                params: vec![
                    Param {
                        ident: Ident {
                            ident: "argc".to_owned(),
                            span: Span::new(&source, 6, 10).unwrap(),
                        },
                    },
                    Param {
                        ident: Ident {
                            ident: "argv".to_owned(),
                            span: Span::new(&source, 12, 16).unwrap(),
                        },
                    },
                ],
                block: Block {
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            lorem.to_owned(),
                            Span::new(&source, 19, 19 + lorem.len() + 2).unwrap(),
                        )),
                    ))],
                    span: Span::new(&source, 17, 19 + lorem.len() + 5).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
            })],
            Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_long_multiline_string() {
        let lorem = "Mollit ex consequat deserunt dolore. Dolore nisi aliquip sunt eu sit in incididunt. Culpa incididunt pariatur laborum sit sunt est pariatur quis eiusmod excepteur consequat amet pariatur magna. Incididunt est labore nisi Lorem fugiat. Laboris tempor adipisicing aliqua laboris qui proident mollit est pariatur dolore occaecat consequat. Aliquip minim nostrud non est laborum mollit exercitation laboris exercitation cillum velit incididunt veniam elit.

        Esse ipsum ipsum cupidatat duis eu aute ipsum consectetur enim irure velit qui cupidatat esse. Fugiat culpa tempor elit dolore. Occaecat nostrud ut eiusmod duis irure minim sunt et est non esse aliqua duis nulla. Nostrud velit occaecat minim Lorem anim. Nostrud ea tempor ex Lorem enim. Labore sit in Lorem occaecat sint.
        
        Sunt sit tempor ad consequat commodo ex occaecat voluptate pariatur nisi laboris ad duis commodo. Exercitation ex aliqua labore ut non fugiat culpa proident consectetur. Cupidatat Lorem deserunt ad ad sunt anim reprehenderit laborum adipisicing aliqua adipisicing.
        
        Occaecat aute veniam nisi enim. Cillum dolor deserunt culpa magna aliquip laborum mollit laborum nostrud eiusmod. Excepteur eiusmod culpa adipisicing ipsum nisi Lorem proident laboris cillum voluptate sunt.
        
        Cillum anim labore esse laboris deserunt cupidatat elit Lorem do laboris sint. Elit consequat nisi eu in veniam duis laborum reprehenderit adipisicing reprehenderit occaecat. Officia enim quis magna nisi.";
        let source = format!(r#"~main<argc><argv>{{<"{}";>}}"#, lorem);
        let ast = OYParser::parse_program(&source);
        assert!(ast.is_ok());
        let program = (
            vec![Statement::Function(FunctionStatement {
                ident: Ident {
                    ident: "main".to_owned(),
                    span: Span::new(&source, 1, 5).unwrap(),
                },
                params: vec![
                    Param {
                        ident: Ident {
                            ident: "argc".to_owned(),
                            span: Span::new(&source, 6, 10).unwrap(),
                        },
                    },
                    Param {
                        ident: Ident {
                            ident: "argv".to_owned(),
                            span: Span::new(&source, 12, 16).unwrap(),
                        },
                    },
                ],
                block: Block {
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            lorem.to_owned(),
                            Span::new(&source, 19, 19 + lorem.len() + 2).unwrap(),
                        )),
                    ))],
                    span: Span::new(&source, 17, 19 + lorem.len() + 5).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
            })],
            Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }
}

#[cfg(test)]
mod array {
    use super::*;

    #[test]
    fn test_empty_array() {
        let source = r#"~main<argc><argv>{<[];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![],
                            Span::new(source, 19, 21).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 24).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 24).unwrap(),
            })],
            Span::new(source, 0, 24).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_one_element() {
        let source = r#"~main<argc><argv>{<[1];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(1.into(), Span::new(source, 20, 21).unwrap()),
                            ))],
                            Span::new(source, 19, 22).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 25).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 25).unwrap(),
            })],
            Span::new(source, 0, 25).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_of_intgers() {
        let source = r#"~main<argc><argv>{<[1, 2, 3];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 20, 21).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        2.into(),
                                        Span::new(source, 23, 24).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        3.into(),
                                        Span::new(source, 26, 27).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 28).unwrap(),
                        )),
                    ))],
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
    fn test_big_array_of_integer() {
        let source = r#"~main<argc><argv>{<[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 20, 21).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        2.into(),
                                        Span::new(source, 23, 24).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        3.into(),
                                        Span::new(source, 26, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        4.into(),
                                        Span::new(source, 29, 30).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        5.into(),
                                        Span::new(source, 32, 33).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        6.into(),
                                        Span::new(source, 35, 36).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        7.into(),
                                        Span::new(source, 38, 39).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        8.into(),
                                        Span::new(source, 41, 42).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        9.into(),
                                        Span::new(source, 44, 45).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        10.into(),
                                        Span::new(source, 47, 49).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 50).unwrap(),
                        )),
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
    fn test_array_with_one_nested_array_of_integer() {
        let source = r#"~main<argc><argv>{<[1, [2, 3, 4], 5];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 20, 21).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    2.into(),
                                                    Span::new(source, 24, 25).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    3.into(),
                                                    Span::new(source, 27, 28).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    4.into(),
                                                    Span::new(source, 30, 31).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 23, 32).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        5.into(),
                                        Span::new(source, 34, 35).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 36).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 39).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 39).unwrap(),
            })],
            Span::new(source, 0, 39).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_tow_nested_array_of_integer() {
        let source = r#"~main<argc><argv>{<[1, [2, 3, 4], 5, [6, 7, 8, 9, 10]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 20, 21).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    2.into(),
                                                    Span::new(source, 24, 25).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    3.into(),
                                                    Span::new(source, 27, 28).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    4.into(),
                                                    Span::new(source, 30, 31).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 23, 32).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        5.into(),
                                        Span::new(source, 34, 35).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    6.into(),
                                                    Span::new(source, 38, 39).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    7.into(),
                                                    Span::new(source, 41, 42).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    8.into(),
                                                    Span::new(source, 44, 45).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    9.into(),
                                                    Span::new(source, 47, 48).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    10.into(),
                                                    Span::new(source, 50, 52).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 37, 53).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 54).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 57).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 57).unwrap(),
            })],
            Span::new(source, 0, 57).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_long_depath_of_array_integer() {
        let source = r#"~main<argc><argv>{<[1,2,[3,4],[323,23,[23,32,[232,32,],],],42];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(source, 20, 21).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        2.into(),
                                        Span::new(source, 22, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    3.into(),
                                                    Span::new(source, 25, 26).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    4.into(),
                                                    Span::new(source, 27, 28).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 24, 29).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    323.into(),
                                                    Span::new(source, 31, 34).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    23.into(),
                                                    Span::new(source, 35, 37).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![
                                                        ExpressionStatement::Value(
                                                            ValueExpression::Object(
                                                                ObjectExpression::Int(
                                                                    23.into(),
                                                                    Span::new(source, 39, 41)
                                                                        .unwrap(),
                                                                ),
                                                            ),
                                                        ),
                                                        ExpressionStatement::Value(
                                                            ValueExpression::Object(
                                                                ObjectExpression::Int(
                                                                    32.into(),
                                                                    Span::new(source, 42, 44)
                                                                        .unwrap(),
                                                                ),
                                                            ),
                                                        ),
                                                        ExpressionStatement::Value(
                                                            ValueExpression::Object(
                                                                ObjectExpression::Array(
                                                                    vec![
                                                                        ExpressionStatement::Value(
                                                                            ValueExpression::Object(
                                                                                ObjectExpression::Int(
                                                                                    232.into(),
                                                                                    Span::new(
                                                                                        source,
                                                                                        46,
                                                                                        49,
                                                                                    )
                                                                                    .unwrap(),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        ExpressionStatement::Value(
                                                                            ValueExpression::Object(
                                                                                ObjectExpression::Int(
                                                                                    32.into(),
                                                                                    Span::new(
                                                                                        source,
                                                                                        50,
                                                                                        52,
                                                                                    )
                                                                                    .unwrap(),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    Span::new(
                                                                        source,
                                                                        45,
                                                                        54,
                                                                    )
                                                                    .unwrap(),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    Span::new(source, 38, 56).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 30, 58).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        42.into(),
                                        Span::new(source, 59, 61).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 62).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 65).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 65).unwrap(),
            })],
            Span::new(source, 0, 65).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_of_float() {
        let source = r#"~main<argc><argv>{<[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(source, 20, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "2.0".parse().unwrap(),
                                        Span::new(source, 24, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "3.0".parse().unwrap(),
                                        Span::new(source, 28, 31).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "4.0".parse().unwrap(),
                                        Span::new(source, 32, 35).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "5.0".parse().unwrap(),
                                        Span::new(source, 36, 39).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "6.0".parse().unwrap(),
                                        Span::new(source, 40, 43).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "7.0".parse().unwrap(),
                                        Span::new(source, 44, 47).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "8.0".parse().unwrap(),
                                        Span::new(source, 48, 51).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "9.0".parse().unwrap(),
                                        Span::new(source, 52, 55).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "10.0".parse().unwrap(),
                                        Span::new(source, 56, 60).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 61).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 64).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 64).unwrap(),
            })],
            Span::new(source, 0, 64).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_big_array_of_float() {
        let source = "~main<argc><argv>{<[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0];>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(source, 20, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "2.0".parse().unwrap(),
                                        Span::new(source, 24, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "3.0".parse().unwrap(),
                                        Span::new(source, 28, 31).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "4.0".parse().unwrap(),
                                        Span::new(source, 32, 35).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "5.0".parse().unwrap(),
                                        Span::new(source, 36, 39).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "6.0".parse().unwrap(),
                                        Span::new(source, 40, 43).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "7.0".parse().unwrap(),
                                        Span::new(source, 44, 47).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "8.0".parse().unwrap(),
                                        Span::new(source, 48, 51).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "9.0".parse().unwrap(),
                                        Span::new(source, 52, 55).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "10.0".parse().unwrap(),
                                        Span::new(source, 56, 60).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "11.0".parse().unwrap(),
                                        Span::new(source, 61, 65).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "12.0".parse().unwrap(),
                                        Span::new(source, 66, 70).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "13.0".parse().unwrap(),
                                        Span::new(source, 71, 75).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "14.0".parse().unwrap(),
                                        Span::new(source, 76, 80).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "15.0".parse().unwrap(),
                                        Span::new(source, 81, 85).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "16.0".parse().unwrap(),
                                        Span::new(source, 86, 90).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "17.0".parse().unwrap(),
                                        Span::new(source, 91, 95).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "18.0".parse().unwrap(),
                                        Span::new(source, 96, 100).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "19.0".parse().unwrap(),
                                        Span::new(source, 101, 105).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "20.0".parse().unwrap(),
                                        Span::new(source, 106, 110).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 111).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 114).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 114).unwrap(),
            })],
            Span::new(source, 0, 114).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_one_nested_array_of_float() {
        let source = "~main<argc><argv>{<[1.0,[2.0,3.0]];>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(source, 20, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(source, 25, 28).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(source, 29, 32).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 24, 33).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 34).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 37).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 37).unwrap(),
            })],
            Span::new(source, 0, 37).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_tow_nested_array_of_float() {
        let source = "~main<argc><argv>{<[1.0,[2.0,3.0],[4.0,5.0]];>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(source, 20, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(source, 25, 28).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(source, 29, 32).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 24, 33).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "4.0".parse().unwrap(),
                                                    Span::new(source, 35, 38).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "5.0".parse().unwrap(),
                                                    Span::new(source, 39, 42).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 34, 43).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 44).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 47).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 47).unwrap(),
            })],
            Span::new(source, 0, 47).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_long_depath_of_array_float() {
        let source = "~main<argc><argv>{<[1.0,[2.0,3.0],[4.0,5.0],[6.0,7.0],[8.0,9.0]];>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(source, 20, 23).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(source, 25, 28).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(source, 29, 32).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 24, 33).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "4.0".parse().unwrap(),
                                                    Span::new(source, 35, 38).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "5.0".parse().unwrap(),
                                                    Span::new(source, 39, 42).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 34, 43).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "6.0".parse().unwrap(),
                                                    Span::new(source, 45, 48).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "7.0".parse().unwrap(),
                                                    Span::new(source, 49, 52).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 44, 53).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "8.0".parse().unwrap(),
                                                    Span::new(source, 55, 58).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "9.0".parse().unwrap(),
                                                    Span::new(source, 59, 62).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 54, 63).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 64).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 67).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 67).unwrap(),
            })],
            Span::new(source, 0, 67).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_of_string() {
        let source = r#"~main<argc><argv>{<["hello","world"];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "hello".to_owned(),
                                        Span::new(source, 20, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "world".to_owned(),
                                        Span::new(source, 28, 35).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 36).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 39).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 39).unwrap(),
            })],
            Span::new(source, 0, 39).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_big_array_of_string() {
        let source =
            r#"~main<argc><argv>{<["hello","everybody","how","are","you","doing","today","?"];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "hello".to_owned(),
                                        Span::new(source, 20, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "everybody".to_owned(),
                                        Span::new(source, 28, 39).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "how".to_owned(),
                                        Span::new(source, 40, 45).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "are".to_owned(),
                                        Span::new(source, 46, 51).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "you".to_owned(),
                                        Span::new(source, 52, 57).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "doing".to_owned(),
                                        Span::new(source, 58, 65).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "today".to_owned(),
                                        Span::new(source, 66, 73).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "?".to_owned(),
                                        Span::new(source, 74, 77).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 78).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 81).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 81).unwrap(),
            })],
            Span::new(source, 0, 81).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_one_nested_array_of_string() {
        let source = r#"~main<argc><argv>{<["hello",["world"]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "hello".to_owned(),
                                        Span::new(source, 20, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "world".to_owned(),
                                                Span::new(source, 29, 36).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 28, 37).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 38).unwrap(),
                        )),
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
    fn test_array_with_tow_nested_array_of_string() {
        let source = r#"~main<argc><argv>{<["hello",["world"],["!"]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "hello".to_owned(),
                                        Span::new(source, 20, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "world".to_owned(),
                                                Span::new(source, 29, 36).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 28, 37).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "!".to_owned(),
                                                Span::new(source, 39, 42).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 38, 43).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 44).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 47).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 47).unwrap(),
            })],
            Span::new(source, 0, 47).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_long_depath_of_array_string() {
        let source = r#"~main<argc><argv>{<["hello",["world",["!"]]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "hello".to_owned(),
                                        Span::new(source, 20, 27).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::String(
                                                    "world".to_owned(),
                                                    Span::new(source, 29, 36).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![ExpressionStatement::Value(
                                                        ValueExpression::Object(
                                                            ObjectExpression::String(
                                                                "!".to_owned(),
                                                                Span::new(source, 38, 41).unwrap(),
                                                            ),
                                                        ),
                                                    )],
                                                    Span::new(source, 37, 42).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 28, 43).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 44).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 47).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 47).unwrap(),
            })],
            Span::new(source, 0, 47).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_of_nil() {
        let source = r#"~main<argc><argv>{<[nil,nil];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(source, 20, 23).unwrap()),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(source, 24, 27).unwrap()),
                                )),
                            ],
                            Span::new(source, 19, 28).unwrap(),
                        )),
                    ))],
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
    fn test_array_with_one_nested_array_of_nil() {
        let source = r#"~main<argc><argv>{<[nil,[nil]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(source, 20, 23).unwrap()),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(
                                                Span::new(source, 25, 28).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 24, 29).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 30).unwrap(),
                        )),
                    ))],
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
    fn test_array_with_tow_nested_array_of_nil() {
        let source = r#"~main<argc><argv>{<[nil,[nil],[nil]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(source, 20, 23).unwrap()),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(
                                                Span::new(source, 25, 28).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 24, 29).unwrap(),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(
                                                Span::new(source, 31, 34).unwrap(),
                                            ),
                                        ))],
                                        Span::new(source, 30, 35).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 36).unwrap(),
                        )),
                    ))],
                    span: Span::new(source, 17, 39).unwrap(),
                },
                visibility: Visibility::Private,
                span: Span::new(source, 0, 39).unwrap(),
            })],
            Span::new(source, 0, 39).unwrap(),
        );
        assert_eq!(ast.unwrap(), program);
    }

    #[test]
    fn test_array_with_long_depath_of_array_nil() {
        let source = r#"~main<argc><argv>{<[nil, [nil, [nil]]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(source, 20, 23).unwrap()),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Nil(
                                                    Span::new(source, 26, 29).unwrap(),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![ExpressionStatement::Value(
                                                        ValueExpression::Object(
                                                            ObjectExpression::Nil(
                                                                Span::new(source, 32, 35).unwrap(),
                                                            ),
                                                        ),
                                                    )],
                                                    Span::new(source, 31, 36).unwrap(),
                                                ),
                                            )),
                                        ],
                                        Span::new(source, 25, 37).unwrap(),
                                    ),
                                )),
                            ],
                            Span::new(source, 19, 38).unwrap(),
                        )),
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
}
