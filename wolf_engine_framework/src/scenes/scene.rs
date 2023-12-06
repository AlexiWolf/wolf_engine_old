use std::marker::PhantomData;

use delegate::delegate;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

/// An alias for a [Boxed](Box) [`SceneTrait`].
pub type SceneBox<E> = Box<dyn SceneTrait<E>>;

/// Holds the main game logic, entities, loaded assets, ext. that make up a game scene.
///
/// # Examples
///
/// Detailed usage examples can be found in the examples folder.
#[allow(unused)]
#[cfg_attr(test, mockall::automock)]
pub trait SceneTrait<E: UserEvent> {
    /// Updates game state, and can send messages the [`Stage`] to change scenes.
    fn update(&mut self, context: &mut Context<E>) -> Option<SceneChange<E>>;

    /// Renders the current game state.
    fn render(&mut self, context: &mut Context<E>);

    /// Runs all preliminary setup required for the scene, such as initializing systems, spawning
    /// entities, loading assets, ext.
    fn load(&mut self, context: &mut Context<E>) {}

    /// Runs all tear-down operations required by the scene, such as removing resources, despawning
    /// entities, unloading assets, ext.
    fn unload(&mut self, context: &mut Context<E>) {}

    /// Updates the current state.
    ///
    /// This method is called when the scene is running in the background, such as when it is not
    /// the top scene on the [`Stage`].
    fn background_update(&mut self, context: &mut Context<E>) {}

    /// Renders the current game state.
    ///
    /// This method is called when the scene is running in the background, such as when it is not
    /// the top scene on the [`Stage`].
    fn background_render(&mut self, context: &mut Context<E>) {}
}

/// Provides type-state structs used by the [`Scene`].
pub mod state {
    /// A [`Scene`](super::Scene) type-state indicating the scene has not yet been loaded.
    pub struct Unloaded;

    /// A [`Scene`](super::Scene) type-state indicating the scene has been loaded.
    pub struct Loaded;
}

use state::*;

use super::SceneChange;

/// Provides a wrapper around a [`SceneTrait`], which can be either [`Unloaded`], or [`Loaded`],
/// and granting access to certain methods only after the scene has been loaded.
///
/// A Scene always starts in the [`Unloaded`] state.  In this state, most methods are inaccessible,
/// and only [`Scene::load()`] may be called.
///
/// Calling [`Scene::load()`] runs the Scene's one-time setup, then puts the Scene into the 
/// [`Loaded`] state.  Once in the [`Loaded`] state, the rest of the Scene's methods, save for
/// [`Scene::load()`] are made accessible.
///
/// Running the [`Scene::unload()`] method will consume the Scene, running it's one-time shutdown
/// code, and dropping the Scene.
pub struct Scene<E: UserEvent, State = Unloaded> {
    inner: SceneBox<E>,
    _state: PhantomData<State>,
}

impl<E: UserEvent, State> Scene<E, State> {
    /// Creates a new Scene, in the [`Unloaded`] state, with the provided [`SceneTrait`].
    pub fn new_unloaded(inner: SceneBox<E>) -> Scene<E, Unloaded> {
        Scene::<E, Unloaded> {
            inner,
            _state: PhantomData,
        }
    }
}

impl<E: UserEvent> Scene<E, Unloaded> {
    /// Loads the Scene, and puts it into the [`Loaded`] state.
    pub fn load(mut self, context: &mut Context<E>) -> Scene<E, Loaded> {
        self.inner.load(context);
        Scene::<E, Loaded> {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

impl<E: UserEvent> Scene<E, Loaded> {
    delegate! {
        to self.inner {
            pub fn update(&mut self, context: &mut Context<E>) -> Option<SceneChange<E>>;
            pub fn render(&mut self, context: &mut Context<E>);
            pub fn background_update(&mut self, context: &mut Context<E>);
            pub fn background_render(&mut self, context: &mut Context<E>);
            pub fn unload(mut self, context: &mut Context<E>);
        }
    }
}

#[cfg(test)]
mod scene_tests {
    use super::*;

    #[test]
    fn shutdown_should_consume_and_drop_scene() {
        let (_event_loop, mut context) = crate::init::<()>().build().unwrap();
        let mut inner = MockSceneTrait::<()>::new();
        inner.expect_load().once().return_const(());
        inner.expect_unload().once().return_const(());
        let scene = Scene::<()>::new_unloaded(Box::from(inner));

        let loaded_scene = scene.load(&mut context);
        loaded_scene.unload(&mut context);
    }
}

