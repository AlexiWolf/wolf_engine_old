#[cfg(test)]
use mockall::automock;

use crate::EngineBuilder;

#[cfg_attr(test, automock)]
pub trait Plugin {
    fn setup(&mut self, engine_builder: EngineBuilder);
}

