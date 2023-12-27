use nexara_text_engine as nte;

fn main() {
    let scene = nte::scene::Scene {
        location: "PLACE".to_string(),
        text: "Text Text Text Text Text Text".to_string(),
        options: vec![
            nte::scene::Options {
                title: "A Option".to_string(),
            },
            nte::scene::Options {
                title: "B Option".to_string(),
            },
            nte::scene::Options {
                title: "C Option".to_string(),
            },
        ],
    };

    nte::render::render(&scene);
}
