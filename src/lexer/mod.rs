pub(crate) mod tokens;






use std::str::FromStr;
use tokens::Tokens;


pub struct Lexer {
    token: Tokens,
    args: String,
}

impl Lexer {
    pub fn new(statement: &String) -> Self {
        let (operation, args) = Self::get_operation(&statement);


        let token = match Tokens::from_str(&operation) {
            Ok(res) => res,
            Err(_err) => {
                println!("ERR: Statement [{}] not found!", &operation);
                Tokens::Invalid
            }
        };

        Self {
            token,
            args: args.to_string(),
        }
    }

    pub fn get_action(&self) -> &Tokens {
        &self.token
    }

    fn process_input(&self, input: &String) -> String {
        input.trim().to_lowercase()
    }

    fn get_operation(s: &str) -> (&str, &str) {
        let s = s.trim();

        match s.find(|ch: char| ch.is_whitespace()) {
            Some(pos) => (&s[..pos], s[pos..].trim_start()),
            None => (s, "")
        }
    }
}
