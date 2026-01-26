/*
vm/stack.rs: Defines a stack and its operations
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

use std::fmt;

pub type StackPointer = i32;
pub type StackItem = i8;

pub trait Shift<T> {
    fn logical_shift(self, shift_amount: T) -> Self;
    fn arithmetic_shift(self, shift_amount: T) -> Self;
}

impl Shift<StackItem> for i8 {
    fn logical_shift(self, shift_amount: StackItem) -> Self {
        assert!(shift_amount >= 0);

        if shift_amount == 0 {
            return self;
        }

        // We know shift_amount >= 1, so MSB will be 0 after shift, so
        // u8->i8 conversion is safe.
        return ((self as u8) >> shift_amount).try_into().unwrap();
    }

    fn arithmetic_shift(self, shift_amount: StackItem) -> Self {
        return (self as i8) >> (shift_amount as usize) as Self;
    }
}

pub trait Stack {
    fn new() -> Self;
    fn get(&self, offset: StackPointer) -> StackItem;
    fn insert(&mut self, offset: StackPointer, item: StackItem);
    fn set(&mut self, offset: StackPointer, item: StackItem);
    fn push(&mut self, item: StackItem);
    fn pop(&mut self) -> Option<StackItem>;
}

pub struct VecStack {
    items: Vec<StackItem>
}

impl Stack for VecStack {
    fn new() -> Self {
        let items = Vec::<StackItem>::new();
        return Self { items };
    }

    fn get(&self, offset: StackPointer) -> StackItem {
        let index = self.items.len() - 1 - offset as usize;
        return self.items[index];
    }

    fn insert(&mut self, offset: StackPointer, item: StackItem) {
        let index = self.items.len() - offset as usize;
        self.items.insert(index, item);
    }

    fn set(&mut self, offset: StackPointer, item: StackItem) {
        let index = self.items.len() - 1 - offset as usize;
        self.items[index] = item;
    }

    fn push(&mut self, item: StackItem) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<StackItem> {
        return self.items.pop();
    }
}

impl fmt::Debug for VecStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.items.fmt(f)
    }
}
