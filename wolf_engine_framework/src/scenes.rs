//! Provides a scene system for the engine.

use std::marker::PhantomData;

use delegate::delegate;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

/// Provides type-state structs used by the [`Scene`].
pub mod state {
    /// A [`Scene`](super::Scene) type-state indicating the scene has not yet been loaded.
    pub struct Unloaded;

    /// A [`Scene`](super::Scene) type-state indicating the scene has been loaded.
    pub struct Loaded;
}

use state::*;

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

/// Represents all scene-change actions [`Stage`] can perform.
pub enum SceneChange<E: UserEvent> {
    /// Push a [`Scene`] to the top of the stack.
    Push(Scene<E, Unloaded>),

    /// Combines a [`SceneChange::Clear`], and a [`SceneChange::Push`] into a single operation.
    CleanPush(Scene<E, Unloaded>),

    /// Pop a single [`Scene`] off the top of the stack.
    Pop,

    /// Pop all [`Scenes`](Scene) off the stack.
    Clear,
}

/// Provides a stack-like structure for managing 1, or more [`Scene`] objects.
///
/// The stage consists of a `stack`, on which the currently-loaded Scenes are stored.  Whatever
/// Scene is on the top of the stack is considered the "active scene", and the rest are considered
/// "background scenes."  
///
/// While background scenes can still run updates, render, and some other operations (through
/// [`Scene::background_update()`], [`Scene::background_render`], ext. methods), the active scene
/// is the only scene that receives input events, or that can trigger a [`SceneChange`].
///
/// The main use-case for the stack-like structure, is to allow games to be composed of 1 or more
/// Scenes, which can be layered.  For example:  
///
/// 1. Main Gameplay Scene is loaded.  It is "active" and running the game.
/// 2. User interacts with an NPC, and the Gameplay Scene pushes an NPC Dialog Scene to the top of
///    the stack.
/// 4. The Dialog Scene becomes the "active" Scene, and begins receiving input.
/// 5. The Dialog Scene runs until the conversation is complete, then it pops itself off the stack.
/// 6. The Main Gameplay Scene is regains "active" status, and continues running the game.
///
/// This same idea could be carried to other Scenes, such as Inventory Screens, Pause Menus, ext.
pub struct Stage<E: UserEvent> {
    stack: Vec<Scene<E, Loaded>>,
}

impl<E: UserEvent> Stage<E> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a new [`Scene`] to the top of the stack, and calls its [`Scene::load()`] method.
    pub fn push(&mut self, context: &mut Context<E>, scene: Scene<E, Unloaded>) {
        let scene = scene.load(context);
        self.stack.push(scene);
    }

    /// Removes the [`Scene`] from the top of the stack, calls its [`Scene::unload()`] method,
    /// and returns the popped scene.
    pub fn pop(&mut self, context: &mut Context<E>) {
        if let Some(scene) = self.stack.pop() {
            scene.unload(context);
        }
    }

    /// Pops all [`Scene`] objects from the stack.
    pub fn clear(&mut self, context: &mut Context<E>) {
        for _ in 0..self.stack.len() {
            let _ = self.pop(context);
        }
    }

    fn run_background_updates(&mut self, context: &mut Context<E>) {
        let stack_size = self.stack.len();
        if stack_size > 1 {
            for i in 0..stack_size - 1 {
                self.stack.get_mut(i).unwrap().background_update(context);
            }
        }
    }

    fn run_background_renders(&mut self, context: &mut Context<E>) {
        let stack_size = self.stack.len();
        if stack_size > 1 {
            for i in 0..stack_size - 1 {
                self.stack.get_mut(i).unwrap().background_render(context);
            }
        }
    }

    fn run_active_update(&mut self, context: &mut Context<E>) {
        if let Some(scene) = self.stack.last_mut() {
            if let Some(scene_change) = scene.update(context) {
                match scene_change {
                    SceneChange::Push(new_scene) => self.push(context, new_scene),
                    SceneChange::CleanPush(new_scene) => {
                        self.clear(context);
                        self.push(context, new_scene);
                    }
                    SceneChange::Pop => {
                        let _ = self.pop(context);
                    }
                    SceneChange::Clear => self.clear(context),
                }
            }
        }
    }
}

impl<E: UserEvent> SceneTrait<E> for Stage<E> {
    /// Updates the whole [`Scene`] stack.
    ///
    /// Updates are run from bottom-to-top order.  Only the top scene has its [`Scene::update()`]
    /// method called, the rest get a [`Scene::background_update()`] instead.
    ///
    /// Unlike a normal [`Scene`], this implementation will always return [`None`].
    fn update(&mut self, context: &mut Context<E>) -> Option<SceneChange<E>> {
        self.run_background_updates(context);
        self.run_active_update(context);
        None
    }

