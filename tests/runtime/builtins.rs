use ocypode_lang::{ast::*, runtime::builtins::functions::*};

#[cfg(test)]
mod format {
    use super::*;

    #[test]
    fn test_one_empty_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String("Hello World".to_owned(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_tow_empty_placeholder() {
        let args =
            vec![
                ObjectExpression::String("Hello {}{}".to_owned(), Span { start: 0, end: 0 }),
                ObjectExpression::Array(
                    vec![
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                        )),
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("!".to_owned(), Span { start: 0, end: 0 }),
                        )),
                    ],
                    Span { start: 0, end: 0 },
                ),
            ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String("Hello World!".to_owned(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_one_index_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {0}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String("Hello World".to_owned(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_tow_index_placeholder() {
        let args =
            vec![
                ObjectExpression::String("Hello {0}{1}".to_owned(), Span { start: 0, end: 0 }),
                ObjectExpression::Array(
                    vec![
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                        )),
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("!".to_owned(), Span { start: 0, end: 0 }),
                        )),
                    ],
                    Span { start: 0, end: 0 },
                ),
            ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String("Hello World!".to_owned(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_one_index_placeholder_with_empty_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {0}, yes {}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String(
                "Hello World, yes World".to_owned(),
                Span { start: 0, end: 0 }
            )
        );
    }

    #[test]
    fn test_one_empty_placeholder_with_index_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {}, yes {0}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String(
                "Hello World, yes World".to_owned(),
                Span { start: 0, end: 0 }
            )
        );
    }

    #[test]
    fn test_one_index_placeholder_with_empty_placeholder_with_index_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {0}, yes {}{1}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "World".to_owned(),
                        Span { start: 0, end: 0 },
                    ))),
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "!".to_owned(),
                        Span { start: 0, end: 0 },
                    ))),
                ],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::String(
                "Hello World, yes World!".to_owned(),
                Span { start: 0, end: 0 }
            )
        );
    }

    #[test]
    fn test_invalid_index_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {0}, yes {1}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 });
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_empty_placeholder() {
        let args = vec![
            ObjectExpression::String("Hello {}, yes {}".to_owned(), Span { start: 0, end: 0 }),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            ),
        ];
        let result = format(args, Span { start: 0, end: 0 });
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod len {
    use super::*;

    #[test]
    fn test_empty_string() {
        let args = vec![ObjectExpression::String(
            "".to_owned(),
            Span { start: 0, end: 0 },
        )];
        let result = len(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Int(0.into(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_string() {
        let args = vec![ObjectExpression::String(
            "Hello World".to_owned(),
            Span { start: 0, end: 0 },
        )];
        let result = len(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Int(11.into(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_empty_array() {
        let args = vec![ObjectExpression::Array(vec![], Span { start: 0, end: 0 })];
        let result = len(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Int(0.into(), Span { start: 0, end: 0 })
        );
    }

    #[test]
    fn test_array() {
        let args = vec![ObjectExpression::Array(
            vec![
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "Hello".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "World".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
            ],
            Span { start: 0, end: 0 },
        )];
        let result = len(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Int(2.into(), Span { start: 0, end: 0 })
        );
    }
}

#[cfg(test)]
mod push {
    use super::*;

    #[test]
    fn test_empty_array() {
        let args = vec![
            ObjectExpression::Array(vec![], Span { start: 0, end: 0 }),
            ObjectExpression::String("Hello".to_owned(), Span { start: 0, end: 0 }),
        ];
        let result = push(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("Hello".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            )
        );
    }

    #[test]
    fn test_array() {
        let args =
            vec![
                ObjectExpression::Array(
                    vec![
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("Hello".to_owned(), Span { start: 0, end: 0 }),
                        )),
                        ExpressionStatement::Value(ValueExpression::Object(
                            ObjectExpression::String("World".to_owned(), Span { start: 0, end: 0 }),
                        )),
                    ],
                    Span { start: 0, end: 0 },
                ),
                ObjectExpression::String("!".to_owned(), Span { start: 0, end: 0 }),
            ];
        let result = push(args, Span { start: 0, end: 0 }).unwrap();
        assert_eq!(
            result,
            ObjectExpression::Array(
                vec![
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "Hello".to_owned(),
                        Span { start: 0, end: 0 }
                    ),)),
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "World".to_owned(),
                        Span { start: 0, end: 0 }
                    ),)),
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "!".to_owned(),
                        Span { start: 0, end: 0 }
                    ),)),
                ],
                Span { start: 0, end: 0 },
            )
        );
    }
}

#[cfg(test)]
mod pop {
    use super::*;

    #[test]
    fn test_empty_array() {
        let args = vec![ObjectExpression::Array(vec![], Span { start: 0, end: 0 })];
        let result = pop(args, Span { start: 0, end: 0 });
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ObjectExpression::Array(vec![], Span { start: 0, end: 0 },)
        )
    }

    #[test]
    fn test_array() {
        let args = vec![ObjectExpression::Array(
            vec![
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "Hello".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "World".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
            ],
            Span { start: 0, end: 0 },
        )];
        let result = pop(args, Span { start: 0, end: 0 });
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ObjectExpression::Array(
                vec![ExpressionStatement::Value(ValueExpression::Object(
                    ObjectExpression::String("Hello".to_owned(), Span { start: 0, end: 0 }),
                ))],
                Span { start: 0, end: 0 },
            )
        );
    }

    #[test]
    fn test_array_with_multiple_expressions() {
        let args = vec![ObjectExpression::Array(
            vec![
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "Hello".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "World".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
                ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                    "!".to_owned(),
                    Span { start: 0, end: 0 },
                ))),
            ],
            Span { start: 0, end: 0 },
        )];
        let result = pop(args, Span { start: 0, end: 0 });
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ObjectExpression::Array(
                vec![
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "Hello".to_owned(),
                        Span { start: 0, end: 0 }
                    ),)),
                    ExpressionStatement::Value(ValueExpression::Object(ObjectExpression::String(
                        "World".to_owned(),
                        Span { start: 0, end: 0 }
                    ),)),
                ],
                Span { start: 0, end: 0 },
            )
        );
    }
}
