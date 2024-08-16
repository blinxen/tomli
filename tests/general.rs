mod common;

use common::generic_test;
use std::process::Command;

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
