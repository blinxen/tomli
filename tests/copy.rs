mod common;

use common::generic_test;
use std::process::Command;

macro_rules! copy_test {
    ($test_name:ident, $source_query:literal, $destination_query:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec![
                "copy",
                "--filepath",
                "tests/files/valid.toml",
                $source_query,
                "/tmp/destination.toml",
                $destination_query
            ],
            $expected
        );
    };
}

/*
* Tests that should always pass
*/

// Test if copying a table into another file works
copy_test!(
    should_copy_whole_table,
    "second_table",
    "new_table",
    r#"
[new_table.'brackets(more_brackets(quotes = "a", more_quotes = "b"))']
key = "value"
"#
);

// Test if copying a key into another file works
copy_test!(
    should_copy_only_key,
    "table.key_with_decorator",
    "new_key",
    r#"new_key = "value"
"#
);

// Test if copying a array into another file works
copy_test!(
    should_copy_only_array,
    "table.array",
    "new_array",
    r#"new_array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]
"#
);

// Test if copying a inline table into another file works
copy_test!(
    should_copy_only_inline_table,
    "table.inline_table",
    "new_inline_table",
    r#"new_inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
"#
);

// Test if copying a inline table into a table that does not exist in another file works
copy_test!(
    should_copy_inline_table_and_create_non_existent_table,
    "table.inline_table",
    "non_existent_table.new_inline_table",
    r#"[non_existent_table]
new_inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
"#
);
