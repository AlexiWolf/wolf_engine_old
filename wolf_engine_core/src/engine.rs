pub trait Context {}

#[cfg(test)]
mod engine_tests {
    use super::*;

    struct TestData {}

    impl TestData { 
        pub fn new() -> Self { Self }
    }

    impl Context for TestData {}
}
