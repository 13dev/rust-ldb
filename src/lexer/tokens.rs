use std::fmt::{Debug};
use std::ops::Deref;
use strum_macros::AsRefStr;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Tokens {
    Insert,
    Update,
    Create,
    Delete,
    Clear,

    #[strum(serialize = ".exit")]
    Exit,

    #[strum(disabled)]
    Invalid,
}
