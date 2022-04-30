//! Provides common helpers and utilities.

pub mod trust_cell;

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
    puffin::GlobalProfiler::lock().new_frame();
}
