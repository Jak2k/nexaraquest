# Nexara Quest & Nexara Text Engine

[![asciicast](https://asciinema.org/a/j97MRngAlp1TuQPJ34y9gDwl1.svg)](https://asciinema.org/a/j97MRngAlp1TuQPJ34y9gDwl1)

## What is Nexara Quest?

Nexara Quest will be a game about a teenager who lives in the city Nexara. He/her/they discovers their superpowers and has to use them to defend against people who want to use their powers for evil.

The game will be text based and will be written in Rust. A user has to use the terminal to play the game. (It's not that hard, I promise.)

## What is Nexara Text Engine?

Nexara Text Engine is a game engine for text based games. You basicly create 2 enums or structs and stick it into two macros:

- `scenify!` - This macro creates a scene from your data structures and a function that calculates what scene can be gone to next.
- `run_scene!` - This macro runs the start scene and allows the user to navigate.

```rust
use nexara_text_engine::prelude::*; // Import all the necessary stuff

enum MyScenes { // Create an enum for your scenes. Could also be a struct.
    Bedroom,
    Kitchen,
    School,
}

struct MyContext { // Create a struct for your context. Could also be an enum.
    morning: bool,
    heard_news: bool,
}

scenify!(
    MyScenes,
    MyContext,
    MyScenes::Bedroom, // The starting scene
    |this: &MyScenes, context: &mut MyContext| {
        match this { // This gives the current scene data from the active scene and context.
            MyScenes::Bedroom => Scene {
                location: "Bedroom".to_string(),
                text: match context.morning { // We can do more logic here...
                    true => {
                        context.morning = false; // ...and change the context.
                        "Your alarm is ringing. You are tired, but you have to go to school."
                            .to_string()
                    }
                    false => "You went into your bedroom.".to_string(),
                },
                options: vec![Option {
                    title: "Go to the kitchen".to_string(), // This is the text that is shown to the user.
                    target: MyScenes::Kitchen, // This is the next scene.
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
                text: "You are at school.".to_string(), // This doesn't need to be logic. A simple string is fine.
                options: vec![Option {
                    title: "Go home".to_string(),
                    target: MyScenes::Kitchen,
                }],
            },
            // We haven't forgotten a scene. Rust would have complained if we did.
        }
    }
);

fn main() {
    run_scene!( // Just sugar for running the start scene.
        MyScenes,
        MyContext {
            morning: true,
            heard_news: false
        }
    );
}
```

Check out the code in `nexara_text_engine/src`. It's not that much.
