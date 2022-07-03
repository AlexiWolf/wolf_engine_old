//! Provides common helpers and utilities.

mod engine_controls;

pub use engine_controls::*;

/// Start a new [puffin] frame.
///
/// Equivalent to calling [puffin::GlobalProfiler::new_frame()].
///
/// # Examples
///
/// ```
/// # use wolf_engine::*;
/// #
/// loop {
///     profile_new_frame!(); // Start the new frame.
///     profile_scope!("frame"); // Start the frame scope.
///     // Update / Render / cool stuff.
/// #   break;
/// }
/// ```
#[macro_export]
macro_rules! profile_new_frame {
    () => {
        #[cfg(feature = "profiling")]
        puffin::GlobalProfiler::lock().new_frame();
    };
}

#[macro_export]
macro_rules! profile_function {
    () => {
        #[cfg(feature = "profiling")]
        puffin::profile_function!();
    };
    ($x:expr) => {
        #[cfg(feature = "profiling")]
        puffin::profile_function!($x);
    };
}

#[macro_export]
macro_rules! profile_scope {
    () => {
        #[cfg(feature = "profiling")]
        puffin::profile_scope!();   
    };
    ($x:expr) => {
        #[cfg(feature = "profiling")]
        puffin::profile_scope!($x);
    };
}

