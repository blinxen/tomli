mod common;

use common::generic_test;
use std::process::Command;

macro_rules! set_test {
    ($test_name:ident, $query:literal, $value:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec![
                "set",
                "--filepath",
                "tests/files/valid.toml",
                $query,
                $value
            ],
            $expected
        );
    };
    ($test_name:ident, $query:literal, $value:literal, $value_type:literal, $expected:literal) => {
        generic_test!(
            $test_name,
            vec![
                "set",
                "--filepath",
                "tests/files/valid.toml",
                "--type",
                $value_type,
                $query,
                $value
            ],
            $expected
        );
    };
}

/*
* Tests that should always pass
*/

// Test if setting a value in a table works
set_test!(
    should_add_value_in_table,
    "table.key",
    "value1",
    r#"[table]
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

"#
);

// Test if adding a value in a table works
set_test!(
    should_set_value_in_table,
    "table.key_with_decorator",
    "value1",
    r#"[table]
key_with_decorator = "value1"
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

"#
);

// Test if setting a value in an array works
set_test!(
    should_set_value_in_array,
    "table.array[1]",
    "5",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, "5", 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if setting a value in an array of tables works
set_test!(
    should_set_value_in_array_of_tables,
    "table.array_of_tables[0].key",
    "value2",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value2"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if setting a value in an inline table works
set_test!(
    should_set_value_in_inline_table,
    "table.inline_table.inline_key",
    "inline_value3",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value3", array_in_inline_table = [] }
array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if adding a new inline table works
set_test!(
    should_add_inline_table_in_inline_table,
    "table.inline_table.second_inline_table.key",
    "value",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] , second_inline_table = { key = "value" } }
array = [1, 2, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if setting the value type to a integer works
set_test!(
    should_set_value_in_array_and_convert_it_to_int,
    "table.array[1]",
    "5",
    "int",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 5, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if setting the value type to a float works
set_test!(
    should_set_value_in_array_and_convert_it_to_float,
    "table.array[1]",
    "5",
    "float",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, 5.0, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

// Test if setting the value type to a bool works
set_test!(
    should_set_value_in_array_and_convert_it_to_bool,
    "table.array[1]",
    "false",
    "bool",
    r#"[table]
key_with_decorator = "value"
key_without_decorator ="value"
number = 2
inline_table = { inline_key = "inline_value", array_in_inline_table = [] }
array = [1, false, 3, [4, 5, 6, { name = "inline_table_in_array", another_array = [8, 9]}]]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

[[table.array_of_tables]]
key = "value"
key2 = "value2"
array = [1, 2, 3]

"#
);

/*
* Tests that should always fail
*/

// Test if setting a table / array to a value fails
set_test!(
    should_not_set_table_to_value,
    "table",
    "value1",
    r#"Invalid query for command set
Transforming a table or an array to a key / pair value is not allowed
"#
);

set_test!(
    should_not_set_array_to_value,
    "table.array",
    "value1",
    r#"Invalid query for command set
Transforming a table or an array to a key / pair value is not allowed
"#
);

set_test!(
    should_not_set_array_of_tables_to_value,
    "table.array_of_tables",
    "value1",
    r#"Invalid query for command set
Transforming a table or an array to a key / pair value is not allowed
"#
);

// Test if setting the value type to a integer fails
set_test!(
    should_fail_to_convert_value_to_integer,
    "table.array[1]",
    "fff",
    "int",
    "Could not convert the given value to an integer (i64)\n"
);

// Test if setting the value type to a float fails
set_test!(
    should_fail_to_convert_value_to_float,
    "table.array[1]",
    "fff",
    "float",
    "Could not convert the given value to a float (f64)\n"
);

// Test if setting the value type to a bool fails
set_test!(
    should_fail_to_convert_value_to_boolean,
    "table.array[1]",
    "fff",
    "bool",
    "Could not convert the given value to a boolean\n"
);
