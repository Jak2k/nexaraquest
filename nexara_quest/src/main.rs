use nexara_text_engine as nte;
use nte::scene::Scenes;

enum MyScenes {
    Bedroom,
    Kitchen,
    School,
}

struct MyContext {
    morning: bool,
    heard_news: bool,
}

impl nte::scene::Scenes<MyScenes, MyContext> for MyScenes {
    fn get_current_scene(&self, context: &mut MyContext) -> nte::scene::Scene<MyScenes> {
        use nte::scene::Option;
        use nte::scene::Scene;
        match self {
            MyScenes::Bedroom => Scene {
                location: "Bedroom".to_string(),
                text: match context.morning {
                    true => {
                        context.morning = false;
                        "Your alarm is ringing. You are tired, but you have to go to school."
                            .to_string()
                    }
                    false => "You went into your bedroom.".to_string(),
                },
                options: vec![Option {
                    title: "Go to the kitchen".to_string(),
                    target: MyScenes::Kitchen,
                }],
            },
            MyScenes::Kitchen => Scene {
                location: "Kitchen".to_string(),
                text: match context.heard_news {
                    true => "You are in the kitchen.".to_string(),
                    false => {
                        context.heard_news = true;
                        "You went into the kitchen. The radio is on and you hear the news."
                            .to_string()
                    }
                },
                options: vec![
                    Option {
                        title: "Go to the bedroom".to_string(),
                        target: MyScenes::Bedroom,
                    },
                    Option {
                        title: "Go to school".to_string(),
                        target: MyScenes::School,
                    },
                ],
            },
            MyScenes::School => Scene {
                location: "School".to_string(),
                text: "You are at school.".to_string(),
                options: vec![Option {
                    title: "Go home".to_string(),
                    target: MyScenes::Kitchen,
                }],
            },
        }
    }

    fn new() -> Self {
        MyScenes::Bedroom
    }
}

fn main() {
    let mut scene: MyScenes = Scenes::new();
    let mut context: MyContext = MyContext {
        morning: true,
        heard_news: false,
    };

    scene.run(&mut context);
}
