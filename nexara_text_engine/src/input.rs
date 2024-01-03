fn get_char() -> String {
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut input = String::new();

    loop {
        let key = crossterm::event::read().unwrap();

        if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char(c),
            ..
        }) = key
        {
            input.push(c);
            break;
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();

    input
}

#[must_use] pub fn input_letter(options_len: usize) -> usize {
    loop {
        let input = get_char();

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

        return index;
    }
}
