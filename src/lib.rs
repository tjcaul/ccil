/*
lib.rs: The CCIL Crate
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

use clap::Parser as ArgParser;

pub mod parser;
pub mod compiler;
pub mod vm;
pub mod constants;

pub fn version() -> (u8, u8, u8) {
    (
        u8::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10).unwrap(),
        u8::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10).unwrap(),
        u8::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10).unwrap()
    )
}

/// The CCIL programming language.
#[derive(ArgParser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of ccil source file
    #[arg(default_value_t = String::new())]
    pub input_path: String,

    /// Whether or not to print compiler information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {
        let args = <$crate::Args as clap::Parser>::parse();
        if args.debug {
            print!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        let args = <$crate::Args as clap::Parser>::parse();
        if args.debug {
            println!($($arg)*);
        }
    };
}
