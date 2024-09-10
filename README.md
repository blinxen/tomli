tomli
=====

`tomli` is a *format preserving* command-line tool for querying and editing TOML files.

This project is currently **under development** and the command-line interface
can change at any time.


Installation
------------

### Using Cargo

```
cargo install tomli
```


Usage
-----

By default, `tomli` will read from stdin unless `--filepath` is specified.

### Query expressions

`tomli` tries to use the same syntax as `jq` for query expressions.
The following expressions are currently supported:

* Key and table identifier paths, e.g `foo.bar` or `.` (whole document)
* Array indices, e.g. `foo[0]` or `foo.bar[0][1]`

### Query TOML files

**Examples**:

Get the value of the `name` key from the `package` table:

```
tomli query -f Cargo.toml package.name
```

Get the first element of the array called `bin`:

```
tomli query -f Cargo.toml bin[0]
```

### Edit TOML files

`tomli` currently supports the following types when setting a value:

* str (String)
* int ([64-bit signed integer](https://doc.rust-lang.org/std/primitive.i64.html))
* float ([64-bit floating point](https://doc.rust-lang.org/std/primitive.f64.html))
* bool (Boolean)

The value type can be specified with the `--type` argument.
If not type is specified, then `str` is used.

**Examples**:

Add a new key to the `package` table called `website`:

```
tomli set -f Cargo.toml package.website https://example.com
```

Set the `name` of the first element in the `bin` array to `tumli`:

```
tomli set -f Cargo.toml bin[0].name tumli
```

Add a new table called `website` to the first element of the `bin` array:

```
tomli set -f Cargo.toml bin[0].website.url https://example.com
```

Delete `name` in the table `package`:

```
tomli delete -f Cargo.toml package.name
```

Delete the first element in the array authors:

```
tomli delete -f Cargo.toml package.authors[0]
```
