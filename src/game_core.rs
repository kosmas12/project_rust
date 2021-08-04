/*
    This file is part of Project Rust - A text-based game written in Rust
    Copyright (C) 2021 Kosmas Raptis

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

use std::io::{self, Write};

pub fn flush_stdout() {
    let flush = io::stdout().flush();

    match flush {
        Ok(flush) => flush,
        Err(error) => panic!("Error 1. Couldn't flush buffer. Reason {:?}", error),
    };
}

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error 2. Couldn't get input.\n");

    // Get rid of newline character
    input.pop();

    input
}

pub fn ask_for_str_input() -> String {
    print!("Enter your input: ");
    flush_stdout();

    let input = get_input();

    input
}

pub fn ask_for_int_input() -> i32 {
    let input = ask_for_str_input();

    input.parse::<i32>().unwrap()
}
