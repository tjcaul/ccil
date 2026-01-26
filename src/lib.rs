/*
lib.rs: The CCIL Crate
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
