pub struct Scene<EnumOfScenes> {
    pub location: String,
    pub text: String,
    pub options: Vec<Option<EnumOfScenes>>,
}

pub struct Option<EnumOfScenes> {
    pub title: String,
    pub target: EnumOfScenes,
}

pub trait Scenes<EnumOfScenes, Context> {
    fn get_current_scene(&self, context: &mut Context) -> Scene<EnumOfScenes>;
    fn new() -> Self;
}
