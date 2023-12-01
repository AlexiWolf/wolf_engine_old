use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

/// An alias for a [Boxed](Box), [Scene].  To make for cleaner code.
pub type SceneBox<E> = Box<dyn Scene<E>>;

/// Holds the main game logic, entities, loaded assets, ext. that make up a game scene.
///
/// TODO: Explain how Scenes are used by the engine.
/// 
/// # Examples
///
/// TODO: Show how to implement a Scene, and how they are used.
#[allow(unused)]
#[cfg_attr(test, mockall::automock)]
pub trait Scene<E: UserEvent> {
    fn update(&mut self, context: &mut Context<E>) -> Option<SceneChange<E>>;
    fn render(&mut self, context: &mut Context<E>);

    fn setup(&mut self, context: &mut Context<E>) {}
    fn shutdown(&mut self, context: &mut Context<E>) {}
    fn background_update(&mut self, context: &mut Context<E>) {}
    fn background_render(&mut self, context: &mut Context<E>) {}
}

/// Represents all scene-change actions [`Stage`] can perform.
pub enum SceneChange<E: UserEvent> {
    Push(SceneBox<E>),
    CleanPush(SceneBox<E>),
    Pop,
    Clear,
}

/// Provides a stack-like structure for managing 1, or more [`State`] objects.
///
/// TODO: Explain how the stack system works, and why users may want to stack scenes in the first
/// place.
/// 
/// # Examples
///
/// TODO: Provide examples on how to use the Stage, and how to control it from an active Scene.
pub struct Stage<E: UserEvent> {
    stack: Vec<Box<dyn Scene<E>>>, 
}

impl<E: UserEvent> Stage<E> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, context: &mut Context<E>, mut scene: SceneBox<E>) {
        scene.setup(context);
        self.stack.push(scene); 
    }

    pub fn pop(&mut self, context: &mut Context<E>) -> Option<SceneBox<E>> {
        match self.stack.pop() {
            Some(mut scene) => {
                scene.shutdown(context);
                Some(scene)
            },
            None => None, 
        }
    }

    pub fn clear(&mut self, context: &mut Context<E>) {
        for _ in 0..self.stack.len() {
            let _ = self.pop(context);
        }
    }

    fn run_background_updates(&mut self, context: &mut Context<E>) {
        let stack_size = self.stack.len();
        if stack_size > 1 {
            for i in 0..stack_size - 1 {
                self.stack.get_mut(i)
                    .unwrap()
                    .background_update(context);
            }
        }
    }

    fn run_background_renders(&mut self, context: &mut Context<E>) {
        let stack_size = self.stack.len();
        if stack_size > 1 {
            for i in 0..stack_size - 1 {
                self.stack.get_mut(i)
                    .unwrap()
                    .background_render(context);
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
                    SceneChange::Pop => { let _ = self.pop(context); },
                    SceneChange::Clear => self.clear(context),
                }
            }
        }
    }
}

impl<E: UserEvent> Scene<E> for Stage<E> {
    fn update(&mut self, context: &mut Context<E>) -> Option<SceneChange<E>> {
        self.run_background_updates(context);
        self.run_active_update(context);
        None
    }

    fn render(&mut self,context: &mut Context<E>) {
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
        let mut scene = MockScene::new();
        scene.expect_setup()
            .once()
            .return_const(());
        scene.expect_shutdown()
            .once()
            .return_const(());

        stage.push(&mut context, Box::from(scene)); let scene = stage.pop(&mut context);

        assert!(scene.is_some(), "No scene was returned."); 
    }

    #[test]
    fn should_delegate_to_scenes() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut background_scene = MockScene::<()>::new();
        background_scene.expect_setup()
            .once()
            .return_const(());
        background_scene.expect_background_update()
            .once()
            .return_const(());
        background_scene.expect_background_render()
            .once()
            .return_const(());
        let mut active_scene = MockScene::<()>::new();
        active_scene.expect_setup()
            .once()
            .return_const(());
        active_scene.expect_update()
            .once()
            .returning(|_| { None });
        active_scene.expect_render()
            .once()
            .return_const(());

        stage.push(&mut context, Box::from(background_scene));
        stage.push(&mut context, Box::from(active_scene));
        stage.update(&mut context);
        stage.render(&mut context);
    }

    #[test]
    fn should_handle_push_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut new_scene = MockScene::new();
        new_scene.expect_setup()
            .once()
            .return_const(());
        new_scene.expect_update()
            .once()
            .returning(|_| { None });
        let mut first_scene = MockScene::<()>::new();
        first_scene.expect_setup()
            .once()
            .return_const(());
        first_scene.expect_update()
            .once()
            .return_once(|_| { Some(SceneChange::Push(Box::from(new_scene))) });
        first_scene.expect_background_update()
            .once()
            .return_const(());
        stage.push(&mut context, Box::from(first_scene));

        for _ in 0..2 {
            stage.update(&mut context);
        }
    }

    #[test]
    fn should_handle_pop_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut scene = MockScene::<()>::new();
        scene.expect_setup()
            .once()
            .return_const(());
        scene.expect_update()
            .once()
            .return_once(|_| { Some(SceneChange::Pop) });
        scene.expect_shutdown()
            .once()
            .return_const(());
        stage.push(&mut context, Box::from(scene));

        stage.update(&mut context);
    }

    #[test]
    fn should_handle_clean_pop_scene_chagne() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut new_scene = MockScene::new();
        new_scene.expect_setup()
            .once()
            .return_const(());
        new_scene.expect_update()
            .once()
            .returning(|_| { None });
        let mut first_scene = MockScene::<()>::new();
        first_scene.expect_setup()
            .once()
            .return_const(());
        first_scene.expect_update()
            .once()
            .return_once(|_| { Some(SceneChange::CleanPush(Box::from(new_scene))) });
        first_scene.expect_shutdown()
            .once()
            .return_const(());
        stage.push(&mut context, Box::from(first_scene));

        for _ in 0..2 {
            stage.update(&mut context);
        }

        assert_eq!(stage.stack.len(), 1, "There should only be 1 scene on the stack.")
    }

    #[test]
    fn should_handle_clear_scene_change() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut second_scene = MockScene::new();
        second_scene.expect_setup()
            .once()
            .return_const(());
        second_scene.expect_update()
            .once()
            .returning(|_| { Some(SceneChange::Clear) });
        second_scene.expect_shutdown()
            .once()
            .return_const(());
        let mut first_scene = MockScene::<()>::new();
        first_scene.expect_setup()
            .once()
            .return_const(());
        first_scene.expect_update()
            .once()
            .return_once(|_| { Some(SceneChange::Push(Box::from(second_scene))) });
        first_scene.expect_background_update()
            .once()
            .return_const(());
        first_scene.expect_shutdown()
            .once()
            .return_const(());
        stage.push(&mut context, Box::from(first_scene));

        for _ in 0..2 {
            stage.update(&mut context);
        }

        assert_eq!(stage.stack.len(), 0, "There should be no scenes left on the stack.")
    }

    #[test]
    fn should_not_panic_on_empty_stack() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        stage.update(&mut context);
        stage.render(&mut context);
    }

}
