#[macro_export]
macro_rules! scenify {
    ($scene_type:ty, $context_type:ty, $initial_scene:expr, $body:expr) => {

        impl Scenes<$scene_type, $context_type> for $scene_type {
            fn get_current_scene(self: &$scene_type, context: &mut $context_type) -> Scene<$scene_type> {
                $body(self, context)
            }

            fn new() -> Self {
                $initial_scene
            }
        }
    };
}
