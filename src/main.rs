mod helpers;
mod lexer;
mod parser;
mod reader;

use crate::parser::Parser;
use clap::{App, Arg, SubCommand};
use lexer::tokens::Tokens;
use linefeed::{DefaultTerminal, Interface, ReadResult};
use reader::Reader;
use std::process;

const APP_NAME: &str = "ldb";

fn main() {
    App::new(APP_NAME)
        .version("0.0.1")
        .author("Leo Oliveira <leo@13dev.pt>")
        .about("A Database developed using Rust.")
        .get_matches();

    let reader = Reader::new(APP_NAME);

    while let ReadResult::Input(input) = reader.read_line() {
        reader.save_history(&input);

        let a = lexer::Lexer::new(&input.to_lowercase());

        // Parser
        match a.get_action() {
            Tokens::Exit => Parser::handle_exit(),
            Tokens::Insert => Parser::handle_insert(),
            Tokens::Update => unimplemented!(),
            Tokens::Create => unimplemented!(),
            Tokens::Delete => unimplemented!(),
            Tokens::Clear => unimplemented!(),
            Tokens::Invalid => eprintln!("Error: Command is invalid please try again."),
        }
    }

    println!("Goodbye.");

    println!("Hello, world!");
}
