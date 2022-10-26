pub trait Context {}

pub struct Engine<C: Context> {
    context: C,
}

impl<C: Context> Engine<C> {
    pub fn new(context: C) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

#[cfg(test)]
mod engine_tests {
    use super::*;

    struct TestData {
        message: String,
    }

    impl TestData {
        pub fn new() -> Self {
            Self {
                message: "Hello, World!".to_string(),
            }
        }
    }

    impl Context for TestData {}

    #[test]
    fn should_provide_context_accessors() {
        let mut engine = Engine::new(TestData::new());

        assert_eq!(engine.context().message, "Hello, World!");
        engine.context_mut().message = "New message!".to_string();
        assert_eq!(engine.context().message, "New message!");
    }

    #[test]
    fn should_take_events() {
        let engine = Engine::new(TestData::new());

        while let Some(event) = engine.next_event() {
            
        }
    }
}
