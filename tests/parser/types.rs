use ocypode_lang::ast::*;
use ocypode_lang::errors::ErrorKind;
use ocypode_lang::parser::OYParser;

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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Nil(Span::new(19, 22))),
                    ))],
                    span: Span::new(17, 25),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 25),
            })],
            pest::Span::new(source, 0, 25).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Ident(Ident {
                            ident: "nilsome".to_owned(),
                            span: Span::new(19, 26),
                        }),
                    ))],
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
    fn test_nil_as_suffix() {
        let source = "~main<argc><argv>{<somnil;>}";
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Ident(Ident {
                            ident: "somnil".to_owned(),
                            span: Span::new(19, 25),
                        }),
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Bool(true, Span::new(19, 23)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Bool(false, Span::new(24, 29)),
                        ))),
                    ],
                    span: Span::new(17, 32),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 32),
            })],
            pest::Span::new(source, 0, 32).unwrap(),
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "truesome".to_owned(),
                                span: Span::new(19, 27),
                            },
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "falsesome".to_owned(),
                                span: Span::new(28, 37),
                            },
                        ))),
                    ],
                    span: Span::new(17, 40),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 40),
            })],
            pest::Span::new(source, 0, 40).unwrap(),
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "sometrue".to_owned(),
                                span: Span::new(19, 27),
                            },
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Ident(
                            Ident {
                                ident: "somefalse".to_owned(),
                                span: Span::new(28, 37),
                            },
                        ))),
                    ],
                    span: Span::new(17, 40),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 40),
            })],
            pest::Span::new(source, 0, 40).unwrap(),
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
                    statements: vec![
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(0.into(), Span::new(19, 20)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(1.into(), Span::new(21, 22)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(2.into(), Span::new(23, 24)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(3.into(), Span::new(25, 26)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(4.into(), Span::new(27, 28)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(5.into(), Span::new(29, 30)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(6.into(), Span::new(31, 32)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(7.into(), Span::new(33, 34)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(8.into(), Span::new(35, 36)),
                        ))),
                        Statement::Expression(ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::Int(9.into(), Span::new(37, 38)),
                        ))),
                    ],
                    span: Span::new(17, 41),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 41),
            })],
            pest::Span::new(source, 0, 41).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Int(
                            1234567890.into(),
                            Span::new(19, 29),
                        )),
                    ))],
                    span: Span::new(17, 32),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 32),
            })],
            pest::Span::new(source, 0, 32).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Int(
                            "123456789012345678901234567890".parse().unwrap(),
                            Span::new(19, 49),
                        )),
                    ))],
                    span: Span::new(17, 52),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 52),
            })],
            pest::Span::new(source, 0, 52).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "0.0".parse().unwrap(),
                            Span::new(19, 22),
                        )),
                    ))],
                    span: Span::new(17, 25),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 25),
            })],
            pest::Span::new(source, 0, 25).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "1234567890.1234567890".parse().unwrap(),
                            Span::new(19, 40),
                        )),
                    ))],
                    span: Span::new(17, 43),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 43),
            })],
            pest::Span::new(source, 0, 43).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Float(
                            "123456789012345678901234567890.123456789012345678901234567890"
                                .parse()
                                .unwrap(),
                            Span::new(19, 80),
                        )),
                    ))],
                    span: Span::new(17, 83),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 83),
            })],
            pest::Span::new(source, 0, 83).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "".to_owned(),
                            Span::new(19, 21),
                        )),
                    ))],
                    span: Span::new(17, 24),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 24),
            })],
            pest::Span::new(source, 0, 24).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "a".to_owned(),
                            Span::new(19, 22),
                        )),
                    ))],
                    span: Span::new(17, 25),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 25),
            })],
            pest::Span::new(source, 0, 25).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "some".to_owned(),
                            Span::new(19, 25),
                        )),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            r#"some\""#.to_owned(),
                            Span::new(19, 27),
                        )),
                    ))],
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            "some\nmultiline".to_owned(),
                            Span::new(19, 35),
                        )),
                    ))],
                    span: Span::new(17, 38),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 38),
            })],
            pest::Span::new(source, 0, 38).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            r#"some\nmultiline\""#.to_owned(),
                            Span::new(19, 38),
                        )),
                    ))],
                    span: Span::new(17, 41),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 41),
            })],
            pest::Span::new(source, 0, 41).unwrap(),
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
                statements: vec![Statement::Expression(ExpressionStatement::Value(
                    ValueExpression::Object(ObjectExpression::String(
                        "Consequat aliquip reprehenderit ex dolore reprehenderit ut sunt cupidatat aute.Incididunt ex nisi id et.Deserunt aute est sit dolor tempor.".to_owned(),
                        Span::new(19, 160),
                    )),
                ))],
                span: Span::new(17, 163),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 163),
        })],
        pest::Span::new(source, 0, 163).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            lorem.to_owned(),
                            Span::new(19, 19 + lorem.len() + 2),
                        )),
                    ))],
                    span: Span::new(17, 19 + lorem.len() + 5),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 19 + lorem.len() + 5),
            })],
            pest::Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::String(
                            lorem.to_owned(),
                            Span::new(19, 19 + lorem.len() + 2),
                        )),
                    ))],
                    span: Span::new(17, 19 + lorem.len() + 5),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 19 + lorem.len() + 5),
            })],
            pest::Span::new(&source, 0, 19 + lorem.len() + 5).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(vec![], Span::new(19, 21))),
                    ))],
                    span: Span::new(17, 24),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 24),
            })],
            pest::Span::new(source, 0, 24).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![ExpressionStatement::Value(ValueExpression::Object(
                                ObjectExpression::Int(1.into(), Span::new(20, 21)),
                            ))],
                            Span::new(19, 22),
                        )),
                    ))],
                    span: Span::new(17, 25),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 25),
            })],
            pest::Span::new(source, 0, 25).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(20, 21)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(2.into(), Span::new(23, 24)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(3.into(), Span::new(26, 27)),
                                )),
                            ],
                            Span::new(19, 28),
                        )),
                    ))],
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
    fn test_big_array_of_integer() {
        let source = r#"~main<argc><argv>{<[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(20, 21)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(2.into(), Span::new(23, 24)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(3.into(), Span::new(26, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(4.into(), Span::new(29, 30)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(5.into(), Span::new(32, 33)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(6.into(), Span::new(35, 36)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(7.into(), Span::new(38, 39)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(8.into(), Span::new(41, 42)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(9.into(), Span::new(44, 45)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(10.into(), Span::new(47, 49)),
                                )),
                            ],
                            Span::new(19, 50),
                        )),
                    ))],
                    span: Span::new(17, 53),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 53),
            })],
            pest::Span::new(source, 0, 53).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(20, 21)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(2.into(), Span::new(24, 25)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(3.into(), Span::new(27, 28)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(4.into(), Span::new(30, 31)),
                                            )),
                                        ],
                                        Span::new(23, 32),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(5.into(), Span::new(34, 35)),
                                )),
                            ],
                            Span::new(19, 36),
                        )),
                    ))],
                    span: Span::new(17, 39),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 39),
            })],
            pest::Span::new(source, 0, 39).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(1.into(), Span::new(20, 21)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(2.into(), Span::new(24, 25)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(3.into(), Span::new(27, 28)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(4.into(), Span::new(30, 31)),
                                            )),
                                        ],
                                        Span::new(23, 32),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(5.into(), Span::new(34, 35)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(6.into(), Span::new(38, 39)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(7.into(), Span::new(41, 42)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(8.into(), Span::new(44, 45)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(9.into(), Span::new(47, 48)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(10.into(), Span::new(50, 52)),
                                            )),
                                        ],
                                        Span::new(37, 53),
                                    ),
                                )),
                            ],
                            Span::new(19, 54),
                        )),
                    ))],
                    span: Span::new(17, 57),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 57),
            })],
            pest::Span::new(source, 0, 57).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        1.into(),
                                        Span::new(20, 21),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        2.into(),
                                        Span::new(22, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    3.into(),
                                                    Span::new(25, 26),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    4.into(),
                                                    Span::new(27, 28),
                                                ),
                                            )),
                                        ],
                                        Span::new(24, 29),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    323.into(),
                                                    Span::new(31, 34),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Int(
                                                    23.into(),
                                                    Span::new(35, 37),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![
                                                        ExpressionStatement::Value(
                                                            ValueExpression::Object(
                                                                ObjectExpression::Int(
                                                                    23.into(),
                                                                    Span::new( 39, 41)
                                                                ),
                                                            ),
                                                        ),
                                                        ExpressionStatement::Value(
                                                            ValueExpression::Object(
                                                                ObjectExpression::Int(
                                                                    32.into(),
                                                                    Span::new( 42, 44)
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
                                                                                        46,
                                                                                        49,
                                                                                    )
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        ExpressionStatement::Value(
                                                                            ValueExpression::Object(
                                                                                ObjectExpression::Int(
                                                                                    32.into(),
                                                                                    Span::new(
                                                                                        50,
                                                                                        52,
                                                                                    )
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    Span::new(
                                                                        45,
                                                                        54,
                                                                    )
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    Span::new(38, 56),
                                                ),
                                            )),
                                        ],
                                        Span::new(30, 58),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Int(
                                        42.into(),
                                        Span::new(59, 61),
                                    ),
                                )),
                            ],
                            Span::new(19, 62),
                        )),
                    ))],
                    span: Span::new(17, 65),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 65),
            })],
            pest::Span::new(source, 0, 65).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(20, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "2.0".parse().unwrap(),
                                        Span::new(24, 27),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "3.0".parse().unwrap(),
                                        Span::new(28, 31),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "4.0".parse().unwrap(),
                                        Span::new(32, 35),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "5.0".parse().unwrap(),
                                        Span::new(36, 39),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "6.0".parse().unwrap(),
                                        Span::new(40, 43),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "7.0".parse().unwrap(),
                                        Span::new(44, 47),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "8.0".parse().unwrap(),
                                        Span::new(48, 51),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "9.0".parse().unwrap(),
                                        Span::new(52, 55),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "10.0".parse().unwrap(),
                                        Span::new(56, 60),
                                    ),
                                )),
                            ],
                            Span::new(19, 61),
                        )),
                    ))],
                    span: Span::new(17, 64),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 64),
            })],
            pest::Span::new(source, 0, 64).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(20, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "2.0".parse().unwrap(),
                                        Span::new(24, 27),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "3.0".parse().unwrap(),
                                        Span::new(28, 31),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "4.0".parse().unwrap(),
                                        Span::new(32, 35),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "5.0".parse().unwrap(),
                                        Span::new(36, 39),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "6.0".parse().unwrap(),
                                        Span::new(40, 43),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "7.0".parse().unwrap(),
                                        Span::new(44, 47),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "8.0".parse().unwrap(),
                                        Span::new(48, 51),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "9.0".parse().unwrap(),
                                        Span::new(52, 55),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "10.0".parse().unwrap(),
                                        Span::new(56, 60),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "11.0".parse().unwrap(),
                                        Span::new(61, 65),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "12.0".parse().unwrap(),
                                        Span::new(66, 70),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "13.0".parse().unwrap(),
                                        Span::new(71, 75),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "14.0".parse().unwrap(),
                                        Span::new(76, 80),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "15.0".parse().unwrap(),
                                        Span::new(81, 85),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "16.0".parse().unwrap(),
                                        Span::new(86, 90),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "17.0".parse().unwrap(),
                                        Span::new(91, 95),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "18.0".parse().unwrap(),
                                        Span::new(96, 100),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "19.0".parse().unwrap(),
                                        Span::new(101, 105),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "20.0".parse().unwrap(),
                                        Span::new(106, 110),
                                    ),
                                )),
                            ],
                            Span::new(19, 111),
                        )),
                    ))],
                    span: Span::new(17, 114),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 114),
            })],
            pest::Span::new(source, 0, 114).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(20, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(25, 28),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(29, 32),
                                                ),
                                            )),
                                        ],
                                        Span::new(24, 33),
                                    ),
                                )),
                            ],
                            Span::new(19, 34),
                        )),
                    ))],
                    span: Span::new(17, 37),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 37),
            })],
            pest::Span::new(source, 0, 37).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(20, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(25, 28),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(29, 32),
                                                ),
                                            )),
                                        ],
                                        Span::new(24, 33),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "4.0".parse().unwrap(),
                                                    Span::new(35, 38),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "5.0".parse().unwrap(),
                                                    Span::new(39, 42),
                                                ),
                                            )),
                                        ],
                                        Span::new(34, 43),
                                    ),
                                )),
                            ],
                            Span::new(19, 44),
                        )),
                    ))],
                    span: Span::new(17, 47),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 47),
            })],
            pest::Span::new(source, 0, 47).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Float(
                                        "1.0".parse().unwrap(),
                                        Span::new(20, 23),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "2.0".parse().unwrap(),
                                                    Span::new(25, 28),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "3.0".parse().unwrap(),
                                                    Span::new(29, 32),
                                                ),
                                            )),
                                        ],
                                        Span::new(24, 33),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "4.0".parse().unwrap(),
                                                    Span::new(35, 38),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "5.0".parse().unwrap(),
                                                    Span::new(39, 42),
                                                ),
                                            )),
                                        ],
                                        Span::new(34, 43),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "6.0".parse().unwrap(),
                                                    Span::new(45, 48),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "7.0".parse().unwrap(),
                                                    Span::new(49, 52),
                                                ),
                                            )),
                                        ],
                                        Span::new(44, 53),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "8.0".parse().unwrap(),
                                                    Span::new(55, 58),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Float(
                                                    "9.0".parse().unwrap(),
                                                    Span::new(59, 62),
                                                ),
                                            )),
                                        ],
                                        Span::new(54, 63),
                                    ),
                                )),
                            ],
                            Span::new(19, 64),
                        )),
                    ))],
                    span: Span::new(17, 67),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 67),
            })],
            pest::Span::new(source, 0, 67).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("hello".to_owned(), Span::new(20, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("world".to_owned(), Span::new(28, 35)),
                                )),
                            ],
                            Span::new(19, 36),
                        )),
                    ))],
                    span: Span::new(17, 39),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 39),
            })],
            pest::Span::new(source, 0, 39).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("hello".to_owned(), Span::new(20, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String(
                                        "everybody".to_owned(),
                                        Span::new(28, 39),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("how".to_owned(), Span::new(40, 45)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("are".to_owned(), Span::new(46, 51)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("you".to_owned(), Span::new(52, 57)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("doing".to_owned(), Span::new(58, 65)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("today".to_owned(), Span::new(66, 73)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("?".to_owned(), Span::new(74, 77)),
                                )),
                            ],
                            Span::new(19, 78),
                        )),
                    ))],
                    span: Span::new(17, 81),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 81),
            })],
            pest::Span::new(source, 0, 81).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("hello".to_owned(), Span::new(20, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "world".to_owned(),
                                                Span::new(29, 36),
                                            ),
                                        ))],
                                        Span::new(28, 37),
                                    ),
                                )),
                            ],
                            Span::new(19, 38),
                        )),
                    ))],
                    span: Span::new(17, 41),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 41),
            })],
            pest::Span::new(source, 0, 41).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("hello".to_owned(), Span::new(20, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "world".to_owned(),
                                                Span::new(29, 36),
                                            ),
                                        ))],
                                        Span::new(28, 37),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::String(
                                                "!".to_owned(),
                                                Span::new(39, 42),
                                            ),
                                        ))],
                                        Span::new(38, 43),
                                    ),
                                )),
                            ],
                            Span::new(19, 44),
                        )),
                    ))],
                    span: Span::new(17, 47),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 47),
            })],
            pest::Span::new(source, 0, 47).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::String("hello".to_owned(), Span::new(20, 27)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::String(
                                                    "world".to_owned(),
                                                    Span::new(29, 36),
                                                ),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![ExpressionStatement::Value(
                                                        ValueExpression::Object(
                                                            ObjectExpression::String(
                                                                "!".to_owned(),
                                                                Span::new(38, 41),
                                                            ),
                                                        ),
                                                    )],
                                                    Span::new(37, 42),
                                                ),
                                            )),
                                        ],
                                        Span::new(28, 43),
                                    ),
                                )),
                            ],
                            Span::new(19, 44),
                        )),
                    ))],
                    span: Span::new(17, 47),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 47),
            })],
            pest::Span::new(source, 0, 47).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(20, 23)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(24, 27)),
                                )),
                            ],
                            Span::new(19, 28),
                        )),
                    ))],
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
    fn test_array_with_one_nested_array_of_nil() {
        let source = r#"~main<argc><argv>{<[nil,[nil]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(20, 23)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(Span::new(25, 28)),
                                        ))],
                                        Span::new(24, 29),
                                    ),
                                )),
                            ],
                            Span::new(19, 30),
                        )),
                    ))],
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
    fn test_array_with_tow_nested_array_of_nil() {
        let source = r#"~main<argc><argv>{<[nil,[nil],[nil]];>}"#;
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(20, 23)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(Span::new(25, 28)),
                                        ))],
                                        Span::new(24, 29),
                                    ),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![ExpressionStatement::Value(ValueExpression::Object(
                                            ObjectExpression::Nil(Span::new(31, 34)),
                                        ))],
                                        Span::new(30, 35),
                                    ),
                                )),
                            ],
                            Span::new(19, 36),
                        )),
                    ))],
                    span: Span::new(17, 39),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 39),
            })],
            pest::Span::new(source, 0, 39).unwrap(),
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
                    statements: vec![Statement::Expression(ExpressionStatement::Value(
                        ValueExpression::Object(ObjectExpression::Array(
                            vec![
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Nil(Span::new(20, 23)),
                                )),
                                ExpressionStatement::Value(ValueExpression::Object(
                                    ObjectExpression::Array(
                                        vec![
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Nil(Span::new(26, 29)),
                                            )),
                                            ExpressionStatement::Value(ValueExpression::Object(
                                                ObjectExpression::Array(
                                                    vec![ExpressionStatement::Value(
                                                        ValueExpression::Object(
                                                            ObjectExpression::Nil(Span::new(
                                                                32, 35,
                                                            )),
                                                        ),
                                                    )],
                                                    Span::new(31, 36),
                                                ),
                                            )),
                                        ],
                                        Span::new(25, 37),
                                    ),
                                )),
                            ],
                            Span::new(19, 38),
                        )),
                    ))],
                    span: Span::new(17, 41),
                },
                visibility: Visibility::Private,
                span: Span::new(0, 41),
            })],
            pest::Span::new(source, 0, 41).unwrap(),
        );

        assert_eq!(ast.unwrap(), program);
    }
}
