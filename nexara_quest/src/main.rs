#![allow(non_camel_case_types)]
use nexara_text_engine::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
enum Superpower {
    None,
    Telekinesis,
    FireBending,
}

const COME_BACK_LATER: &str = r#"This scene is not written yet. Watch out for an update.

https://github.com/Jak2k/nexaraquest"#;

game!(
    enum MyScenes {
        FirstMorining_Bedroom,
        FirstMorning_Kitchen,
        FirstMorning_Subway,
        FirstMorning_CarDad,
        FirstMorning_CarMom,
        FirstMorning_SchoolNoAttack, // Leads to ClassRoom where people attack
        FirstMorning_School,
        FirstMorning_School_PrincipalsOffice,
        FirstMorning_ClassRoom,
        FirstMorning_ClassRoom_Defend,

        NewAtRadiantOrder,

        KiddnapedByShadowSyndicate_Cell,
        KiddnapedByShadowSyndicate_Cell_CallHelp,
        KiddnapedByShadowSyndicate_Cell_Wait,
        KiddnapedByShadowSyndicate_Interrogation,
        KiddnapedByShadowSyndicate_Decline,
        KiddnapedByShadowSyndicate_Accept,
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
            MyScenes::FirstMorning_CarDad => {
                context.superpower = Superpower::FireBending;
                Scene {
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
            }},
            MyScenes::FirstMorning_CarMom => Scene {
                location: "Nexara City > Highway > Mom's Car".to_string(),
                text: r#"You decided to ask: "Mom, mind giving me a ride to school today?" Your mom, preoccupied with a phone call, gives a distracted nod of approval, and you find yourself navigating the city streets. The journey proceeds without any unusual incidents, and as you reach the school, your mom bids you farewell with a quick, absentminded "Have a good day."

Exiting the car, you wonder if today will be as uneventful as the drive to school."#.to_string(),
                options: vec![
                    Option {
                        title: "Enter the school building".to_string(),
                        target: MyScenes::FirstMorning_SchoolNoAttack,
                    }
                ],
            },
            MyScenes::FirstMorning_School => Scene {
                location: "Nexara City > School".to_string(),
                text: r#"Arriving at school, the mundane routine of classes looms ahead, but boredom quickly dissipates when the secretary of the principal approaches you with an unexpected message.

"Two police officers are waiting to speak with you regarding the incident on your way to school," she says, her tone serious. Curiosity mingled with a hint of nervousness fills the air as you follow her to the principal's office, wondering what this unexpected turn of events might entail."#.to_string(),
                options: vec![
                    Option {
                        title: "Enter the principal's office".to_string(),
                        target: MyScenes::FirstMorning_School_PrincipalsOffice,
                    }
                ],
            },
            MyScenes::FirstMorning_School_PrincipalsOffice => Scene {
                location: "Nexara City > School > Principal's Office".to_string(),
                text: r#"The so-called cops ask the principal to give you some privacy, and as soon as the door clicks shut, they drop the act. "We ain't really cops; we're from the 'Radiant Order.' We're the good guys, you know? Fighting against the people who rob and injure the people all over the city and all that jazz."

They lean in, keeping it hush-hush. "Look, after what you did on your way here, we reckon you've got some serious potential. We're short on hands, and we could use someone like you. Wanna join the fight against the 'Shadow Syndicate'? Or, you know, you can just head back to class if you're not feeling it. Your call.""#.to_string(),
                options: vec![
                    Option {
                        title: "Join the Radiant Order".to_string(),
                        target: MyScenes::NewAtRadiantOrder,
                    },
                    Option {
                        title: "Go back to class".to_string(),
                        target: MyScenes::FirstMorning_ClassRoom,
                    }
                ],
            },
            MyScenes::FirstMorning_SchoolNoAttack => Scene {
                location: "Nexara City > School".to_string(),
                text: r#"After you enter the school, the buzz of conversations about the recent attacks fills the hallway. You overhear snippets of worried discussions as students share their thoughts on the unsettling news. The atmosphere is charged with a mix of curiosity and apprehension.

Navigating through the crowd, you catch glimpses of concerned faces and hushed whispers about the escalating situation in the city. It seems the events from the morning have stirred a collective unease among your peers."#.to_string(),
                options: vec![
                    Option {
                        title: "Enter the classroom".to_string(),
                        target: MyScenes::FirstMorning_ClassRoom,
                    }
                ]
            },
            MyScenes::FirstMorning_ClassRoom => Scene {
                location: "Nexara City > School > Classroom".to_string(),
                text: r#"A few minutes later, your math teacher enters the room and starts droning on about the Pythagorean theorem, the monotony of the lesson lulling you into a sense of normalcy. Just as you resign yourself to the tedium, the classroom is abruptly thrust into chaos.

Without warning, a man warps into the room, brandishing a gun aimed directly at you. Simultaneously, another intruder, appearing through the door, conjures weapons out of thin air and points them menacingly at your startled classmates. The sudden intrusion disrupts the mundane routine, injecting an element of danger into the once mundane classroom.

Adding to the surreal scene, a third attacker, a woman, crashes through the window, clutching a handful of grenades. The air grows tense as the unexpected assailants assert control over the classroom, leaving you and your classmates at the mercy of this bizarre and threatening situation.

The man with the gun now wants you to go with him, his voice carrying a dangerous edge. "You're coming with us, kid. Don't try anything funny, or your classmates will be in for a nasty surprise.""#.to_string(),
                options: vec![
                    Option {
                        title: "Go with the man".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Cell,
                    }
                ],
            },
            MyScenes::FirstMorning_ClassRoom_Defend => Scene {
                location: "Nexara City > School > Classroom".to_string(),
                text: r#"You decide to defend yourself and your classmates, but you fail. The attackers shoot your best friend and the men shouts: "Come with us or we will kill everyone in this room!""#.to_string(),
                options: vec![
                    Option {
                        title: "Go with the man".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Cell,
                    },
                    Option {
                        title: "Try to defend".to_string(),
                        target: MyScenes::FirstMorning_ClassRoom_Defend,
                    }
                ]
            },
            MyScenes::NewAtRadiantOrder => Scene {
                // TODO
                location: "A Mystery".to_string(),
                text: COME_BACK_LATER.to_string(),
                options: vec![],
            },
            MyScenes::KiddnapedByShadowSyndicate_Cell => Scene {
                location: "Nexara City > Shadow Syndicate Headquarter > Prison Cell".to_string(),
                text: r#"You awaken in a dark, unfamiliar cell, the cold surroundings closing in. Fear grips you as you try to make sense of your surroundings. The air is heavy with uncertainty, and the absence of light intensifies the feeling of isolation. In the disorienting darkness, your senses strain, and every sound echoes, heightening the sense of vulnerability. The unknown looms, and anxiety tightens its grip as you grapple with the unsettling reality of your situation."#.to_string(),
                options: vec![
                    Option {
                        title: "Call for help".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Cell_CallHelp,
                    },
                    Option {
                        title: "Wait".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Cell_Wait,
                    },
                ],
            },
            MyScenes::KiddnapedByShadowSyndicate_Cell_CallHelp => Scene {
                location: "Nexara City > Shadow Syndicate Headquarter > Prison Cell".to_string(),
                text: r#""Hey, someone! Help!" you call out, your voice echoing in the dimness. Fear still clings to you, a palpable presence in the dark cell.


Before long, the man from the classroom appears, a silhouette in the shadows. Without a word, he gestures for you to follow him, leading you out of the oppressive cell and into another room. "#.to_string(),
                options: vec![
                    Option {
                        title: "Follow him".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Interrogation,
                    },
                ],
            },
            MyScenes::KiddnapedByShadowSyndicate_Cell_Wait => Scene {
                location: "Nexara City > Shadow Syndicate Headquarter > Prison Cell".to_string(),
                text: r#"You wait in the dim cell, anxiety lingering in the air. After what feels like an eternity, the man from the classroom appears. Without uttering a word, he motions for you to follow him."#.to_string(),
                options: vec![
                    Option {
                        title: "Follow him".to_string(),
                        target: MyScenes::KiddnapedByShadowSyndicate_Interrogation,
                    },
                ],
            },
            MyScenes::KiddnapedByShadowSyndicate_Interrogation => todo!(),
            MyScenes::KiddnapedByShadowSyndicate_Decline => todo!(),
            MyScenes::KiddnapedByShadowSyndicate_Accept => todo!(),
        }
    },
    MyScenes::FirstMorining_Bedroom,
    MyContext {
        superpower: Superpower::None,
    },
);
