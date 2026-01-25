use bevy::{
    color::Color,
    text::{TextColor, TextFont, TextSpan},
    ui::{AlignSelf, JustifySelf, Node, PositionType, Val, widget::Text},
    utils::default,
};
use bevy_ecs::{children, component::Component, system::Commands};
use rand::{Rng, distr::uniform::SampleRange};

#[derive(Component)]
pub struct Death;

const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

fn death_talk<'a>() -> Vec<&'a str> {
    let phrases = vec![
        "Oh… that’s dark. Anyway.",
        "You’ll probably grow out of that.",
        "Yeah, I used to think like that when I was 15.",
        "That’s deep, I guess.",
        "Have you tried just… not caring?",
        "It’s not that serious.",
        "Everyone feels like that sometimes.",
        "That’s just how life is.",
        "You’re overthinking it.",
        "It’ll make sense when you’re older.",
        "Wow, that’s intense! So, what are you doing this weekend?",
        "That reminds me of a podcast I listened to.",
        "Cool aesthetic.",
        "You should really go outside more.",
        "Have you tried therapy?",
        "You should journal.",
        "Just build a routine.",
        "Wake up earlier, it helps.",
        "Delete social media for a while.",
        "Have you tried meditation?",
        "Everything happens for a reason.",
        "Good vibes only.",
        "It’s all about mindset.",
        "You attract what you think.",
        "Be grateful.",
        "That’s kinda cringe.",
        "Ok.",
        "Sure.",
    ];
    phrases
}

fn random_pick<'a>() -> &'a str {
    let mut rng = rand::rng();
    let talk = death_talk();

    let range_pick = 0..talk.len();

    if let Ok(key) = range_pick.sample_single(&mut rng) {
        talk[key]
    } else {
        "Sure."
    }
}

pub fn spawn(commands: &mut Commands) {
    commands.spawn((
        Text::new(random_pick()),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Death,
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}
