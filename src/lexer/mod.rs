pub(crate) mod tokens;

use std::str::FromStr;
use tokens::Tokens;

pub struct Lexer {
    token: Tokens,
    args: Option<String>,
}

impl Lexer {
    pub fn new(statement: &str) -> Self {
        let (operation, args) = Self::get_operation(statement);

        let token = match Tokens::from_str(operation) {
            Ok(res) => res,
            Err(_err) => {
                println!("ERR: Statement [{}] not found!", &operation);
                Tokens::Invalid
            }
        };

        Self { token, args: args }
    }

    pub fn get_action(&self) -> &Tokens {
        &self.token
    }

    fn process_input(&self, input: &str) -> String {
        input.trim().to_lowercase()
    }

    fn get_operation(s: &str) -> (&str, Option<String>) {
        let s = s.trim();

        match s.find(|ch: char| ch.is_whitespace()) {
            Some(pos) => (&s[..pos], Some(String::from(s[pos..].trim_start()))),
            None => (s, None),
        }
    }
}
