use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

fn typewriter(text: &str, max_delay: u64) {
    for c in text.chars() {
        execute!(std::io::stdout(), Print(c)).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(
            rand::random::<u64>() % max_delay,
        ));
    }
}

pub fn render(scene: &crate::scene::Scene) {
    // ASCII art how the screen should look like
    /*

    +---------------------------------------+
    |                    PLACE              |
    |                                       |
    |     Text Text Text Text Text Text     |  (With a typewriter effect)
    |     Text Text Text Text Text Text     |
    |     Text Text Text Text Text Text     |
    |                                       |
    |  A Option                             |
    |  B Option                             |
    |  C Option                             |
    |                                       |
    +---------------------------------------+

    */

    // Clear the screen and reset the cursor
    execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();

    execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0)).unwrap();

    // calculate the location indent (centered) using the width of the terminal
    let mut location_indent =
        (crossterm::terminal::size().unwrap().0 as i32 - scene.location.len() as i32) / 2;

    if location_indent < 0 {
        location_indent = 0;
    }

    // Print the location
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Blue),
        // indent the location
        Print(" ".repeat(location_indent as usize)),
        Print(scene.location.clone()),
        ResetColor,
        Print("\n\n"),
    )
    .unwrap();

    // Print the text with animation
    typewriter(&scene.text, 150);

    // Print the options

    execute!(std::io::stdout(), Print("\n")).unwrap();

    for option in &scene.options {
        execute!(
            std::io::stdout(),
            Print("\n"),
            SetForegroundColor(Color::Blue),
        )
        .unwrap();

        typewriter(&option.title, 150);

        execute!(std::io::stdout(), ResetColor).unwrap();
    }
}
