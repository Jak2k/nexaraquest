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

fn render_location(location: &str) {
    // calculate the location indent (centered) using the width of the terminal
    let mut location_indent =
        (crossterm::terminal::size().unwrap().0 as i32 - location.len() as i32) / 2;

    if location_indent < 0 {
        location_indent = 0;
    }

    // Print the location
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Blue),
        // indent the location
        Print(" ".repeat(location_indent as usize)),
        Print(location),
        ResetColor,
        Print("\n\n"),
    )
    .unwrap();
}

fn render_options<T>(options: &Vec<crate::scene::Option<T>>) {
    execute!(std::io::stdout(), Print("\n")).unwrap();

    let mut next_option = 'A';

    for option in options {
        execute!(
            std::io::stdout(),
            Print("\n("),
            Print(next_option),
            Print(") "),
            SetForegroundColor(Color::Blue),
        )
        .unwrap();

        next_option = (next_option as u8 + 1) as char;

        typewriter(&option.title, 150);

        execute!(std::io::stdout(), ResetColor).unwrap();
    }

    execute!(
        std::io::stdout(),
        Print("\n\n(x)"),
        SetForegroundColor(Color::Blue),
        Print(" Exit"),
        ResetColor,
    )
    .unwrap();
}

pub fn render<T>(scene: &crate::scene::Scene<T>) {
    // ASCII art how the screen should look like this (the lines are not part of the output):
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

    // Print the location
    render_location(&scene.location);

    // Print the text with animation
    typewriter(&scene.text, 150);

    // Print the options
    render_options(&scene.options);
}
