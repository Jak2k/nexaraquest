use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use color_eyre::Result;

fn typewriter(text: &str) -> Result<()> {
    const MAX_DELAY: u64 = 50;

    for c in text.chars() {
        execute!(std::io::stdout(), Print(c))?;
        std::thread::sleep(std::time::Duration::from_millis(
            rand::random::<u64>() % MAX_DELAY,
        ));
    }

    Ok(())
}

fn render_location(location: &str) -> Result<()> {
    // calculate the location indent (centered) using the width of the terminal
    let mut location_indent =
        (i32::from(crossterm::terminal::size()?.0) - location.len() as i32) / 2;

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
    ?;

    Ok(())
}

fn render_options<T>(options: &Vec<crate::scene::Option<T>>) -> Result<()> {
    execute!(std::io::stdout(), Print("\n"))?;

    let mut next_option = 'A';

    for option in options {
        std::thread::sleep(std::time::Duration::from_millis(200));

        execute!(
            std::io::stdout(),
            Print("\n("),
            Print(next_option),
            Print(") "),
            SetForegroundColor(Color::Blue),
        )
        ?;

        next_option = (next_option as u8 + 1) as char;

        typewriter(&option.title)?;

        execute!(std::io::stdout(), ResetColor)?;
    }

    execute!(
        std::io::stdout(),
        Print("\n\n(x)"),
        SetForegroundColor(Color::Blue),
        Print(" Exit"),
        ResetColor,
    )
    ?;

    Ok(())
}

pub fn render<T>(scene: &crate::scene::Scene<T>) -> Result<()> {
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
    ?;

    execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0))?;

    // Print the location
    render_location(&scene.location)?;

    // Print the text with animation
    typewriter(&scene.text)?;

    // Print the options
    render_options(&scene.options)?;

    Ok(())
}
