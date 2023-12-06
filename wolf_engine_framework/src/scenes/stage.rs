use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

use crate::scenes::state::*;
use crate::scenes::Scene;

/// Represents an action command for the [`Stage`].
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

/// Provides a stack-like structure managing a collection of [`Scene`] objects.
///
/// The Stage consists of a stack, on which the currently-loaded Scenes are stored.  Whatever
/// Scene is on the top of the stack is considered the "active" scene, and the rest are considered
/// "background" scenes.
///
/// When a Scene is pushed to the stack, it is first [loaded](Scene::load()), and when a Scene is
/// popped off the stack, it is [unloaded](Scene::unload()).
///
/// All Scenes run with the Stage, even the "background" Scenes, but only the "active" Scene is
/// able to return a [`SceneChange`] to control the Stage.  "Background" are only run through
/// their "background" methods, which do not return a [`SceneChange`].
pub struct Stage<E: UserEvent> {
    stack: Vec<Scene<E, Loaded>>,
}

impl<E: UserEvent> Stage<E> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Updates the whole [`Scene`] stack.
    ///
    /// Updates are run from bottom-to-top order.  Only the top scene has its [`Scene::update()`]
    /// method called, the rest get a [`Scene::background_update()`] instead.
    pub fn update(&mut self, context: &mut Context<E>) {
        self.run_background_updates(context);
        self.run_active_update(context);
    }

    /// Renders the whole [`Scene`] stack.
    ///
    /// Renders are run from bottom-to-top order.  Only the top scene has its [`Scene::render()`]
    /// method called, the rest get a [`Scene::background_render()`] instead.
    pub fn render(&mut self, context: &mut Context<E>) {
        self.run_background_renders(context);
        if let Some(scene) = self.stack.last_mut() {
            scene.render(context);
        }
    }

    /// Pushes a [`Scene`] to the top of the stack, and [loads](Scene::load()) it.
    pub fn push(&mut self, context: &mut Context<E>, scene: Scene<E, Unloaded>) {
        let scene = scene.load(context);
        self.stack.push(scene);
    }

    /// Pops a [`Scene`] off the top of the stack, and [unloads](Scene::unload()) it.
    pub fn pop(&mut self, context: &mut Context<E>) {
        if let Some(scene) = self.stack.pop() {
            scene.unload(context);
        }
    }

    /// Pops all [`Scene`] objects from the stack, and [unloads](Scene::unload()) them.
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

#[cfg(test)]
mod stage_tests {
    use crate::scenes::MockSceneTrait;

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
