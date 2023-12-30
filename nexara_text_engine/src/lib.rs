pub mod input;
pub mod render;
pub mod run_scene;
pub mod scene;
pub mod scenify;

pub mod prelude {
    pub use crate::game;
    pub use crate::run_scene;
    pub use crate::scene::{Option, Scene, Scenes};
    pub use crate::scenify;
}

#[macro_export]
macro_rules! game {
    (
        enum $enum_name:ident {
            $($variant:ident),* $(,)?
        },
        struct $struct_name:ident {
            $($field:ident: $type:ty),* $(,)?
        },
        $body:expr
    ) => {
        enum $enum_name {
            $($variant),*
        }

        struct $struct_name {
            $($field: $type),*
        }

        impl Scenes<$enum_name, $struct_name> for $enum_name {

            fn get_current_scene(&self, context: &mut $struct_name) -> Scene<$enum_name> {
                $body(self, context)
            }

            fn new() -> Self {
                $enum_name::Bedroom
            }

            fn run(&mut self, context: &mut $struct_name) {
                let mut scene = self.get_current_scene(context);

                loop {
                    nexara_text_engine::render::render(&scene);

                    let index =  nexara_text_engine::input::input_letter(scene.options.len());

                    // get the target scene
                    scene = scene.options[index].target.get_current_scene(context);
                }
            }
        }

        fn main() {
            let mut scenes = MyScenes::Bedroom;
            let mut context = MyContext {
                morning: true,
                heard_news: false,
            };

            scenes.run(&mut context);
        }
    };
}
