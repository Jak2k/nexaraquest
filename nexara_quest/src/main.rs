#![allow(non_camel_case_types)]
use nexara_text_engine::prelude::*;

//ignoe case in the whole file

game!(
    enum MyScenes {
        FirstMorining_Bedroom,
        FirstMorning_Kitchen,
        FirstMorning_Subway,
        FirstMorning_CarDad,
        FirstMorning_CarMom,
        FirstMorning_School,
    },
    struct MyContext {},
    |this: &MyScenes, _context: &mut MyContext| {
        match this {
            MyScenes::FirstMorining_Bedroom => Scene {
                location: "Nexara City > Home > Your Bedroom".to_string(),
                text: "You wake up to the annoying buzz of your alarm, the relentless reminder that school is waiting. The warmth of your bed is tempting, but the clock's ticking, and you reluctantly drag yourself out from the cozy embrace of your blankets. Another day in the adventure called life, or, well, high school awaits. Time to face the world beyond the comfort of your bedroom.".to_string(),
                options: vec![
                    Option {
                        title: "Get out of bed".to_string(),
                        target: MyScenes::FirstMorning_Kitchen,
                    }
                ],
            },
            MyScenes::FirstMorning_Kitchen => Scene {
                location: "Nexara City > Home > Kitchen".to_string(),
                text: r#"You went into the kitchen. While you grab something to eat, the radio in the background crackles to life with the morning news.

"...tragic incident this morning as individuals with superpowers once again unleashed chaos among normal citizens. Several casualties reported. Authorities are urging heightened vigilance. In the city of Nexara, it's becoming increasingly clear that these superhuman confrontations pose a serious threat to public safety..."

Concern creeps in as you listen to the grim update. The subway doesn't sound so appealing right now. Maybe it's worth asking one of your parents to give you a lift to school today. The thought lingers as you ponder the safest way to navigate the city today."#.to_string(),
                options: vec![
                    Option {
                        title: "Take the subway".to_string(),
                        target: MyScenes::FirstMorning_Subway,
                    },
                    Option {
                        title: "Ask your dad for a ride".to_string(),
                        target: MyScenes::FirstMorning_CarDad,
                    },
                    Option {
                        title: "Ask your mom for a ride".to_string(),
                        target: MyScenes::FirstMorning_CarMom,
                    },
                ],
            },
            MyScenes::FirstMorning_Subway => todo!(),
            MyScenes::FirstMorning_CarDad => todo!(),
            MyScenes::FirstMorning_CarMom => todo!(),
            MyScenes::FirstMorning_School => todo!(),
        }
    },
    MyScenes::FirstMorining_Bedroom,
    MyContext {},
);