    /// Renders the whole [`Scene`] stack.
    ///
    /// Renders are run from bottom-to-top order.  Only the top scene has its [`Scene::render()`]
    /// method called, the rest get a [`Scene::background_render()`] instead.
    fn render(&mut self, context: &mut Context<E>) {
        self.run_background_renders(context);
        if let Some(scene) = self.stack.last_mut() {
            scene.render(context);
        }
    }
}

#[cfg(test)]
mod stage_tests {
    use super::*;

    #[test]
    fn should_push_and_pop_scenes() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();
        let mut scene = MockSceneTrait::new();
        scene.expect_load().once().return_const(());
        scene.expect_unload().once().return_const(());
        let scene = Scene::<()>::new_unloaded(Box::from(scene));

        stage.push(&mut context, scene);
        stage.pop(&mut context);

        assert_eq!(stage.stack.len(), 0, "There should no scenes on the stack.")
    }

    #[test]
    fn should_delegate_to_scenes() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut background_scene = MockSceneTrait::<()>::new();
        background_scene.expect_load().once().return_const(());
        background_scene
            .expect_background_update()
            .once()
            .return_const(());
        background_scene
            .expect_background_render()
            .once()
            .return_const(());
        let background_scene = Scene::<()>::new_unloaded(Box::from(background_scene));
        let mut active_scene = MockSceneTrait::<()>::new();
        active_scene.expect_load().once().return_const(());
        active_scene.expect_update().once().returning(|_| None);
        active_scene.expect_render().once().return_const(());
        let active_scene = Scene::<()>::new_unloaded(Box::from(active_scene));

        stage.push(&mut context, background_scene);
        stage.push(&mut context, active_scene);
        stage.update(&mut context);
        stage.render(&mut context);
    }

    #[test]
    fn should_handle_push_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut new_scene = MockSceneTrait::new();
        new_scene.expect_load().once().return_const(());
        new_scene.expect_update().once().returning(|_| None);
        let new_scene = Scene::<()>::new_unloaded(Box::from(new_scene));
        let mut first_scene = MockSceneTrait::<()>::new();
        first_scene.expect_load().once().return_const(());
        first_scene
            .expect_update()
            .once()
            .return_once_st(|_| Some(SceneChange::Push(new_scene)));
        first_scene
            .expect_background_update()
            .once()
            .return_const(());
        let first_scene = Scene::<()>::new_unloaded(Box::from(first_scene));
        stage.push(&mut context, first_scene);

        for _ in 0..2 {
            stage.update(&mut context);
        }
    }

    #[test]
    fn should_handle_pop_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut scene = MockSceneTrait::<()>::new();
        scene.expect_load().once().return_const(());
        scene
            .expect_update()
            .once()
            .return_once_st(|_| Some(SceneChange::Pop));
        scene.expect_unload().once().return_const(());
        let scene = Scene::<()>::new_unloaded(Box::from(scene));
        stage.push(&mut context, scene);

        stage.update(&mut context);
    }

    #[test]
    fn should_handle_clean_pop_scene_chagne() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut new_scene = MockSceneTrait::new();
        new_scene.expect_load().once().return_const(());
        new_scene.expect_update().once().returning(|_| None);
        let new_scene = Scene::<()>::new_unloaded(Box::from(new_scene));
        let mut first_scene = MockSceneTrait::<()>::new();
        first_scene.expect_load().once().return_const(());
        first_scene
            .expect_update()
            .once()
            .return_once_st(|_| Some(SceneChange::CleanPush(new_scene)));
        first_scene.expect_unload().once().return_const(());
        let first_scene = Scene::<()>::new_unloaded(Box::from(first_scene));
        stage.push(&mut context, first_scene);

        for _ in 0..2 {
            stage.update(&mut context);
        }

        assert_eq!(
            stage.stack.len(),
            1,
            "There should only be 1 scene on the stack."
        )
    }

    #[test]
    fn should_handle_clear_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut second_scene = MockSceneTrait::new();
        second_scene.expect_load().once().return_const(());
        second_scene
            .expect_update()
            .once()
            .returning(|_| Some(SceneChange::Clear));
        second_scene.expect_unload().once().return_const(());
        let second_scene = Scene::<()>::new_unloaded(Box::from(second_scene));
        let mut first_scene = MockSceneTrait::<()>::new();
        first_scene.expect_load().once().return_const(());
        first_scene
            .expect_update()
            .once()
            .return_once_st(|_| Some(SceneChange::Push(second_scene)));
        first_scene
            .expect_background_update()
            .once()
            .return_const(());
        first_scene.expect_unload().once().return_const(());
        let first_scene = Scene::<()>::new_unloaded(Box::from(first_scene));
        stage.push(&mut context, first_scene);

        for _ in 0..2 {
            stage.update(&mut context);
        }

        assert_eq!(
            stage.stack.len(),
            0,
            "There should be no scenes left on the stack."
        )
    }

    #[test]
    fn should_not_panic_on_empty_stack() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        stage.update(&mut context);
        stage.render(&mut context);
    }
}
