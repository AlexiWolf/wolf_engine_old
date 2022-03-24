# Wolf Engine
[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

Wolf Engine is a game framework that's designed to be flexible and easy to work with.

### Project Status

**Wolf Engine is in early alpha.**  The core API is mostly complete, and has started to stabilize somewhat, but it's 
still unproven *alpha* software.  You should expect missing features, bugs, changing APIs, and other spooky stuff until 
release 1.0.

In addition to being in alpha, I'm still actively learning about game and engine development.  A best-effort is made to 
ensure things work well, but I'm by no means an expert at this. I fully expect to make mistakes.  Feedback and / or  
contribution is absolutely welcome, so feel free to create an issue for anything you feel could be done better.

If you wish to contribute, please make sure you have read the [Contribution Guidelines](#Contribution).

### Design Goals

- **Simple:** Offer sensible default options and a clean and simple API.  Always remember to KISS.
- **Capable:** Build anything from small prototypes and game jams to full-featured production releases.
- **Flexible:** Allow users the freedom to bring their own tools and customize the engine to fit their project's needs.
- **Stable:** Utilize Rust's powerful type system and good BDD / TDD practices to squash bugs before they appear.
- **Capable:** Provide 2D and 3D support and many more useful features.
- **Light:** Every module except the core module is optional, so if it's not needed it won't be included.
- **Fast:** Code should strive to run as fast as possible.

### Features

These are the currently planned features for Wolf Engine.  Not all of them are ready yet, so this check-list is 
provided to show you what is and is not ready, and to help you decide if Wolf Engine is right for your project.  

- [x] Core 
  - [x] Core Functions
  - [x] Dynamic Context Data
  - [x] Timing Controls (Schedulers)
  - [x] Game States
  - [x] Plugins
- [x] Event
  - [x] Built-in (Core) Events
  - [x] Custom Events
- [x] Logging
- [ ] Input 
  - [ ] Keyboard / Mouse Input
  - [ ] Touch / Pen Input
  - [ ] Gamepad Input
  - [ ] Input-to-Action Map
- [ ] Window 
- [ ] Graphics
  - [ ] Low-level Graphics
  - [ ] High-level 2D Graphics
  - [ ] High-level 3D Graphics
  - [ ] Shaders
- [ ] Audio
- [ ] Networking
- [ ] FFI
  - [ ] C / C++ Bindings
  - [ ] Lua Scripting
  - [ ] Python Scripting 

## Getting Started

### Installation

Add Wolf Engine to the dependencies section in your `Cargo.toml`:

```TOML
[dependencies]
wolf_engine = "*"
```

### Usage 

For basic usage, see:

 - [The Quickstart Example](https://github.com/AlexiWolf/wolf_engine/blob/main/examples/quickstart.rs)
 - [The Documentation](https://docs.rs/wolf_engine/latest/wolf_engine/) 

And for more advanced usage, see:

 - [The Examples Folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples)

## Influences and Credits

## Contribution

### License

Wolf Engine is licensed under either

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.

