use clap::{Arg, App, SubCommand};

use linefeed::{Interface, ReadResult, DefaultTerminal};
use std::borrow::Borrow;
use std::ptr::read;
mod reader;
use reader::*;

fn main() {
    let _matches = App::new("ldb")
        .version("0.0.1")
        .author("Leo Oliveira <qwerty124563@gmail.com>")
        .about("A Database developed using Rust.")
        .get_matches();

    let reader = Reader::new();

    while let ReadResult::Input(input) = reader.read_line() {
        reader.save_history(&input);

        println!("got input {:?}", input);
    }

    println!("Goodbye.");

    println!("Hello, world!");
}

fn save_history() {}
