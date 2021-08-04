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

use crate::menu;
use crate::game_core;

use std::fs::{OpenOptions, File};
use std::fs::{read_dir};
use std::ffi::{OsStr, OsString};
use std::io::{self, Write, BufRead};
use std::path::PathBuf;
use std::process::exit;

pub fn create_savefile() {
    let mut file_name = menu::verify_player_name();
    file_name.push_str(".sav");

    let _file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name);

}

pub fn save(save_file: String, level: i32) {
    panic!("Saving not implemented");
}

fn ask_for_savefile() -> OsString {
    let directory = read_dir(".").unwrap();
    let mut files = vec![];
    // Resize vector to fit 255 save files
    files.resize(255, PathBuf::new());

    let mut num_files = 0;

    println!("Please pick a savefile number:");

    // Find all files in the current directory that end in .sav, print them and put them to the vector
    for file in directory {
        let file_path = file.unwrap().path();
        if file_path.extension() == Some(OsStr::new("sav")) {
            println!("{}: {}", num_files, file_path.display());
            files[num_files] = file_path;
            num_files += 1;
        }
    }

    let mut picked_save = false;
    let mut save_choice = 0;

    while !picked_save {
        save_choice = game_core::ask_for_int_input() as usize;
        if save_choice < num_files && save_choice >= 0 {
            println!("\nAre you sure you want to load {}? Type 1 if you're sure.", files[save_choice].display());
            let confirm = game_core::ask_for_int_input();
            if confirm == 1 {
                picked_save = true;
            }
        }
    }

    let file_name = OsString::from(files[save_choice].file_name().unwrap());

    file_name
}

pub fn load_savefile() {
    panic!("Loading not implemented");
}