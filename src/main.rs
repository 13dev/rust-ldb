use clap::{Arg, App, SubCommand};

use linefeed::{Interface, ReadResult, DefaultTerminal};
use std::borrow::Borrow;
use std::ptr::read;

const FILE_HISTORY: &str = ".history";
const PROMPT_MESSAGE: &str = "ldb >>> ";


struct Reader {
    interface: Interface<DefaultTerminal>,

}

impl Reader {
    fn new() -> Self {
        let mut reader = Interface::new("ldb")
            .expect("Can't create a new application.");

        // Set prompt
        reader.set_prompt(PROMPT_MESSAGE)
            .expect("Can't set a prompt.");

        // Load history
        if reader.load_history(FILE_HISTORY).is_err() {
            println!("No History.");
        }

        Self {
            interface: reader,
        }
    }

    fn save_history(&self, line: String) {
        if !line.trim().is_empty() {
            self.get_interface()
                .add_history_unique(line.clone());
        }
    }

    fn get_interface(&self) -> &Interface<DefaultTerminal> {
        &self.interface
    }

    fn read_line(&self) -> ReadResult {
        self.get_interface()
            .read_line()
            .expect("Cant read line.")
    }
}


fn main() {
    let matches = App::new("ldb")
        .version("0.0.1")
        .author("Leo Oliveira <qwerty124563@gmail.com>")
        .about("A Database developed using Rust.")
        .get_matches();

    let reader = Reader::new();

    while let ReadResult::Input(input) = reader.read_line() {
        println!("got input {:?}", input);
    }

    println!("Goodbye.");

    println!("Hello, world!");
}

fn save_history() {}
