use std::fmt::Debug;

use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
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
