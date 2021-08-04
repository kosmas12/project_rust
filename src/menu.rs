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

use std::process::exit;
use crate::game_core;
use crate::save_files;

pub fn get_new_player_name() -> String {
    game_core::flush_stdout();

    let name = game_core::ask_for_str_input();

    name
}

pub fn verify_player_name() -> String {
    println!("Please enter your name");

    let mut name = get_new_player_name();
    let mut verified = false;

    while !verified {
        println!("So your name is: {}\n", name);

        println!("Is that correct?");
        println!("1. Yes");
        println!("2. No\n");

        let confirm_num = game_core::ask_for_int_input();

        if confirm_num == 2 {
            println!("Please reenter your name");
            name = get_new_player_name();
        }
        else {
            verified = true;
        }
    }

    name
}

pub fn show_menu() {
    println!("Hello and welcome to Project Rust!\n");

    println!("What would you like to do?");
    println!("1. Load existing game save");
    println!("2. Start new game");
    println!("3. Exit");

    let menu_choice = game_core::ask_for_int_input();


    match menu_choice {
        1 => save_files::load_savefile(),
        2 => save_files::create_savefile(),
        _ => exit(0),
    };

}