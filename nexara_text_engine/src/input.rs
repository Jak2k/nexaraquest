use color_eyre::Result;

fn get_char() -> Result<String> {
    crossterm::terminal::enable_raw_mode()?;

    // discard previous input by reading it all
    while crossterm::event::poll(std::time::Duration::from_millis(1))? {
        crossterm::event::read()?;
    }
    
    let mut input = String::new();

    loop {
        let key = crossterm::event::read()?;

        if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char(c),
            ..
        }) = key
        {
            input.push(c);
            break;
        }
    }

    crossterm::terminal::disable_raw_mode()?;

    Ok(input)
}

pub fn letter(options_len: usize) -> Result<usize> {
    loop {
        let input = get_char()?;

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

        if index >= options_len {
            continue;
        }

        return Ok(index);
    }
}
