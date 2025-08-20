use std::str;
use std::str::FromStr;
use toml_edit::{DocumentMut, InlineTable, Item, Table, Value};

use crate::errors::TomliError;
use crate::{ValueType, parser};

pub fn exec(
    document: &mut DocumentMut,
    query: &str,
    value: &str,
    value_type: ValueType,
) -> Result<String, TomliError> {
    // Editing the whole document makes no sense
    // If the user wants to do this, then he should use echo (or a similiar tool) to edit the file manually
    if query == "." || query.is_empty() {
        return Err(TomliError::InvalidInputQuery(
            "set",
            "Editing the document as a whole is currently not supported",
        ));
    }

    let mut item = document.as_item_mut();
    let toml_path = parser::evaluate(query)?;
    // When parsing the toml path we want to keep track of whether we are currently in a inlined
    // table or not. Depending on this, we either crate a new table or a new inlined table.
    let mut inline_table = false;

    for segment in toml_path.iter() {
        match segment {
            parser::Item::Key(key) => {
                item = match item {
                    // Either retrieve the item from a table or create a new one
                    Item::Table(t) => t.entry(key).or_insert(Item::None),
                    Item::ArrayOfTables(_) | Item::Value(Value::Array(_)) => {
                        return Err(TomliError::InvalidKeyAccess(key.to_string()));
                    }
                    // Either retrieve the item from a table or create a new one
                    // Also mark that we are now in an inline table
                    Item::Value(Value::InlineTable(t)) => {
                        inline_table = true;
                        t.entry(key)
                            .or_insert(Value::InlineTable(toml_edit::InlineTable::new()));

                        item.get_mut(key)
                            .expect("BUG: Could not find key that was just inserted")
                    }
                    // Create a new table / inline table and add the new item to it
                    // The way this works is the follwing:
                    //
                    // The first element in a toml document will always be a table
                    // This means that item will either be an actual item or an empty item (see the
                    // first match arm)
                    // Since we already have the item, we just set it to either a table or inline
                    // table and add a new key / value pair to it and return that item
                    _ => {
                        *item = if inline_table {
                            let mut table = InlineTable::new();
                            table.insert(key, Value::from(""));

                            Item::Value(Value::InlineTable(table))
                        } else {
                            let mut table = Table::new();
                            table.insert(key, toml_edit::value(""));

                            Item::Table(table)
                        };

                        item.get_mut(key)
                            .expect("BUG: Could not find key that was just inserted")
                    }
                };
            }
            parser::Item::ArrayIndex(index) => {
                item = match item {
                    Item::ArrayOfTables(_) | Item::Value(Value::Array(_)) => {
                        // Either return the item at index "index" or show an out of bounds message
                        // TODO: Maybe we want to replace the out of bounds message with just
                        // appending the item to the end of the array
                        if item.get(*index).is_none() {
                            return Err(TomliError::IndexOutOfBounds(*index));
                        }

                        item.get_mut(index)
                            .expect("BUG: Expected item at index but could not find it")
                    }
                    // We only accept arrays here
                    // All other types are an invalid access
                    _ => return Err(TomliError::InvalidKeyAccess(index.to_string())),
                };
            }
        };
    }

    if item.is_table() || item.is_array() || item.is_array_of_tables() {
        return Err(TomliError::InvalidInputQuery(
            "set",
            "Transforming a table or an array to a key / pair value is not allowed",
        ));
    }

    match value_type {
        ValueType::Str => *item = toml_edit::value(value),
        ValueType::Int => *item = toml_edit::value::<i64>(i64::from_str(value)?),
        ValueType::Float => *item = toml_edit::value::<f64>(f64::from_str(value)?),
        ValueType::Bool => *item = toml_edit::value::<bool>(bool::from_str(value)?),
    };

    Ok(document.to_string())
}
