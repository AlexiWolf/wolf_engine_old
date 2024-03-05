# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

**NOTE:**  This is the old, prototype versions of the engine.  It is here
only to preserve the history of the project.  You can find the new repo
[here](https://github.com/AlexiWolf/wolf_engine).

A simple, and flexible game framework written in Rust.

Usage documentation, and interactive examples can be found on 
[docs.rs](https://docs.rs/wolf_engine/latest/), and in the 
[examples/](examples/) directory.

### Status

Wolf Engine is currently in very early development.  You should expect missing
features, bugs, changing APIs, and other spooky stuff until release 1.0.

## Quick-Start Guide

### Install

To use the latest release version:

```
[dependencies]
wolf_engine = "*"
```

To use the latest development version:

```
wolf_engine = { git = "https://github.com/AlexiWolf/wolf_engine" }
```

To add the latest release to your project.

### Crate Features

- `framework`: Enable the high-level framework (enabled by default.)
- `logging`: Enable built-in logging implementation.
- `serde`: Enable [Serde](https://crates.io.crates/serde) support for some 
           types. 
- `window`: Enable Wolf Engine's high-level window API.

### Basic Usage

See the [documentation](https://docs.rs/wolf_engine/latest), or the 
[Quick-Start example](examples/core_engine_basics.rs), for basic usage 
examples.

## License

Wolf Engine is licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

At your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0 
license, shall be dual licensed as above, without additional terms or 
conditions.


