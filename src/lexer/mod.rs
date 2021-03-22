

use std::fmt::{self, Debug, Display};
use std::ops::Deref;
use crate::helpers;
use strum_macros::AsRefStr;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Debug, PartialEq, EnumString)]
pub enum Token {
    Insert,
    Update,
    Create,
    Delete,
    Exit,
    Clear,

    #[strum(disabled)]
    Invalid,
}

pub struct Lexer {
    token: Token,
    args: String,
}


impl Lexer {
    pub fn new(statement: &String) -> Self {
       let (operation, args) = helpers::split_first_word(statement);
        let token = match Token::from_str(&operation) {
            Ok(res) => res,
            Err(err) => {
                println!("Invalid '{}' command...", &operation);
                Token::Invalid
            },
        };

        Self {
            token,
            args: args.to_string(),
        }
    }
}
