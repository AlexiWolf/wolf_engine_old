//! Provides common helpers and utilities.

pub mod trust_cell;

pub use puffin::{current_file_name, current_function_name, profile_function, profile_scope};

/// Start a new [puffin] frame.
///
/// Equivalent to calling [puffin::GlobalProfiler::new_frame()].
pub fn profile_new_frame() {
    puffin::GlobalProfiler::lock().new_frame(); 
}
