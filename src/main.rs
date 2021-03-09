use clap::{Arg, App, SubCommand};

use linefeed::{Interface, ReadResult};
use std::borrow::Borrow;
use std::ptr::read;


fn main() {
    let matches = App::new("ldb")
        .version("0.0.1")
        .author("Leo Oliveira <qwerty124563@gmail.com>")
        .about("A Database developed using Rust.")
        .get_matches();


    let mut reader = Interface::new("ldb")
        .expect("Can't create a new application.");

    reader.set_prompt("my-app> ")
        .expect("Can't set a prompt.");

    loop {
        let line = reader
            .read_line()
            .expect("Cant read line");

        match line {
            ReadResult::Input(input) => {
                // Parse result
                println!("got input {:?}", input);
            }
            _ => panic!("Error getting the input."),
        }
    }


    println!("Goodbye.");

    println!("Hello, world!");
}
