

#[cfg(test)]
mod scene_tests {
    #[test]
    fn should_add_scene_to_stage() {
        let (_event_loop, context) = wolf_engine_core::init()
            .build();

        let stage = Stage::new();
        stage.push(MockScene);

        stage.update(&mut context);
    }
}
