# Wolf Engine

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
ready just yet, so this is more of a roadmap, than a feature list for now. 
Features will be checked off as they are implemented.

- [ ] Core
  - [x] Logging
  - [x] Game Loop
  - [x] Game States
  - [ ] Events / Listeners
  - [ ] Filesysten / Asset Loading
  - [ ] Input
  - [ ] ECS
  - [ ] Scenes / Prefabs
  - [ ] Python Scripting
- [ ] Graphics
  - [ ] Windowing
  - [ ] Low-level graphics 
  - [ ] High-level 2D graphics functions
  - [ ] High-level 3D graphics functions
  - [ ] Cameras
- [ ] Audio
  - [ ] Low-level graphics
  - [ ] High-level audio functions
- [ ] Networking


## Getting Started

### Installation

Add Wolf Engine to your dependencies in `Cargo.toml`:

```TOML
[dependencies]
wolf_engine = "*"
```

### License

Wolf Engine is licensed under either 

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without additional terms or conditions.

