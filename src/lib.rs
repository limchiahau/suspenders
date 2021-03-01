//
// Copyright 2021 Lim Chia Hau.
//
// Licensed under the GNU GENERAL PUBLIC LICENSE Version 3 <LICENSE or
// https://www.gnu.org/licenses/gpl-3.0.en.html>. This file may not be copied,
// modified, or distributed except according to those terms.
//

use std::io::Result;
use std::process::Command;
use std::process::Output;

pub enum Flag {
    On,
    Off,
    Status,
}

pub fn on() {
    match run(Flag::On) {
        Ok(_) => println!("You are now able to suspend your computer"),
        Err(e) => print_error(e),
    }
}

pub fn off() {
    match run(Flag::Off) {
        Ok(_) => println!("Your computer no longer able to suspend"),
        Err(e) => print_error(e),
    }
}

pub fn status() {
    match run(Flag::Status) {
        Ok(output) => print_status(output.stdout),
        Err(e) => print_error(e),
    }
}

fn run(f: Flag) -> Result<Output> {
    Command::new("sudo")
        .arg("systemctl")
        .arg(match f {
            Flag::On => "unmask",
            Flag::Off => "mask",
            Flag::Status => "status",
        })
        .arg("suspend.target")
        .output()
}

fn print_error(e: std::io::Error) {
    println!("Opps and error has occured.\n {}", e);
}

fn print_status(status: Vec<u8>) {
    match String::from_utf8(status) {
        Ok(status) => {
            if status.contains("Loaded: masked") {
                println!("Suspend Disabled")
            } else {
                println!("Suspend Enabled")
            }
        }
        Err(_) => (),
    }
}
