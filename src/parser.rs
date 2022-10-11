use crate::{Lexer, Tokens};
use std::process;

pub struct Parser {
    action: Tokens,
}

impl Parser {
    pub fn handle_insert(args: &Option<String>) -> () {
        todo!()
    }
    pub fn handle_exit() {
        process::exit(0)
    }
}
