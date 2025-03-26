mod common;

use common::generic_test;
use std::{io::Read, process::Command};

// All sub commands use the same method to read the input document
// To simplify testing, we will only be using the query command here

macro_rules! input_test {
    ($test_name:ident, $file:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec!["query", "--filepath", concat!("tests/files/", $file), "."],
            $expected
        );
    };
}

// Empty TOML documents are valid
input_test!(should_print_an_empty_document, "empty.toml", "\n");
// Test whether invalid TOML files are accepted
input_test!(
    should_print_error_message_on_invalid_toml,
    "invalid.toml",
    "The provided TOML document has an invalid syntax:\n\nTOML parse error at line 2, column 5\n  |\n2 | name\n  |     ^\nexpected `.`, `=`\n\n"
);
input_test!(
    should_print_error_message_on_json_file,
    "json.toml",
    "The provided TOML document has an invalid syntax:\n\nTOML parse error at line 1, column 1\n  |\n1 | {\n  | ^\ninvalid key\n\n"
);

#[test]
fn should_set_and_write_into_file() {
    let expected = r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]
key = "value1"

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[second_table.'brackets(more_brackets(quotes = "a", more_quotes = "b"))']
key = "value"
"#;
    let mut file = tempfile::NamedTempFile::new().expect("Could not create temporary file");
    std::fs::copy("tests/files/valid.toml", file.path()).expect("Could not copy file");
    let result = Command::new(env!("CARGO_BIN_EXE_tomli"))
        .args(vec![
            "set",
            "--in-place",
            "--filepath",
            &file.path().to_string_lossy(),
            "table.key",
            "value1",
        ])
        .output();

    if let Ok(result) = result {
        assert!(result.status.success());
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
        let mut actual = String::new();
        file.read_to_string(&mut actual).expect("Could not read");
        assert_eq!(actual, expected)
    } else {
        panic!("Command could not be executed --> {}", result.unwrap_err());
    }
}

#[test]
fn should_delete_and_write_into_file() {
    let expected = r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[second_table.'brackets(more_brackets(quotes = "a", more_quotes = "b"))']
key = "value"
"#;
    let mut file = tempfile::NamedTempFile::new().expect("Could not create temporary file");
    std::fs::copy("tests/files/valid.toml", file.path()).expect("Could not copy file");
    let result = Command::new(env!("CARGO_BIN_EXE_tomli"))
        .args(vec![
            "delete",
            "--in-place",
            "--filepath",
            &file.path().to_string_lossy(),
            "table.number",
        ])
        .output();

    if let Ok(result) = result {
        assert!(result.status.success());
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
        let mut actual = String::new();
        file.read_to_string(&mut actual).expect("Could not read");
        assert_eq!(actual, expected)
    } else {
        panic!("Command could not be executed --> {}", result.unwrap_err());
    }
}

#[test]
fn should_query_and_not_write_into_file() {
    let mut file = tempfile::NamedTempFile::new().expect("Could not create temporary file");
    std::fs::copy("tests/files/valid.toml", file.path()).expect("Could not copy file");
    let result = Command::new(env!("CARGO_BIN_EXE_tomli"))
        .args(vec![
            "query",
            "--in-place",
            "--filepath",
            &file.path().to_string_lossy(),
            "table.key_with_decorator",
        ])
        .output();

    if let Ok(result) = result {
        assert!(result.status.success());
        assert!(result.stderr.is_empty());
        let mut actual = String::new();
        file.read_to_string(&mut actual).expect("Could not read");
        assert_eq!(
            actual,
            std::fs::read_to_string("tests/files/valid.toml").expect("Could not read file")
        );
        assert_eq!(String::from_utf8_lossy(&result.stdout), " \"value\"\n");
    } else {
        panic!("Command could not be executed --> {}", result.unwrap_err());
    }
}
