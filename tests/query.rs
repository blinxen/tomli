mod common;

use common::generic_test;
use std::process::Command;

macro_rules! query_test {
    ($test_name:ident, $query:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec!["query", "--filepath", "tests/files/valid.toml", $query],
            $expected
        );
    };
}

/*
* Tests that should always pass
*/

// Test if the whole document will be printed
query_test!(
    should_print_whole_document,
    ".",
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

// Test if strings are returned correctly (with decorators)
// Quotes are present, since the value we are requesting is a string
query_test!(
    should_print_value_with_whitespace_prefix,
    "table.key_with_decorator",
    " \"value\""
);
query_test!(
    should_print_value_without_whitespace_prefix,
    "table.key_without_decorator",
    "\"value\""
);
// Test if numbers are returned correctly
query_test!(should_print_value_as_number, "table.number", " 2");
// Test if inline tables are returned correctly
query_test!(
    should_print_inline_table,
    "table.inline_table",
    " { inline_key = \"inline_value\", array_in_inline_table = [] }"
);
// Test if array of tables are returned correctly
query_test!(
    should_print_array_of_tables,
    "table.array_of_tables",
    "[{ key = \"value\", key2 = \"value2\", array = [1, 2, 3] }, { key = \"value\", key2 = \"value2\", array = [1, 2, 3] }]"
);
// Test if arrays are returned correctly
query_test!(
    should_print_array,
    "table.array",
    " [1, 2, 3, [4, 5, 6, { name = \"inline_table_in_array\", another_array = [8, 9]}]]"
);
// Test if we can get the first item in an array
query_test!(should_print_element_in_array, "table.array[1]", " 2");
// Test if we can get a key in a table stored in an array of tables
query_test!(
    should_print_value_from_table_in_array_of_tables,
    "table.array_of_tables[0].key",
    " \"value\""
);
// Test if quotes and brackets can be used as a table header
query_test!(
    should_print_table_with_weird_name,
    "second_table.'brackets(more_brackets(quotes = \"a\", more_quotes = \"b\"))'.key",
    " \"value\""
);

/*
* Tests that should always fail
*/

// Test if using non-existant keys fails
query_test!(
    should_print_key_not_found,
    "I_dont_exist",
    "Key not found: I_dont_exist\n"
);
query_test!(
    should_print_key_not_found_in_table,
    "table.I_dont_exist",
    "Key not found: I_dont_exist\n"
);
query_test!(
    should_print_key_not_found_in_inline_table,
    "table.inline_table.I_dont_exist",
    "Key not found: I_dont_exist\n"
);
// Test if using an index that is out-of-bounds fails
query_test!(
    should_print_index_out_of_bounds,
    "table.array[5]",
    "Index (\"5\") is out of bounds\n"
);
query_test!(
    should_print_index_out_of_bounds_for_array_of_tables,
    "table.array_of_tables[5]",
    "Index (\"5\") is out of bounds\n"
);
