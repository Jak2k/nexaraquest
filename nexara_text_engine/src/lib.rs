// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod render;
pub mod scene;

pub fn input_letter() -> String {
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut input = String::new();

    loop {
        let key = crossterm::event::read().unwrap();

        match key {
            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                code: crossterm::event::KeyCode::Char(c),
                ..
            }) => {
                input.push(c);
                break;
            }
            _ => {}
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();

    input
}
