mod common;

use common::generic_test;
use std::process::Command;

macro_rules! delete_test {
    ($test_name:ident, $query:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec!["delete", "--filepath", "tests/files/valid.toml", $query],
            $expected
        );
    };
    ($test_name:ident, "--if-exists", $query:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec![
                "delete",
                "--if-exists",
                "--filepath",
                "tests/files/valid.toml",
                $query
            ],
            $expected
        );
    };
}

delete_test!(
    should_delete_key_in_table,
    "table.key_with_decorator",
    r#"[table]
key_without_decorator ="value"
number = 2
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

"#
);

delete_test!(
    should_delete_element_in_array,
    "table.array[3]",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 2, 3]

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

"#
);

delete_test!(
    should_exit_without_error_if_key_does_not_exist,
    "--if-exists",
    "table.this_key_does_not_exist",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
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

"#
);

delete_test!(
    should_exit_without_error_if_parent_key_does_not_exist,
    "--if-exists",
    "table.parent_key_does_not_exist.child_key_does_not_exist",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
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

"#
);
