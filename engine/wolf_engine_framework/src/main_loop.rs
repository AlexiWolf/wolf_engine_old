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

/// Provides a wrapper around some [`MainLoop`] implementation, making it possible to access it as
/// a [`Resource`] at run-time.
pub(crate) struct MainLoopResource {
    inner: Box<dyn MainLoop>,
}

impl MainLoopResource {
    /// Creates a new resource from the provided [`MainLoop`].
    pub fn new<L: MainLoop + 'static>(main_loop: L) -> Self {
        Self {
            inner: Box::from(main_loop),
        }
    }

    /// Sets the inner [`MainLoop`].
    pub fn set_main_loop(&mut self, main_loop: Box<dyn MainLoop>) {
        self.inner = main_loop;
    }

    /// Consumes the resource, and returns a pointer to underlying [`MainLoop`].
    pub fn extract(self) -> Box<dyn MainLoop> {
        self.inner
    }
}

