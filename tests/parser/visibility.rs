use ocypode_lang::{ast::*, errors::ErrorKind, parser::OYParser};

#[test]
fn test_public_visibility() {
    let source = "^~foo{<>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "foo".to_owned(),
                span: Span::new(2, 5),
            },
            params: vec![],
            block: Some(Block {
                statements: vec![],
                span: Span::new(5, 9),
            }),
            visibility: Visibility::Public,
            span: Span::new(0, 9),
        })],
        pest::Span::new(source, 0, 9).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_private_visibility() {
    let source = "~foo{<>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "foo".to_owned(),
                span: Span::new(1, 4),
            },
            params: vec![],
            block: Some(Block {
                statements: vec![],
                span: Span::new(4, 8),
            }),
            visibility: Visibility::Private,
            span: Span::new(0, 8),
        })],
        pest::Span::new(source, 0, 8).unwrap(),
    );

    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_double_public_visibility() {
    let source = "^^~foo{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}
