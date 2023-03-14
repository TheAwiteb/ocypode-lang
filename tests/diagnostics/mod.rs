//! Tests for all Ocypode diagnostics.
//! This module contains tests for all diagnostics that Ocypode can produce. To add a new test, add a
//! new file to this module with the name of the diagnostic you want to test with the extension `.txt`. The file should contain the diagnostic.
//! And create a file with the same name as the diagnostic, but with the extension `.oy`.
//! This file should contain the source code that will trigger the diagnostic.
//! Lastly, add the name of the diagnostic to the `test_diagnostics` macro. This macro is used to create all tests.
//!
//! Note: The diagnostic is without a color code, so it is not colored. Make sure when you want to add or test a diagnostic that it is not colored.
//! Run with `NO_COLOR=1` environment variable to disable colors.

/// Create a test for each diagnostic
macro_rules! test_diagnostics {
    ($($diagnostic:ident)+) => {
        $(
            #[test]
            fn $diagnostic() {
                let source = std::fs::read_to_string(concat!("tests/diagnostics/", stringify!($diagnostic), ".oy")).unwrap();
                let expected = std::fs::read_to_string(concat!("tests/diagnostics/", stringify!($diagnostic), ".txt")).unwrap();
                let program = ocypode_lang::parser::OYParser::parse_program(&source);
                if let Err(err) = program {
                    let diagnostic = err.as_diagnostic(&source, concat!("tests/diagnostics/", stringify!($diagnostic), ".oy"));
                    assert_eq!(format!("{}", diagnostic), expected);
                } else {
                    panic!("The program has no errors, but it should have one.")
                }
            }
        )+
    };
}

test_diagnostics!(
    invalid_function_name
    invalid_variable_name
    invalid_parameter_name
    main_function_contains_no_parameters
    main_function_contains_more_than_tow_parameters
    main_function_contains_one_parameter
    main_function_contains_one_invalid_parameter
    main_function_contains_tow_invalid_parameters
    main_function_invalid_second_parameter
    main_function_cannot_be_public
);
