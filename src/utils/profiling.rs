/// Start a new profiler frame.
///
/// The [MainLoop](crate::MainLoop) should call this once per frame.
///
/// This macro is only active when the `profiling` feature is enabled.
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

/// Create a new profiler scope with the function name as the scope name.
///
/// This macro is only active when the `profiling` feature is enabled.
#[macro_export]
macro_rules! profile_function {
    () => {
        #[cfg(feature = "profiling")]
        puffin::profile_function!();
    };
    ($data:expr) => {
        #[cfg(feature = "profiling")]
        puffin::profile_function!($data);
    };
}

/// Create a new profiler scope with a custom name.
///
/// This macro is only active when the `profiling` feature is enabled.
#[macro_export]
macro_rules! profile_scope {
    ($id:expr) => {
        #[cfg(feature = "profiling")]
        puffin::profile_scope!($id);   
    };
    ($id:expr, $data:expr) => {
        #[cfg(feature = "profiling")]
        puffin::profile_scope!($x, $data);
    };
}

