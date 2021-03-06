mod reader;
mod lexer;
mod helpers;

use clap::{Arg, App, SubCommand};
use linefeed::{Interface, ReadResult, DefaultTerminal};
use reader::{Reader};
use std::process;
use lexer::tokens::Tokens;

fn main() {
    let _matches = App::new("ldb")
        .version("0.0.1")
        .author("Leo Oliveira <qwerty124563@gmail.com>")
        .about("A Database developed using Rust.")
        .get_matches();

    let reader = Reader::new();

    while let ReadResult::Input(input) = reader.read_line() {
        reader.save_history(&input);

        let a = lexer::Lexer::new(&input.to_lowercase());

        // Parser
        match a.get_action() {
            Tokens::Exit => process::exit(0),
            _ => println!("got input {:?}", input),
        }
    }

    println!("Goodbye.");

    println!("Hello, world!");
}
