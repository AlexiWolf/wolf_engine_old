pub use wolf_engine_core::*;

#[cfg(feature = "framework")]
pub use wolf_engine_framework as framework;

#[cfg(feature = "logging")]
pub use wolf_engine_framework::logging;

pub mod prelude {
    //! Provides the prelude for Wolf Engine.
    pub use super::*;
    #[cfg(feature = "framework")]
    pub use framework::*;
}
