/*
main.rs: The CCIL Runtime
Copyright (C) 2025-26 The CCIL Developers

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

use std::{cell::RefCell, fs::{self, create_dir_all, exists, read_to_string}, io::{self, Write}, path::Path, process::exit};

use ccil::{Args, compiler::{Compiler, StringPool}, constants::GPL_REPL_NOTICE, dprintln, parser::{Parser, token::Token}, vm::{VirtualMachine, chunk::Chunk}};

fn repl() -> ! {
    println!("{}", GPL_REPL_NOTICE);

    let compiler = Compiler::new();
    let mut vm = VirtualMachine::new(&compiler.string_pool);

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
        for expr in &parser.expressions {
            dprintln!("{:?}", expr);
        }

        let compiled_chunk = compiler.compile(&parser.expressions);
        vm.execute(compiled_chunk);
    }
}

fn to_bytecode_filename(file_path: &Path) -> String {
    let filename = file_path.file_stem().unwrap().to_str().unwrap();
    let parent_dir = file_path.parent().unwrap().to_str().unwrap();

    format!("{}/.ccil/{}.ccilb", parent_dir, filename)
}

fn get_compiled_chunk(file_path: &Path) -> Option<(Vec<u8>, StringPool)> {
    // todo: fix this mess
    let binding = to_bytecode_filename(file_path);
    let bytecode_filename = binding.as_str();
    match exists(bytecode_filename) {
        Ok(true) => {
            // recompile if and only if source file postdates bytecode
            let bytecode_timestamp = fs::metadata(bytecode_filename).unwrap().modified().unwrap();
            let source_file_timestamp = fs::metadata(file_path).unwrap().modified().unwrap();

            if source_file_timestamp > bytecode_timestamp {
                return None;
            }

            let headered_chunk: Vec<u8> = Chunk::from_file(bytecode_filename);

            let binding = &format!("{}.sp", bytecode_filename);
            let string_pool_path = Path::new(binding);
            let string_pool = RefCell::new(fs::read(string_pool_path).unwrap());
            dprintln!("Using already compiled version of {}", bytecode_filename);
            Some((headered_chunk.without_header(), string_pool))
        }
        Ok(false) | Err(_) => None,
    }
}

fn compile_chunk(file_path: &Path) -> (Vec<u8>, StringPool) {
    let compiler = Compiler::new();

    let source_file = match read_to_string(file_path) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Failed to read input file: {}", error);
            exit(1);
        }
    };

    let scan_result = Token::full_scan(&source_file);
    let mut parser = Parser::new(scan_result);
    parser.full_parse();
    for expr in &parser.expressions {
        dprintln!("{:?}", expr);
    }

    let compiled_chunk = compiler.compile(&parser.expressions);

    let bytecode_filename = &to_bytecode_filename(file_path);
    // create .ccil if not exists
    let _ = create_dir_all(format!("{}/.ccil", file_path.parent().unwrap().to_str().unwrap()));
    compiled_chunk.clone().with_header(false).to_file(bytecode_filename);

    let binding = &format!("{}.sp", bytecode_filename);
    let string_pool_path = Path::new(binding);
    compiler.write_string_pool(string_pool_path);

    (compiled_chunk, compiler.string_pool)
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    if args.input_path.is_empty() {
        repl();
    }

    let file_path = Path::new(&args.input_path);
    if file_path.extension().is_none_or(|x| x != "ccil") {
        eprintln!("Path should be .ccil file");
        exit(1);
    }

    let (compiled_chunk, string_pool) = match get_compiled_chunk(file_path) {
        Some((chunk, string_pool)) => (chunk, string_pool),
        None => compile_chunk(file_path)
    };


    let mut vm = VirtualMachine::new(&string_pool);    
    vm.execute(compiled_chunk);
}
