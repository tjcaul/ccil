use std::{fs, process::exit};

use clap::Parser;

use crate::constants::{BYTECODE_HEADER_SIZE, CCIL_MAGIC_BYTE_0, CCIL_MAGIC_BYTE_1};
use crate::vm::{chunk::Chunk, opcode::OpCode, opcode::OpCodeLookup};
use crate::vm::stack::StackPointer;

mod vm;
mod constants;

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
        let (major, minor, patch) = match std::env::var("CARGO_PKG_VERSION") {
            Ok(val) => {
                // ugly as fuck, but this takes our semver as a string and maps it to three u8s
                let ver_vec = val.split(".")
                                          .map(|x| match x.parse::<u8>() {
                                              Ok(y) => y,
                                              Err(_) => panic!("Error parsing version number when building header")
                                        }).collect::<Vec<u8>>();
                assert_eq!(ver_vec.len(), 3);
                (ver_vec[0], ver_vec[1], ver_vec[2])
            },
            Err(_) => {
                eprintln!("Error fetching version number when building header");
                exit(1);
            }
        };

        // header is 16 bytes and consists of magic value 0xCC17, followed by big endian semver,
        // and rest is reserved for now 
        let mut header: Vec<u8> = vec![CCIL_MAGIC_BYTE_0, CCIL_MAGIC_BYTE_1, major, minor, patch];
        for _ in 0..BYTECODE_HEADER_SIZE-header.len() {
            header.push(u8::MIN);
        }
        header.append(&mut chunk);
        header.to_file(&args.output_path);
    }

    exit(0);
}
