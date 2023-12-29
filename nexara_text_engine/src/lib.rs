pub mod input;
pub mod render;
pub mod run_scene;
pub mod scene;
pub mod scenify;

pub mod prelude {
    pub use crate::run_scene;
    pub use crate::scene::{Option, Scene, Scenes};
    pub use crate::scenify;
}
