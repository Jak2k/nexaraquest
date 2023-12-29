#[macro_export]
macro_rules! run_scene {
    ($scene_type:ty, $context:expr) => {
        let mut scene: $scene_type = Scenes::new();
        let mut context = $context;

        scene.run(&mut context);
    };
}
