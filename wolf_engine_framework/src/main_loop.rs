use wolf_engine_core::prelude::*;

/// Provides a wrapper around some [`MainLoop`] implementation, making it possible to access it as
/// a [`Resource`] at run-time.
pub(crate) struct MainLoopResource<E: UserEvent> {
    inner: Box<dyn MainLoop<E>>,
}

impl<E: UserEvent> MainLoopResource<E> {
    /// Creates a new resource from the provided [`MainLoop`].
    pub fn new<L: MainLoop<E> + 'static>(main_loop: L) -> Self {
        Self {
            inner: Box::from(main_loop),
        }
    }

    /// Sets the inner [`MainLoop`].
    pub fn set_main_loop(&mut self, main_loop: Box<dyn MainLoop<E>>) {
        self.inner = main_loop;
    }
    
    /// Consumes the resource, and returns a pointer to underlying [`MainLoop`].
    pub fn extract(self) -> Box<dyn MainLoop<E>> {
        self.inner
    }
}

/// An implementation of the engine's main-loop.
#[cfg_attr(test, mockall::automock)]
pub trait MainLoop<E: UserEvent> {
    /// Runs the main-loop until the engine quits.
    fn run(&mut self, engine: Engine<E>);
}

impl<E: UserEvent, T> MainLoop<E> for T
where
    T: FnMut(Engine<E>),
{
    fn run(&mut self, engine: Engine<E>) {
        (self)(engine)
    }
}

