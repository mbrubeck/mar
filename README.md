This is a Rust implementation of the [Mozilla Archive (MAR) file format][1]
used to deliver automatic updates to Firefox.  It includes both a library and
a command-line tool for reading and writing MAR files.

* [Documentation](https://docs.rs/mar/)
* [crates.io](https://crates.io/crates/mar)

Currently supports:

* Extracting MAR archives

Not yet supported:

* Creating MAR archives
* Signing MAR archives
* Verifying signed MAR archives

This code is subject to the terms of the Mozilla Public License, v. 2.0.

[1]: https://wiki.mozilla.org/Software_Update:MAR
