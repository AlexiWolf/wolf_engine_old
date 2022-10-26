
#[cfg(test)]
mod engine_tests {
    struct TestData {}

    impl TestData { 
        pub fn new() -> Self { Self }

    }

    impl Context for TestData {}
}
