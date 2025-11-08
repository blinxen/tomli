use toml_edit::DocumentMut;

use crate::{errors::TomliError, parser, query, set};

pub fn exec(
    source: &DocumentMut,
    source_query: &str,
    destination: &mut DocumentMut,
    destination_query: &str,
    dotted_key: bool,
) -> Result<String, TomliError> {
    let item_to_copy = if source_query.is_empty() || source_query == "." {
        source.as_item()
    } else {
        query::parse_toml_path(parser::evaluate(source_query)?, source.as_item())?
    };

    let item = set::parse_toml_path(
        parser::evaluate(destination_query)?,
        destination.as_item_mut(),
        dotted_key,
    )?;
    *item = item_to_copy.clone();

    Ok(destination.to_string())
}
