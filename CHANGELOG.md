# Changelog for `tomli`

## 3.2.0 (UPCOMING)

* Add datetime type to supported value types
* Add `--dotted-key` flag to the `set` command so users can decide themselfs
  if they want to set the value in a subtable or a dotted key.
  See the help message for more information on how this looks.
* Allow the creation of arrays with the `set` command instead of showing an error

## 0.3.1

* Fix [issue](https://github.com/blinxen/tomli/issues/5)
  where quotes could not be used in a table name
* Update dependencies
* Fix previous changelog entry

## 0.3.0

* Add `--in-place` flag to allow making modifications in place

## 0.2.3

* Document `--if-exists` flag
* Update dependencies

## 0.2.2

* Add `--if-exists` flag to delete command

## 0.2.1

* Update dependencies

## 0.2.0

* Update dependencies
* Mention license in README
* Add more characters to the set of valid toml key characters for a query

## 0.1.1

* Fix examples in README

## 0.1.0

* Initial release
