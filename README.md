# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

Wolf Engine is a game framework for Rust with a focus on flexibility and ease of
use.   It aims to provide sensible default workflows to those who just want to 
build a game while allowing custom options for those who don't want to be forced
to do things *The Wolf Engine Way (TM)*.  

The main motivations for building Wolf Engine was to learn about how games / 
game engines work under the hood and to provide a production-ready system for 
future game projects.

### Features

**Note**  Wolf Engine is still very much a W.I.P, so you should expect missing
features, bugs, changing APIs, and other spooky stuff until release 1.0.

These are the currently planned features for Wolf Engine.  Not all of them are
ready just yet, so this is more of a road map than a feature list for now. 

Features will be checked off as they are implemented.

- [ ] Core
  - [x] Logging
  - [x] Game Loop
  - [x] Game States
  - [ ] Events / Listeners
  - [ ] Filesystem / Asset Loading
  - [ ] Input
  - [ ] ECS
  - [ ] Scenes / Prefabs
  - [ ] Python Scripting
- [ ] Graphics
  - [ ] Windowing
  - [ ] Low-level graphics 
  - [ ] High-level 2D graphics functions
  - [ ] High-level 3D graphics functions
- [ ] Audio
  - [ ] Low-level audio 
  - [ ] High-level audio functions
- [ ] Networking


## Quick-start 

### Installation

Add Wolf Engine to your dependencies in `Cargo.toml`:

```TOML
[dependencies]
wolf_engine = "*"
```

### Usage 

For basic usage, see:

 - [The Quickstart Example](https://github.com/AlexiWolf/wolf_engine/blob/main/examples/quickstart.rs)


And for more advanced usage, see:

 - [The Documentation](https://docs.rs/wolf_engine/latest/wolf_engine/) 
 - [The Examples Folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples).

### License

Wolf Engine is licensed under either 

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without additional terms or conditions.

