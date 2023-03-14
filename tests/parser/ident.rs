use ocypode_lang::ast::*;
use ocypode_lang::errors::ErrorKind;
use ocypode_lang::parser::OYParser;
use pest::Span;

#[test]
fn test_single_ident() {
    let source = "~main<argc><argv>{<ident;>}";
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
                        ident: "ident".to_owned(),
                        span: Span::new(source, 19, 24).unwrap(),
                    }),
                ))],
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
fn test_ident_with_underscore() {
    let source = "~main<argc><argv>{<some_ident;>}";
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
                        ident: "some_ident".to_owned(),
                        span: Span::new(source, 19, 29).unwrap(),
                    }),
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
fn test_ident_start_with_underscore() {
    let source = "~main<argc><argv>{<_ident;>}";
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
                        ident: "_ident".to_owned(),
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

#[test]
fn test_ident_end_with_underscore() {
    let source = "~main<argc><argv>{<ident_;>}";
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
                        ident: "ident_".to_owned(),
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

#[test]
fn test_ident_with_multiple_underscore() {
    let source = "~main<argc><argv>{<some_ident_;>}";
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
                        ident: "some_ident_".to_owned(),
                        span: Span::new(source, 19, 30).unwrap(),
                    }),
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
fn test_ident_with_number() {
    let source = "~main<argc><argv>{<some_ident1;>}";
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
                        ident: "some_ident1".to_owned(),
                        span: Span::new(source, 19, 30).unwrap(),
                    }),
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
fn test_ident_with_underscore_and_number() {
    let source = "~main<argc><argv>{<some_ident_1;>}";
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
                        ident: "some_ident_1".to_owned(),
                        span: Span::new(source, 19, 31).unwrap(),
                    }),
                ))],
                span: Span::new(source, 17, 34).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 34).unwrap(),
        })],
        Span::new(source, 0, 34).unwrap(),
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
                        ident: "some_ident_1_2".to_owned(),
                        span: Span::new(source, 19, 33).unwrap(),
                    }),
                ))],
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
fn test_ident_with_underscore_and_tow_number_and_underscore() {
    let source = "~main<argc><argv>{<some_ident_1_2_;>}";
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
                        ident: "some_ident_1_2_".to_owned(),
                        span: Span::new(source, 19, 34).unwrap(),
                    }),
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
fn test_ident_with_underscore_and_tow_number_and_underscore_and_number() {
    let source = "~main<argc><argv>{<some_ident_1_2_3;>}";
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
                        ident: "some_ident_1_2_3".to_owned(),
                        span: Span::new(source, 19, 35).unwrap(),
                    }),
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
