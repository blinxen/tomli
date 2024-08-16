tomli
=====

`tomli` is a *format preserving* command-line tool for querying and editing TOML files.

This project is currently **under development** and the command-line interface
can change at any time.


Installation
------------

### Using Cargo

```bash
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

Examples:

Get the value of the `name` key from the `package` table:

```
$ tomli query -f Cargo.toml package.name

"tomli"
```

Get the first element of the array called `bin`:

```
$ tomli query -f Cargo.toml bin[0]

name = "tomli"
path = "src/main.rs"
```

### Edit TOML files

`tomli` currently supports the following types when setting a value:

* str (String)
* int ([64-bit signed integer](https://doc.rust-lang.org/std/primitive.i64.html))
* float ([64-bit floating point](https://doc.rust-lang.org/std/primitive.f64.html))
* bool (Boolean)

The value type can be specified with the `--type` argument.
If not type is specified, then `str` is used.

Examples:

Add a new key to the `package` table called `website`:

```
$ tomli set -f Cargo.toml package.website https://example.com

[package]
name = "tomli"
version = "0.1.0"
description = "A simple CLI for quering and editing TOML files"
authors = ["blinxen <h-k-81@hotmail.com>"]
repository = "https://github.com/blinxen/tomli"
readme = "README.md"
license = "MIT"

edition = "2021"
website = "https://example.com"
```

Set the `name` of the first element in the `bin` array to `tumli`:

```
$ tomli set -f Cargo.toml bin[0].name tumli

[[bin]]
name = "tumli"
path = "src/main.rs"
```

Add a new table called `website` to the first element of the `bin` array:

```
$ tomli set -f Cargo.toml bin[0].website.url https://example.com

[[bin]]
name = "tomli"
path = "src/main.rs"

[bin.website]
url = "https://example.com"
```

Delete `name` in the table `package`:

```
$ tomli set -f Cargo.toml package.name

[package]
version = "0.1.0"
description = "A simple CLI for quering and editing TOML files"
authors = ["blinxen <h-k-81@hotmail.com>"]
repository = "https://github.com/blinxen/tomli"
readme = "README.md"
license = "MIT"

edition = "2021"
website = "https://example.com"
```

Delete the first element in the array authors:

```
$ tomli set -f Cargo.toml package.authors[0]


[package]
version = "0.1.0"
description = "A simple CLI for quering and editing TOML files"
authors = []
repository = "https://github.com/blinxen/tomli"
readme = "README.md"
license = "MIT"

edition = "2021"
website = "https://example.com"
```
