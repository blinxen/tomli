use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
use toml_edit::DatetimeParseError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TomliError {
    #[error(
        "The provided file does not exist or \
        you don't have enough permissions to read it: {0}"
    )]
    FileReadError(#[from] std::io::Error),
    #[error("The provided TOML document has an invalid syntax:\n\n{0}")]
    TomlSyntaxError(#[from] toml_edit::TomlError),
    #[error("The provided query has an invalid syntax")]
    QuerySyntaxError(usize),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error(
        "\"{0}\" was used wrongly in the query.\n\
        This usually means that a key which has a type (e.g. key / value pair) \
        was used as another type (e.g. array)"
    )]
    InvalidKeyAccess(String),
    #[error("Index (\"{0}\") is out of bounds")]
    IndexOutOfBounds(usize),
    #[error("Invalid query for command {0}\n{1}")]
    InvalidInputQuery(&'static str, &'static str),
    #[error("Could not convert the given value to an integer (i64)")]
    ValueToIntError(#[from] ParseIntError),
    #[error("Could not convert the given value to a float (f64)")]
    ValueToFloatError(#[from] ParseFloatError),
    #[error("Could not convert the given value to a boolean")]
    ValueToBoolError(#[from] ParseBoolError),
    #[error("Could not convert the given value to a datetime")]
    ValueToDatetimeError(#[from] DatetimeParseError),
}
