//
// Copyright 2021 Lim Chia Hau.
//
// Licensed under the GNU GENERAL PUBLIC LICENSE Version 3 <LICENSE or
// https://www.gnu.org/licenses/gpl-3.0.en.html>. This file may not be copied, 
// modified, or distributed except according to those terms.
//


extern crate suspenders;


use suspenders::{on,off,status};
use suspenders::{Flag};
use std::env;


fn main() {
    let flag = get_flag();
    
    match flag {
        None => println!("The first argument must be either 'on, off, or status'"),
        Some(Flag::On) => on(),
        Some(Flag::Off) => off(),
        Some(Flag::Status) => status(),
    }
}

fn get_flag() -> Option<Flag> {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1) {
        Some(arg) => get_flag_helper(arg),
        None => None,
    }
}

fn get_flag_helper(arg: &str) -> Option<Flag> {
    match arg {
        "on" => Some(Flag::On),
        "off" => Some(Flag::Off),
        "status" => Some(Flag::Status),
        _ => None,
    }    
}
