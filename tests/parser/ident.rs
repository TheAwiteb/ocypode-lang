use ocypode_lang::ast::*;
use ocypode_lang::errors::ErrorKind;
use ocypode_lang::parser::OYParser;

#[test]
fn test_single_ident() {
    let source = "~main<argc><argv>{<ident;>}";
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
                        ident: "ident".to_owned(),
                        span: Span::new(19, 24),
                    }),
                ))],
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
fn test_ident_with_underscore() {
    let source = "~main<argc><argv>{<some_ident;>}";
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
                        ident: "some_ident".to_owned(),
                        span: Span::new(19, 29),
                    }),
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
fn test_ident_start_with_underscore() {
    let source = "~main<argc><argv>{<_ident;>}";
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
                        ident: "_ident".to_owned(),
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

#[test]
fn test_ident_end_with_underscore() {
    let source = "~main<argc><argv>{<ident_;>}";
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
                        ident: "ident_".to_owned(),
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

#[test]
fn test_ident_with_multiple_underscore() {
    let source = "~main<argc><argv>{<some_ident_;>}";
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
                        ident: "some_ident_".to_owned(),
                        span: Span::new(19, 30),
                    }),
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
fn test_ident_with_number() {
    let source = "~main<argc><argv>{<some_ident1;>}";
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
                        ident: "some_ident1".to_owned(),
                        span: Span::new(19, 30),
                    }),
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
fn test_ident_with_underscore_and_number() {
    let source = "~main<argc><argv>{<some_ident_1;>}";
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
                        ident: "some_ident_1".to_owned(),
                        span: Span::new(19, 31),
                    }),
                ))],
                span: Span::new(17, 34),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 34),
        })],
        pest::Span::new(source, 0, 34).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_ident_with_underscore_and_tow_number() {
    let source = "~main<argc><argv>{<some_ident_1_2;>}";
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
                        ident: "some_ident_1_2".to_owned(),
                        span: Span::new(19, 33),
                    }),
                ))],
                span: Span::new(17, 36),
            },
            visibility: Visibility::Private,
            span: Span::new(0, 36),
        })],
        pest::Span::new(source, 0, 36).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_ident_with_underscore_and_tow_number_and_underscore() {
    let source = "~main<argc><argv>{<some_ident_1_2_;>}";
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
                        ident: "some_ident_1_2_".to_owned(),
                        span: Span::new(19, 34),
                    }),
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
fn test_ident_with_underscore_and_tow_number_and_underscore_and_number() {
    let source = "~main<argc><argv>{<some_ident_1_2_3;>}";
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
                        ident: "some_ident_1_2_3".to_owned(),
                        span: Span::new(19, 35),
                    }),
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
fn test_invalid_ident_by_space() {
    let source = "~main<argc><argv>{<some ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_start_with_number() {
    let source = "~main<argc><argv>{<1some_ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_dash() {
    let source = "~main<argc><argv>{<some-ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_dot() {
    let source = "~main<argc><argv>{<some.ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_slash() {
    let source = "~main<argc><argv>{<some/ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_backslash() {
    let source = "~main<argc><argv>{<some\\ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_plus() {
    let source = "~main<argc><argv>{<some+ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_star() {
    let source = "~main<argc><argv>{<some*ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_invalid_ident_by_percent() {
    let source = "~main<argc><argv>{<some%ident;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}

#[test]
fn test_reseved_ident() {
    let source = "~main<argc><argv>{<return = some;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));

    let source = "~main<argc><argv>{<nil = some;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));

    let source = "~main<argc><argv>{<true = some;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));

    let source = "~main<argc><argv>{<false = some;>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}
