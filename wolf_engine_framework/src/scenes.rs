use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

#[allow(unused)]
#[cfg_attr(test, mockall::automock)]
pub trait Scene<E: UserEvent> {
    fn update(&mut self, context: &mut Context<E>);
    fn render(&mut self, context: &mut Context<E>);

    fn setup(&mut self, context: &mut Context<E>) {}
    fn shutdown(&mut self, context: &mut Context<E>) {}
    fn background_update(&mut self, context: &mut Context<E>) {}
}

pub type SceneBox<E> = Box<dyn Scene<E>>;

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
}

impl<E: UserEvent> Scene<E> for Stage<E> {
    fn update(&mut self, context: &mut Context<E>) {
        let stack_size = self.stack.len();
        for i in 0..stack_size - 1 {
            let scene = self.stack.get_mut(i)
                .unwrap()
                .background_update(context);
        }
        self.stack.last_mut()
            .unwrap()
            .update(context)
    }

    fn render(&mut self,context: &mut Context<E>) {
        self.stack.last_mut()
            .unwrap()
            .render(context)
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

        stage.push(&mut context, Box::from(scene));
        let scene = stage.pop(&mut context);

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
        let mut active_scene = MockScene::<()>::new();
        active_scene.expect_setup()
            .once()
            .return_const(());
        active_scene.expect_update()
            .once()
            .return_const(());
        active_scene.expect_render()
            .once()
            .return_const(());

        stage.push(&mut context, Box::from(background_scene));
        stage.push(&mut context, Box::from(active_scene));
        stage.update(&mut context);
        stage.render(&mut context);
    }
}
