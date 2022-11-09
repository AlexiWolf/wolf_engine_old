# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

Wolf Engine is a game framework that's designed to be flexible and easy to work with.

### Project Status

**Wolf Engine is in early alpha.** You should expect missing features, bugs, changing APIs, and other spooky stuff 
until release 1.0.

Feedback and / or contribution is absolutely welcome, so feel free to create an issue for anything you feel could be 
better. Before contributing, please make sure you have read the [Contribution Guidelines](#Contribution).

## Getting Started

[The documentation](https://docs.rs/wolf_engine/latest/wolf_engine/) provides an overview of the engine, and its APIs.
[The examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) offer practical, and more advanced 
usage examples.

## Design Goals

- **Simple:** Offer sensible default options and a clean and simple API.  Always remember to KISS.
- **Capable:** Build anything from small prototypes and game jams to full-featured production releases.
- **Flexible:** Allow users the freedom to bring their own tools and customize the engine to fit their project's needs.
- **Stable:** Utilize Rust's powerful type system and good BDD / TDD practices to squash bugs before they appear.
- **Light:** Every module except the core module is optional, so if it's not needed it won't be included.
- **Fast:** Code should strive to run as fast as possible.
- **Cross-Platform:** Run on as many platforms as possible.

## Platform Support 

Excellent cross-platform support is one of the main goals of Wolf Engine.  The entirety of the engine, with
`--all-features` enabled, should work on Desktop (Windows, Linux, MacOS), Mobile (Android, iOS), and WASM. Failure to 
build / run on these platforms is considered a bug.  Please create a bug report if you run into any problems.

### The Core Module 

The core module is intended to be a highly-portable subset of wolf engine enabling wider platform support, FFI, and 
support for no-std platforms.  The core module should theoretically run on any platform Rust itself can run on.  
However, for no-std platforms, you will very likely need to provide your own no-std-compatible `Context` data, and 
`EventLoop` implementation.

## License

Wolf Engine is licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

At your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.

