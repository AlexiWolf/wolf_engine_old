//! Provides common helpers and utilities.

mod engine_controls;

pub use engine_controls::*;

#[cfg(feature = "profiling")]
pub use puffin::{current_file_name, current_function_name, profile_function, profile_scope};

/// Start a new [puffin] frame.
///
/// Equivalent to calling [puffin::GlobalProfiler::new_frame()].
///
/// # Examples
///
/// ```
/// # use wolf_engine::utils::*;
/// #
/// loop {
///     profile_new_frame(); // Start the new frame.
///     profile_scope!("frame"); // Start the frame scope.
///     // Update / Render / cool stuff.
/// #   break;
/// }
/// ```
pub fn profile_new_frame() {
    #[cfg(feature = "profiling")]
    puffin::GlobalProfiler::lock().new_frame();
}
