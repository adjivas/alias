Macro
=====

[![Build Status](https://travis-ci.org/adjivas/macro.svg)](https://travis-ci.org/adjivas/macro)

This librairy is a systen's project for replace word by alias or by variable's environement -env-.

#### Example:
```shell
$ cargo run
with
Ok(", ")
hello
Err("hello")
...
```

#### Directory-Tree:

```shell
.
|_ Cargo.toml
|_ data
|   \_ example.alias
|_ LICENSE
|_ README.md
\_ src
    |_ bin.rs
    \_ lib.rs
```

# License
*macro*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/macro/blob/master/LICENSE).
