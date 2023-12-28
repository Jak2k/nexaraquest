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
    let scene: MyScenes = Scenes::new();
    let mut context: MyContext = MyContext {
        morning: true,
        heard_news: false,
    };
    let mut scene = scene.get_current_scene(&mut context);

    loop {
        nte::render::render(&scene);

        let input = nte::input_letter();

        // convert the input to an index
        let index = match input.as_str() {
            "x" => std::process::exit(0),
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            _ => continue,
        };

        // check if the index is valid
        if index >= scene.options.len() {
            continue;
        }

        // get the target scene
        scene = scene.options[index].target.get_current_scene(&mut context);
    }
}
