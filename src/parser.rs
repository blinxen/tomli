use std::str::Chars;

use crate::TomliError;

#[derive(Debug)]
pub enum Item {
    Key(String),
    ArrayIndex(usize),
}

#[derive(PartialEq, Debug)]
enum Token {
    OpenBracket,
    CloseBracket,
    Dot,
    Char(char),
    Number(char),
    Unkown(char),
}

fn tokenize(chars: Chars) -> Vec<Token> {
    let mut tokens = Vec::new();

    for character in chars {
        tokens.push(match character {
            '.' => Token::Dot,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            c if c.is_alphabetic()
                || ['_', '-', '"', '\'', '(', ')', '=', ','].contains(&c)
                || c.is_whitespace() =>
            {
                Token::Char(c)
            }
            n if n.is_numeric() => Token::Number(n),
            c => Token::Unkown(c),
        });
    }

    tokens
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Item>, TomliError> {
    let mut tokens_iterator = tokens.iter().peekable();
    // TOML path that will be contructed depending on the tokens
    let mut path = Vec::new();
    // This key will be used to form key Items while iterating
    let mut key = String::new();
    // This key will be used to form index Items while iterating
    let mut index = String::new();
    let mut counter = 0;
    let mut previous_token: Option<&Token> = None;
    let mut brackets_open = false;
    let mut quote: Option<char> = None;

    while let Some(next) = tokens_iterator.next() {
        match next {
            Token::Char(c) | Token::Number(c) => {
                let char = *c;
                // Depending on whether we are currently looking at an index or a key
                // we need to append a different variable
                let mut str_to_append = &mut key;

                // if brackets are open and the current character is a number then we consider it
                // and index
                if brackets_open && matches!(next, Token::Number(_)) {
                    str_to_append = &mut index;
                // If a bracket was opened then it is not allowed to have characters until the
                // bracket is closed again
                } else if brackets_open
                    // Parenthesis and commas can only be used when they are inside of quotes
                    || ((char == '(' || char == ')' || char == ',') && quote.is_none())
                    // Quotes pair can only be started if placed at the beginning of a key or after a DOT
                    || ((char == '\'' || char == '"') && quote.is_none() && !matches!(previous_token, None | Some(&Token::Dot)))
                    // Quotes pair can only be closed at the end or if a DOT is the next character
                    || (quote.is_some() && quote == Some(char) && !matches!(tokens_iterator.peek(), Some(&Token::Dot) | None))
                {
                    return Err(TomliError::QuerySyntaxError(counter));
                } else if quote.is_none() && (char == '\'' || char == '"') {
                    quote = Some(char);
                } else if quote == Some(char) {
                    quote = None;
                }

                // The outer quotes of a key inside a table definition will get removed by
                // toml_edit. At the time of writing it seems that this is expected and the toml format does
                // not specify that they need to be preserved.
                // That is why we don't add them to the key
                if !(((char == '\'' || char == '"') && quote.is_none()) || quote == Some(char)) {
                    str_to_append.push(char);
                }

                // If we are at the end of the tokenstream, then we can push the last build key
                // to the path
                if tokens_iterator.peek().is_none() {
                    path.push(Item::Key(key.clone()));
                }
            }
            Token::CloseBracket => {
                // Sanity checks before doing anything
                // We can't close a bracket that was never opened
                // Closing a bracket that was only just opened is not valid
                // A closing bracket must be following by either a dot or another opening bracket
                if !brackets_open
                    || matches!(previous_token, Some(&Token::OpenBracket))
                    || !matches!(
                        tokens_iterator.peek(),
                        Some(&Token::Dot) | Some(&Token::OpenBracket) | None
                    )
                {
                    return Err(TomliError::QuerySyntaxError(counter));
                }
                if quote.is_none() {
                    path.push(Item::ArrayIndex(index.parse().unwrap()));
                    brackets_open = false;
                    index.clear();
                } else {
                    key.push(']');
                }
            }
            Token::Dot => {
                // Sanity checks before doing anything
                // A dot cannot be placed before a opening bracket
                // A dot cannot be placed before a dot
                // A query cannot end with a "."
                if matches!(
                    previous_token,
                    Some(&Token::OpenBracket) | Some(&Token::Dot)
                ) || (previous_token.is_some() && tokens_iterator.peek().is_none())
                {
                    return Err(TomliError::QuerySyntaxError(counter));
                }
                if !key.is_empty() && quote.is_none() {
                    path.push(Item::Key(key.clone()));
                    key.clear();
                } else {
                    key.push('.');
                }
            }
            Token::OpenBracket => {
                // Sanity checks before doing anything
                // An opening bracket can only be placed after a character
                // A query cannot end with a opening bracket
                if !matches!(
                    previous_token,
                    Some(&Token::Char(_)) | Some(&Token::CloseBracket)
                ) || tokens_iterator.peek().is_none()
                {
                    return Err(TomliError::QuerySyntaxError(counter));
                }
                if !key.is_empty() && quote.is_none() {
                    path.push(Item::Key(key.clone()));
                } else {
                    key.push('[');
                }
                key.clear();
                brackets_open = true;
            }
            Token::Unkown(_) => return Err(TomliError::QuerySyntaxError(counter)),
        }
        counter += 1;
        previous_token = Some(next);
    }

    if brackets_open || quote.is_some() {
        Err(TomliError::QuerySyntaxError(counter))
    } else {
        Ok(path)
    }
}

/// Evaluate `query` and generate a path to the requested TOML item
/// If query is none or "" or ".", then an empty vec is returned
pub fn evaluate(query: &str) -> Result<Vec<Item>, TomliError> {
    let mut path = Vec::new();

    let tokens = tokenize(query.chars());
    if !(tokens.is_empty() && tokens.len() == 1 && tokens[0] == Token::Dot) {
        path = parse_tokens(tokens)?;
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use crate::parser::evaluate;

    #[test]
    fn test_valid_quries() {
        let queries = vec![
            ".",
            ".table",
            "table",
            "table.key",
            "table[1]",
            "table[1].key",
            "table.key[1]",
            "table[1][3]",
            "table[1][3].key",
            "table[1][3].key[1]",
            "table[1][3].key[1][5]",
            "_table[1]._",
            "-table[1].-",
            "_",
            "-",
            "_table[1]",
            "_1",
            "1_",
            "-1",
            "1-",
            "table.\"key\"",
            "table.\"'key\"",
            "table.'key'",
            "table.'\"key'",
        ];

        for query in queries {
            println!("Testing {}", query);
            assert!(evaluate(query).is_ok());
        }
    }

    #[test]
    fn test_invalid_quries() {
        let queries = vec![
            ",",
            "..",
            "table...key",
            "table[1]f",
            "table[1",
            "table[1].",
            "[]",
            "1[]",
            "[1]",
            "][",
            "table[_]",
            "table[f]",
            "table]",
            "table][",
            "table]",
            "table[",
            "table[5].[5]",
            "table[5].[]",
            "table\"",
            "table\"\"",
            "table\"'",
            "table.\"key\"\"",
            "table.\"key\"'",
            "table(",
            "table)",
            "table.('key')",
        ];

        for query in queries {
            println!("Testing {}", query);
            assert!(evaluate(query).is_err());
        }
    }
}
