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
    pub use color_eyre::Result;
    pub use serde::{Deserialize, Serialize};
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
        $body:expr,
        $initial_scene:expr,
        $initial_context:expr,
    ) => {
        #[derive(Serialize, Deserialize, Clone)]
        enum $enum_name {
            $($variant),*
        }

        #[derive(Serialize, Deserialize, Clone)]
        struct $struct_name {
            #[serde(default)]
            $($field: $type),*
        }

        impl Scenes<$enum_name, $struct_name> for $enum_name {

            fn get_current_scene(&self, context: &mut $struct_name) -> Result<Scene<$enum_name>> {
                Ok($body(self, context))
            }

            fn new() -> Self {
                $initial_scene
            }

            fn run(&mut self, context: &mut $struct_name)-> Result<()> {
                let mut old_context = context.clone();
                let mut scene = self.get_current_scene(context)?;

                {
                    use std::io::Write;
                    let mut file = std::fs::File::create("save.json").unwrap();
                    let save = serde_json::to_string(&(&*self, &old_context)).unwrap();
                    file.write_all(save.as_bytes()).unwrap();
                }

                loop {


                    nexara_text_engine::render::render(&scene)?;

                    let index =  nexara_text_engine::input::letter(scene.options.len())?;

                    old_context = context.clone();

                    // get the target scene
                    let target = scene.options[index].target.clone();
                    scene = target.get_current_scene(context)?;

                    use std::io::Write;
                    let mut file = std::fs::File::create("save.json").unwrap();
                    let save = serde_json::to_string(&(&target, &old_context)).unwrap();
                    file.write_all(save.as_bytes()).unwrap();
                }
            }
        }

        fn main() -> Result<()> {
            let mut scenes = $enum_name::new();
            let mut context = $initial_context;

            // load the context and the current scene
            // check if the file exists
            if std::path::Path::new("save.json").exists() {
                let file = std::fs::File::open("save.json").unwrap();
                let (scenes_, context_) = serde_json::from_reader(file).unwrap();
                scenes = scenes_;
                context = context_;
            };

            scenes.run(&mut context)
        }
    };
}
