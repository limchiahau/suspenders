# Suspenders

A utility to turn on/off the suspend behaviour on linux.

## Usage

There are three commands available:
1. on
2. off
3. status

The names of the commands are self explantory.

To run the commands enter this into the terminal:

        suspenders cmd
        
Replace cmd with one of the commands listed above.

## Installation

*This program won't work if `rustup` is not installed on your machine.*

If you are on ubuntu, first you need to run:

        sudo apt install build-essential
        
The Linux Rust installer doesn't check for a compiler toolchain, but seems to assume that you've already got a C linker installed! The best solution is to install the tried-and-true gcc toolchain.[stack-overflow](https://stackoverflow.com/questions/52445961/how-do-i-fix-the-rust-error-linker-cc-not-found-for-debian-on-windows-10) 

Than use cargo to install suspender.

        cargo install --git https://github.com/limchiahau/suspenders.git




