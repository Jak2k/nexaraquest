pub struct Scene<EnumOfScenes> {
    pub location: String,
    pub text: String,
    pub options: Vec<Option<EnumOfScenes>>,
}

pub struct Option<EnumOfScenes> {
    pub title: String,
    pub target: EnumOfScenes,
}

pub trait Scenes<EnumOfScenes: Scenes<EnumOfScenes, Context>, Context> {
    fn get_current_scene(&self, context: &mut Context) -> Scene<EnumOfScenes>;
    fn new() -> Self;
    fn run(&mut self, context: &mut Context) {
        let mut scene = self.get_current_scene(context);

        loop {
            crate::render::render(&scene);

            let index = crate::input_letter(scene.options.len());

            // get the target scene
            scene = scene.options[index].target.get_current_scene(context);
        }
    }
}
