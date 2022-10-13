mod engine;
mod main_loop;
mod plugin;
mod plugin_loader;
mod state;
mod state_stack;
mod transition;

pub mod contexts;
pub mod events;
pub mod plugins;
pub mod schedulers;
pub mod stages;
pub mod utils;

#[cfg(feature = "logging")]
pub mod logging;

pub use engine::*;
pub use main_loop::*;
pub use plugin::*;
pub use plugin_loader::*;
pub use state::*;
pub use state_stack::*;
pub use transition::*;

use log::info;

pub(crate) fn log_startup_information() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    info!("Hello from {} v{} - {}", name, version, homepage);
}

pub(crate) fn log_shutdown() {
    info!("Engine has stopped.  Goodbye.")
}

#[cfg(test)]
use mockall::mock;
#[cfg(test)]
mock! {
    pub Subcontext{}

    impl wolf_engine_core::Subcontext for Subcontext {}
}
