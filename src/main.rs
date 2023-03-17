use std::{fs, process};

use ocypode_lang::{parser::OYParser, runtime::interpreter::Interpreter};

fn main() {
    // FIXME: Use clap, structopt or something else to parse arguments
    
    let mut args = std::env::args().skip(1);
    let file = args.next().unwrap_or_else(|| {
        eprintln!("Usage: ocypode <file>");
        process::exit(1);
    });
    let source = fs::read_to_string(&file).unwrap_or_else(|_| {
        eprintln!("Could not read file '{}'", file);
        process::exit(1);
    });

    let program =
        match OYParser::parse_program(&source).map_err(|e| e.as_diagnostic(&source, &file)) {
            Ok(parser) => parser,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
    let exit_code = Interpreter::new()
        .interpret(program, args.len(), args.collect())
        .unwrap_or_else(|e| {
            eprintln!("{}", e.as_diagnostic(source, file));
            process::exit(1);
        });
    process::exit(exit_code as i32);
}
