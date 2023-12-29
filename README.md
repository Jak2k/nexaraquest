# Nexara Quest & Nexara Text Engine

[![asciicast](https://asciinema.org/a/j97MRngAlp1TuQPJ34y9gDwl1.svg)](https://asciinema.org/a/j97MRngAlp1TuQPJ34y9gDwl1)

## What is Nexara Quest?

<!-- TODO: Add a description of Nexara Quest -->

## What is Nexara Text Engine?

Nexara Text Engine is a game engine for text based games. You basicly create a enum or struct and implement the `nexura_text_engine::scene::Scenes` trait. Then you can just create an instance of `nexura_text_engine::game::Scenes` as your enum or struct and call the `run()` method.

```rust
use nexara_text_engine::prelude::*;

enum MyScenes {
    Bedroom,
    Kitchen,
}

struct MyContext {
    morning: bool,
    heard_news: bool,
}

impl Scenes<MyScenes, MyContext> for MyScenes {
    fn get_current_scene(&self, context: &mut MyContext) -> Scene<MyScenes> {
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
                ],
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
```
