use ocypode_lang::ast::*;
use ocypode_lang::parser::OYParser;

#[test]
fn test_single_line_comment() {
    let source = "// This is a comment\n~main<argc>/*Block comment*/<argv>{<>}//Another comment";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(22, 26),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(27, 31),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(50, 54),
                    },
                },
            ],
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(55, 59),
            }),
            visibility: Visibility::Private,
            span: Span::new(21, 59),
        })],
        pest::Span::new(source, 0, 76).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_multi_line_block_comment() {
    let source = "/* Hi\nThis is multi line block comment\nIs cool */~main<argc><argv>{<>}/*Another multi line\nblock comment\nIs amazing*/";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(50, 54),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(55, 59),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(61, 65),
                    },
                },
            ],
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(66, 70),
            }),
            visibility: Visibility::Private,
            span: Span::new(49, 70),
        })],
        pest::Span::new(source, 0, 117).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_block_comment_in_beginning() {
    let source = "/*Block comment*/~main<argc><argv>{<>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(18, 22),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(23, 27),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(29, 33),
                    },
                },
            ],
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(34, 38),
            }),
            visibility: Visibility::Private,
            span: Span::new(17, 38),
        })],
        pest::Span::new(source, 0, 38).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_block_comment_in_end() {
    let source = "~main<argc><argv>{<>}/*Block comment*/";
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
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(17, 21),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 21),
        })],
        pest::Span::new(source, 0, 38).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_block_comment_in_beginning_and_end() {
    let source = "/*Block comment*/~main<argc><argv>{<>}\n/*Just a long comment*/";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(18, 22),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(23, 27),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(29, 33),
                    },
                },
            ],
            block: Some(Block {
                statements: Vec::new(),
                span: Span::new(34, 38),
            }),
            visibility: Visibility::Private,
            span: Span::new(17, 38),
        })],
        pest::Span::new(source, 0, 62).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}
