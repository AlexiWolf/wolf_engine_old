use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Context;

#[cfg_attr(test, mockall::automock)]
pub trait Scene<E: UserEvent> {
    fn update(&mut self, context: &mut Context<E>);
}

pub type SceneBox<E: UserEvent> = Box<dyn Scene<E>>;

pub struct Stage<E: UserEvent> {
    stack: Vec<Box<dyn Scene<E>>>, 
}

impl<E: UserEvent> Stage<E> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, scene: SceneBox<E>) {
        self.stack.push(scene); 
    }

    pub fn pop(&mut self) -> Option<SceneBox<E>> {
        self.stack.pop()
    }
}

impl<E: UserEvent> Scene<E> for Stage<E> {
    fn update(&mut self, context: &mut Context<E>) {
        todo!()
    }
}

#[cfg(test)]
mod scene_tests {
    use super::*; 

    #[test]
    fn should_push_and_pop_scenes() {
        let (_event_loop, context) = wolf_engine_core::init::<()>()
            .build();
        let mut stage = Stage::<()>::new();
        let scene = MockScene::new();

        stage.push(Box::from(scene));
        let scene = stage.pop();
        
       assert!(scene.is_some(), "No scene was returned."); 
    }

    #[test]
    fn should_delegate_to_scenes() {
        let (_event_loop, mut context) = wolf_engine_core::init::<()>().build();
        let mut stage = Stage::<()>::new();

        let mut scene = MockScene::<()>::new();
        scene.expect_update()
            .once()
            .return_const(());

        stage.push(Box::from(scene));
        stage.update(&mut context);
    }
}
