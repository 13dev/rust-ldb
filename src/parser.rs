use crate::Tokens;
use std::process;

pub struct Parser {
    action: Tokens,
}

impl Parser {
    pub fn handle_insert() -> () {
        todo!()
    }
    pub fn handle_exit() {
        process::exit(0)
    }
}
