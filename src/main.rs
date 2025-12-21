use std::fs::read_to_string;

use crate::parser::token::Token;

mod parser;
mod vm;

fn main() {
    let source_file = read_to_string("ccil_source_files/tokenizer_stress_test.ccil").unwrap();

    let scan_result = Token::full_scan(&source_file);

    println!("{:?}", scan_result);
}
