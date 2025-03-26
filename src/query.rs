use std::str;
use toml_edit::DocumentMut;

use crate::errors::TomliError;
use crate::parser;

pub fn exec(document: &DocumentMut, query: &str) -> Result<String, TomliError> {
    // An empty query or a dot are evaluated as the whole document
    if query == "." || query.is_empty() {
        return Ok(document.to_string());
    }

    let mut item = document.as_item();
    let toml_path = parser::evaluate(query)?;

    for segment in toml_path.iter() {
        match segment {
            parser::Item::Key(key) => {
                item = if let Some(item) = item.get(key) {
                    item
                } else if item.as_array_of_tables().is_some() {
                    return Err(TomliError::InvalidKeyAccess(key.to_string()));
                } else {
                    return Err(TomliError::KeyNotFound(key.to_string()));
                }
            }
            parser::Item::ArrayIndex(index) => {
                item = if let Some(item) = item.get(index) {
                    item
                } else if item.as_array().is_some() || item.as_array_of_tables().is_some() {
                    return Err(TomliError::IndexOutOfBounds(*index));
                } else {
                    return Err(TomliError::InvalidKeyAccess(index.to_string()));
                }
            }
        };
    }

    Ok(item.to_string())
}
