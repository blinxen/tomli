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
            c if c.is_alphabetic() || ['_', '-'].contains(&c) => Token::Char(c),
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

    while let Some(next) = tokens_iterator.next() {
        match next {
            Token::Number(n) => {
                // Keys can also contains numbers, so we only add number chars
                // to the index if a bracket was opened
                if brackets_open {
                    index.push(*n);
                } else {
                    key.push(*n);
                }
            }
            Token::Char(c) => {
                // If a bracket was opened then it is no allowed to have characters until the
                // bracket is closed again
                if brackets_open {
                    return Err(TomliError::QuerySyntaxError(counter));
                } else {
                    key.push(*c);
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
                path.push(Item::ArrayIndex(index.parse().unwrap()));
                brackets_open = false;
                index.clear();
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
                if !key.is_empty() {
                    path.push(Item::Key(key.clone()));
                    key.clear();
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
                // If the key is empty, don't push it to the path since this means
                // that the last token was a closing bracket
                if !key.is_empty() {
                    path.push(Item::Key(key.clone()));
                }
                key.clear();
                brackets_open = true;
            }
            Token::Unkown(_) => return Err(TomliError::QuerySyntaxError(counter)),
        }
        counter += 1;
        previous_token = Some(next);
    }

    if brackets_open {
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
        ];

        for query in queries {
            println!("Testing {}", query);
            assert!(evaluate(query).is_err());
        }
    }
}
