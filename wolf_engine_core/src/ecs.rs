//! Provides an Entity-Component-System based on [Legion](::legion).

pub use legion::*;
pub use wolf_engine_codegen::system;

#[doc(hidden)]
pub mod prelude {
    pub use super::system;
    pub use super::Schedule;
}
