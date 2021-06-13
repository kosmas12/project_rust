/*
    Project Rust - A text-based game written in Rust
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
use std::fs::File;
use std::fs::OpenOptions;

fn flush_stdout() {
    let flush = io::stdout().flush();

        match flush {
            Ok(flush) => flush,
            Err(error) => panic!("Error 1. Couldn't flush buffer. Reason {:?}", error),
        };
}

fn get_input() -> String {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error 2. Couldn't get input.\n");

    // Get rid of newline character
    input.pop();

    input
}


fn ask_for_input() -> String {
    print!("Enter your input: ");
    flush_stdout();

    let input = get_input();

    input

}

fn get_player_name() -> String {
    flush_stdout();

    let name = ask_for_input();

    name
}

fn verify_player_name() -> String {
    let mut name = get_player_name();
    let mut verified = false;

    while !verified {
        println!("So your name is: {}\n", name);

        println!("Is that correct?");
        println!("1. Yes");
        println!("2. No\n");

        let confirm = ask_for_input();
        let confirm_num = confirm.parse::<i32>().unwrap();

        if confirm_num == 2 {
            println!("Please reenter your name");
            name = get_player_name();
        }
        else {
            verified = true;
        }
    }

    name
}

fn create_savefile() {
    let mut file_name = verify_player_name();
    file_name.push_str(".sav");

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name);

}

fn start() {
    // Clear terminal screen
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("Hello and welcome to Project Rust!");
    println!("Please enter your name");
    create_savefile();
}

fn main() {
    start();
}
