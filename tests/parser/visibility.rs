use ocypode_lang::{ast::*, errors::ErrorKind, parser::OYParser};
use pest::Span;

#[test]
fn test_public_visibility() {
    let source = "^~foo{<>}";
    let ast = OYParser::parse_program(source);
    assert!(ast.is_ok());

    let program = (
        vec![Statement::Function(FunctionStatement {
            ident: Ident {
                ident: "foo".to_owned(),
                span: Span::new(source, 2, 5).unwrap(),
            },
            params: vec![],
            block: Block {
                statements: vec![],
                span: Span::new(source, 5, 9).unwrap(),
            },
            visibility: Visibility::Public,
            span: Span::new(source, 0, 9).unwrap(),
        })],
        Span::new(source, 0, 9).unwrap(),
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
                span: Span::new(source, 1, 4).unwrap(),
            },
            params: vec![],
            block: Block {
                statements: vec![],
                span: Span::new(source, 4, 8).unwrap(),
            },
            visibility: Visibility::Private,
            span: Span::new(source, 0, 8).unwrap(),
        })],
        Span::new(source, 0, 8).unwrap(),
    );
    assert_eq!(ast.unwrap(), program);
}

#[test]
fn test_double_public_visibility() {
    let source = "^^~foo{<>}";
    let ast = OYParser::parse_program(source);
    assert!(matches!(ast.unwrap_err().kind, ErrorKind::Parse(..)));
}
