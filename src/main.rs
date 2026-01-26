/*
main.rs: The CCIL Runtime
Copyright (C) 2025-26 Tyson Caul and Ray Chen

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{fs::read_to_string, io::{self, Write}, process::exit};

use ccil::{constants::GPL_REPL_NOTICE, parser::{Parser, token::Token}};
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
    println!("{}", GPL_REPL_NOTICE);
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
