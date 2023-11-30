use wolf_engine_core::events::UserEvent;

#[cfg_attr(test, mockall::automock)]
pub trait Scene<E: UserEvent> {

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
        
    }

    pub fn pop(&mut self) -> Option<SceneBox<E>> {
        None
    }
}

#[cfg(test)]
mod scene_tests {
    use super::*; 

    #[test]
    fn should_add_scene_to_stage() {
        let (_event_loop, context) = wolf_engine_core::init::<()>()
            .build();
        let mut stage = Stage::<()>::new();
        let scene = MockScene::new();

        stage.push(Box::from(scene));
        let scene = stage.pop();
        
       assert!(scene.is_some(), "No scene was returned."); 
    }
}
