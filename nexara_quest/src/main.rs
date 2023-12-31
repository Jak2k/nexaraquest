#![allow(non_camel_case_types)]
use nexara_text_engine::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
enum Superpower {
    None,
    Telekinesis,
}

game!(
    enum MyScenes {
        FirstMorining_Bedroom,
        FirstMorning_Kitchen,
        FirstMorning_Subway,
        FirstMorning_CarDad,
        FirstMorning_CarMom,
        FirstMorning_AttackSchool,
        FirstMorning_School,
    },
    struct MyContext {
        superpower: Superpower,
    },
    |this: &MyScenes, context: &mut MyContext| {
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
            MyScenes::FirstMorning_Subway => {
                context.superpower = Superpower::Telekinesis;
                Scene {
                location: "Nexara City > Subway".to_string(),
                text: r#"You decided to take the subway. The train rumbles along, and you find yourself immersed in the usual hustle and bustle of the city. However, things take a sudden turn as a trio of robbers enters your subway car.

One of them, with a menacing glint in their eye, summons knives and begins threatening passengers. Another flexes bulging muscles, capable of bending metal with ease, while the third mysteriously warps through walls and obstacles.

Panic sets in as the robbers demand money, and your heart races when a knife is pointed in your direction. In that moment, an unexpected surge of power courses through you. Without consciously realizing it, you find yourself manipulating the knife with an invisible force.The blade spins mid-air, catching the would-be assailant off guard, causing him to stagger back in pain.

As the subway car erupts in applause, you look around in amazement, finally noticing the newfound ability within you. The passengers, once terror-stricken, now thank you for your courageous intervention. The atmosphere shifts, and the robbers, realizing they've met their match, hastily retreat as the subway continues its journey through the city."#.to_string(),
                options: vec![
                    Option {
                        title: "Continue to school".to_string(),
                        target: MyScenes::FirstMorning_School,
                    }
                ],
            }},
            MyScenes::FirstMorning_CarDad => Scene {
                location: "Nexara City > Highway > Dad's Car".to_string(),
                text: r#"You decided to ask: "Dad, mind giving me a ride to school today?" Your dad, ever the reliable chauffeur, agrees, and you find yourself cruising down the highway. However, the usual commute takes an unexpected turn as chaos unfolds on the road.

ON THE HIGHWAY

A group of individuals, armed with extraordinary abilities, interrupts the flow of traffic, creating a scene of panic. One of them shoots fire into the air, causing drivers to scramble. Another seamlessly transforms into different people, making it difficult to discern friend from foe.

As the tension escalates, your instincts kick in. In a moment of urgency, you focus on the flames and, to your surprise, find yourself bending the fire away from the terrified drivers. The flames dance to your unspoken command, forming a protective barrier.

It's only when your dad exclaims, "Wow. When did you get that superpower?" that you realize the extent of your newfound abilities."#.to_string(),
                options: vec![
                    Option {
                        title: "Continue to school".to_string(),
                        target: MyScenes::FirstMorning_School,
                    }
                ],
            },
            MyScenes::FirstMorning_CarMom => todo!(),
            MyScenes::FirstMorning_School => todo!(),
            MyScenes::FirstMorning_AttackSchool => todo!(),
        }
    },
    MyScenes::FirstMorining_Bedroom,
    MyContext {
        superpower: Superpower::None,
    },
);
