# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

## Table of Contents 

<!--toc:start-->
- [Wolf Engine](#wolf-engine)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
    - [Status](#status)
    - [Features](#features)
  - [Quick-Start Guide](#quick-start-guide)
    - [Install](#install)
    - [Crate Features](#crate-features)
    - [Basic Usage](#basic-usage)
  - [License](#license)
  - [Contribution](#contribution)
<!--toc:end-->

## Introduction 

A simple, and flexible game framework written in Rust.

Usage documentation, and interactive examples can be found on 
[docs.rs](https://docs.rs/wolf_engine/latest/), and in the 
[examples/](examples/) directory.

### Status

Wolf Engine is currently in very early development.  You should expect missing
features, bugs, changing APIs, and other spooky stuff until release 1.0.

### Features

- [ ] Hardware-accelerated 2D, and 3D graphics.
- [ ] Back-end agnostic input events, and input state provided by 
      [Input Helper](https://crates.io/crates/input_helper/).
- [ ] High-level [Framework](https://docs.rs/wolf_engine_framework/latest/)
      providing many convenient features.
- [ ] Fixed updates, inspired by 
      [Fix Your Timestep!](https://www.gafferongames.com/post/fix_your_timestep/).
- [ ] Hot-reloading, to help speed up game development.

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


