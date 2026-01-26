/*
bin/assembler.rs: The CCIL assembler
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

use std::{fs, process::exit};

use clap::Parser;

use ccil::vm::{chunk::Chunk, opcode::OpCode, opcode::OpCodeLookup, stack::StackPointer};

/// Quick and dirty assembler for ccil bytecode, supports both writing to file and immediate execution
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of ccil assembly file
    input_path: String,

    /// Flag to execute bytecode rather than writing it, will compile otherwise
    #[arg(short, long, default_value_t = false)]
    execute: bool,

    /// Output file (required if compiling, ignored if executing)
    #[arg(short, long, default_value_t = String::new())]
    output_path: String
}

fn main() {
    let opcode_lookup = OpCodeLookup::new();

    let args = Args::parse();
    
    if !args.execute && args.output_path.is_empty() {
        eprintln!("Output file not specified, run --help for more info");
        exit(1);
    }

    let input_file = match fs::read_to_string(args.input_path) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Failed to read input file: {}", error);
            exit(1);
        }
    };

    let mut chunk = Vec::<u8>::new();
    for (i, line) in input_file.split("\n").enumerate() {
        // strip line of comments and whitespace
        let stripped_line = line[0..line.find("//").unwrap_or(line.len())].trim();
        if stripped_line.is_empty() {
            continue;
        }

        // split on whitespace
        let line_split: Vec<&str> = stripped_line.split_whitespace().collect();

        // get opcode from first value of split (guaranteed to exist)
        let opcode_str = &line_split[0].to_lowercase();

        let line_opcode: &OpCode = match opcode_lookup.from_symbol(opcode_str) {
            Some(opcode) => opcode,
            None => panic!(
                "Error assembling line {}: invalid opcode {}",
                i, opcode_str
            )
        };
        
        let num_args = line_opcode.num_params;
        if num_args != line_split.len() - 1 {
            eprintln!("Error assembling line {}: expected {} args for opcode {} but got {}", i, num_args, opcode_str, line_split.len() - 1);
            exit(1);
        }

        // finally, write instruction
        chunk.write_op(line_opcode);
        for arg in &line_split[1..] {
            let int_arg = match arg.parse::<StackPointer>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error assembling line {}: arg is not a number", i);
                    exit(1);
                }
            };
            chunk.write_arg(int_arg);
        }
    }

    if args.execute {
        chunk.execute(&opcode_lookup);
    } else {
        chunk.with_header(true).to_file(&args.output_path);
    }

    exit(0);
}
