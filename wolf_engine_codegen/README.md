# Wolf Engine Code-gen

A proc-macro crate for [Wolf Engine](https://github.com/AlexiWolf/wolf_engine).

## Third-Party

This crate directly uses / modifies code from the following projects:

### [Legion](https://github.com/amethyst/)

- **[Commit](https://github.com/amethyst/legion/tree/0d0a53953465ca73e6f9231ecbae9044bf4b4389)**
- **License**: [MIT](LICENSE_LEGION)
- **Code Used**: The `system` macro found in 
  [`codegen/src/lib.rs`](https://github.com/amethyst/legion/blob/0d0a53953465ca73e6f9231ecbae9044bf4b4389/codegen/src/lib.rs).
- **Changes Made**: Replaced `legion::` with `wolf_engine::ecs::` to fix
  breakage with re-exports.
