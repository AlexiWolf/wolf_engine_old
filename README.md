# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

Wolf Engine is a game framework written in Rust with a focus on flexibility and ease of use.   It aims to provide 
sensible default workflows to those who just want to build a game while allowing custom options for those who don't 
want to be forced to do things *The Wolf Engine Way (TM)*.  

The main motivations for building Wolf Engine is to learn about how games / game engines work under the hood and to 
provide a production-ready system for future game projects.  A best-effort is made to ensure things work well, but I'm 
**not** an expert on game engines, or game development in general.  There will be mistakes, and there will be bad 
solutions while I stumble my way through the learning process.  Feedback and / or contributions is absolutely 
appreciated, so feel free to make an issue about anything you feel could be done better.

### Features

**Note**  Wolf Engine is still very much a W.I.P, so you should expect missing features, bugs, changing APIs, and 
other spooky stuff until release 1.0.  Things are also moving very fast right now, so you may be better off using 
something else until the API has stabilized a bit.

These are the features currently planned for Wolf Engine.  Not all of them are ready just yet, so this is more of a 
road map than a feature list for now.  Entries are checked off as they are implemented.

- [x] Core
  - [x] Core functions
  - [x] Dynamic Context data 
  - [x] Schedulers (timing controls)
  - [x] Game States
- [ ] Engine Modules
  - [x] Logging
  - [ ] Event
  - [ ] Input 
  - [ ] ECS
  - [ ] Windowing
  - [ ] Graphics
  - [ ] Audio
  - [ ] Networking 
  - [ ] FFI
  - [ ] Scripting
    - [ ] Lua scripting
    - [ ] Python scripting 

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
 - [The Documentation](https://docs.rs/wolf_engine/latest/wolf_engine/) 

And for more advanced usage, see:

 - [The Examples Folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples)

### License

Wolf Engine is licensed under either 

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.

