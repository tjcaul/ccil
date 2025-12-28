use std::{fs::read_to_string, io::{self, Write}, process::exit};

use ccil::parser::{Parser, token::Token};
use clap::Parser as ArgParser;

/// The CCIL programming language.
#[derive(ArgParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of ccil source file
    #[arg(default_value_t = String::new())]
    input_path: String,
}

fn repl() -> ! {
    loop {
        print!("ccil> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {},
            Err(_) => { continue; }
        };
        let tokenization_result = Token::full_scan(&buffer);
        let mut parser = Parser::new(tokenization_result);
        parser.full_parse();
        for expr in parser.expressions {
            println!("{:?}", expr);
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.input_path.is_empty() {
        repl();
    }
    let source_file = match read_to_string(args.input_path) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Failed to read input file: {}", error);
            exit(1);
        }
    };

    let scan_result = Token::full_scan(&source_file);
    let mut parser = Parser::new(scan_result);
    parser.full_parse();
    for expr in parser.expressions {
        println!("{:?}", expr);
    }
}
