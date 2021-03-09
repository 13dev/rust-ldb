

use std::fmt::{self, Debug, Display};
use std::ops::Deref;
use crate::helpers;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
pub enum Token {
    Insert,
    Update,
    Create,
    Delete,
    Exit,
    Clear,
}


struct Lexer {
    operation: Token,
    args: Vec<String>,
}


// impl Lexer {
//     pub fn new(&self, statement: &String) -> Self {
//        let (operation, args) = helpers::split_first_word(statement);
//         Self {
//             operation: operation,
//             args: Vec::from(args.to_string()),
//         }
//     }
// }
pub fn find_operation() {

}