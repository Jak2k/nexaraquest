pub struct Scene<ScenesData> {
    pub location: String,
    pub text: String,
    pub options: Vec<Option<ScenesData>>,
}

pub struct Option<ScenesData> {
    pub title: String,
    pub target: ScenesData,
}

pub trait Scenes<ScenesData: Scenes<ScenesData, Context>, Context> {
    fn get_current_scene(&self, context: &mut Context) -> Scene<ScenesData>;
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
