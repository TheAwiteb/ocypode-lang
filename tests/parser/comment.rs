use ocypode_lang::ast::*;
use ocypode_lang::parser::OYParser;
use pest::Span;

#[test]
fn test_single_line_comment() {
    let source = "// This is a comment\n~main<argc>/*Block comment*/<argv>{<>}//Another comment";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());
    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "main".to_owned(),
                span: Span::new(source, 22, 26).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 27, 31).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 50, 54).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: Vec::new(),
                span: Span::new(source, 55, 59).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 21, 59).unwrap(),
        })],
        Span::new(source, 0, 76).unwrap(),
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
                span: Span::new(source, 50, 54).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 55, 59).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 61, 65).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: Vec::new(),
                span: Span::new(source, 66, 70).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 49, 70).unwrap(),
        })],
        Span::new(source, 0, 117).unwrap(),
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
                span: Span::new(source, 18, 22).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 23, 27).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 29, 33).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: Vec::new(),
                span: Span::new(source, 34, 38).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 17, 38).unwrap(),
        })],
        Span::new(source, 0, 38).unwrap(),
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
                statements: Vec::new(),
                span: Span::new(source, 17, 21).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 21).unwrap(),
        })],
        Span::new(source, 0, 38).unwrap(),
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
                span: Span::new(source, 18, 22).unwrap(),
            },
            params: vec![
                Param {
                    ident: Ident {
                        ident: "argc".to_owned(),
                        span: Span::new(source, 23, 27).unwrap(),
                    },
                },
                Param {
                    ident: Ident {
                        ident: "argv".to_owned(),
                        span: Span::new(source, 29, 33).unwrap(),
                    },
                },
            ],
            block: Block {
                statements: Vec::new(),
                span: Span::new(source, 34, 38).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 17, 38).unwrap(),
        })],
        Span::new(source, 0, 62).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}
