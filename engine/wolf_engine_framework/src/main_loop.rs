use wolf_engine_core::Engine;

/// An implementation of the engine's main-loop.
#[cfg_attr(test, mockall::automock)]
pub trait MainLoop {
    /// Runs the main-loop until the engine quits.
    fn run(&mut self, engine: Engine);
}

impl<T> MainLoop for T
where
    T: FnMut(Engine),
{
    fn run(&mut self, engine: Engine) {
        (self)(engine)
    }
}
