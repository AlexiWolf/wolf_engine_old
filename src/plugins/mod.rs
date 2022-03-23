use crate::{EngineBuilder, Plugin};

pub(crate) struct CorePlugin; 

impl Plugin for CorePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder {
        engine_builder
    }
}
