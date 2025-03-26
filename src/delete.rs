use std::str;
use toml_edit::{DocumentMut, Item, Value};

use crate::errors::TomliError;
use crate::parser;

pub fn exec(document: &mut DocumentMut, query: &str) -> Result<String, TomliError> {
    // Deleting the whole document makes no sense with this tool
    if query == "." || query.is_empty() {
        return Err(TomliError::InvalidInputQuery(
            "delete",
            "Deleting the document as a whole is currently not supported",
        ));
    }

    let mut parent_item = document.as_item_mut();
    let mut toml_path = parser::evaluate(query)?;
    let item_to_delete = toml_path.pop();

    for segment in toml_path.iter() {
        match segment {
            parser::Item::Key(key) => {
                parent_item = match parent_item {
                    Item::Table(_) | Item::Value(Value::InlineTable(_)) => {
                        if let Some(next_item) = parent_item.get_mut(key) {
                            next_item
                        } else {
                            return Err(TomliError::KeyNotFound(key.to_string()));
                        }
                    }
                    _ => return Err(TomliError::InvalidKeyAccess(key.to_string())),
                }
            }
            parser::Item::ArrayIndex(index) => {
                parent_item = match parent_item {
                    Item::ArrayOfTables(_) | Item::Value(Value::Array(_)) => {
                        if let Some(next_item) = parent_item.get_mut(index) {
                            next_item
                        } else {
                            return Err(TomliError::IndexOutOfBounds(*index));
                        }
                    }
                    _ => return Err(TomliError::InvalidKeyAccess(index.to_string())),
                }
            }
        }
    }

    match item_to_delete {
        Some(parser::Item::Key(key)) => {
            if let Some(parent_item) = parent_item.as_table_like_mut() {
                if parent_item.remove(&key).is_none() {
                    return Err(TomliError::KeyNotFound(key));
                }
            } else {
                return Err(TomliError::InvalidKeyAccess(key.to_string()));
            }
        }
        Some(parser::Item::ArrayIndex(index)) => {
            if let Some(parent_item) = parent_item.as_array_mut() {
                if parent_item.len() > index {
                    parent_item.remove(index);
                } else {
                    return Err(TomliError::IndexOutOfBounds(index));
                }
            } else if let Some(parent_item) = parent_item.as_array_of_tables_mut() {
                if parent_item.len() > index {
                    parent_item.remove(index);
                } else {
                    return Err(TomliError::IndexOutOfBounds(index));
                }
            } else {
                return Err(TomliError::InvalidKeyAccess(index.to_string()));
            }
        }
        None => {}
    }

    Ok(document.to_string())
}
