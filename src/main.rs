use std::{fs, process};

use ocypode_lang::{parser::OYParser, runtime::interpreter::Interpreter};

fn main() {
    let mut args = Vec::from(["test.oy".to_owned()]).into_iter();

    let content = fs::read_to_string(args.next().unwrap()).unwrap();
    let ast = OYParser::parse_program(&content);
    match ast {
        Ok(ast) => {
            let exit_code = Interpreter::new()
                .interpret(ast, args.len(), args.collect())
                .expect("The main function should return the exit code");
            process::exit(exit_code)
        }
        Err(err) => println!("{}", err.as_diagnostic(content, "test.oy"))
    }
}
