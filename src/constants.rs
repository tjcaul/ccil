/*
constants.rs: The CCIL constants file
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

use const_format::formatcp;

pub const CCIL_MAGIC_BYTE_0: u8 = 0xCC;
pub const CCIL_MAGIC_BYTE_1: u8 = 0x17;

pub const BYTECODE_HEADER_SIZE: usize = 16;

pub const DISASSEMBLER_METADATA_BORDER_LINE: &str = "// -----------------------------------------------------------\n";

pub const GPL_REPL_NOTICE: &str = formatcp!("The Caul-Chen Interpreted Language, Version {}
Copyright (C) 2025-26 The CCIL Developers
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under certain conditions.\n", env!("CARGO_PKG_VERSION"));