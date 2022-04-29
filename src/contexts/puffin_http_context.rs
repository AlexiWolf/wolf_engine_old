use crate::*;

pub struct PuffinHttpContext {}

impl PuffinHttpContext {
    pub fn new() -> Result<Self, ()> {
        let http_context = Self {};
        Ok(http_context)
    }
}

impl Subcontext for PuffinHttpContext {}
