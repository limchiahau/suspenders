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
