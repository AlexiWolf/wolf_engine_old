# Wolf Engine

Wolf Engine is a game library written in Rust with a focus on flexability and
ease of use.  It provides solid default options and tools for those who just
want to make their game while also getting out of the way of those who want to
do things their way.

### Features

**Note**  Wolf Engine is still very much a W.I.P, so you should expect missing
features, bugs, changing APIs, and other spooky stuff until release 1.0.

These are the currently planned features for Wolf Engine.  Not all of them are
ready just yet, so this is more of a roadmap, than a feature list for now. 
Features will be checked off as they are implemented.

- [ ] Core
  - [x] Logging
  - [ ] Game States
  - [ ] Events / Listeners
  - [ ] Filesysten / Asset Loading
  - [ ] Input
  - [ ] ECS
  - [ ] Scenes / Prefabs
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

### Installation

Add Wolf Engine to your dependencies in `Cargo.toml`:

```TOML
[dependencies]
wolf_engine = "*"
```

### Getting Started

TODO: Write a "quick-start" guide.  I'm waiting to do this because the 
instructions will be chainging very soon.  Refer to the documentation in 
[lib.rs](src/lib.rs) for a current guide.

### License

Wolf Engine is licensed under either 

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

### Contribution

Unless you explicity state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without additional terms or conditions.

