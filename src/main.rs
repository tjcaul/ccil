use std::fs::read_to_string;

use ccil::parser::token::Token;

fn main() {
    let source_file = read_to_string("examples/ccil_source_files/tokenizer_stress_test.ccil").unwrap();

    let scan_result = Token::full_scan(&source_file);

    println!("{:?}", scan_result);
}
