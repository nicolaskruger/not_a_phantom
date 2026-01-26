use bevy::{
    text::{TextColor, TextFont, TextSpan},
    ui::{AlignSelf, JustifySelf, Node, PositionType, widget::Text},
    utils::default,
};
use bevy_ecs::{
    children,
    query::With,
    system::{Commands, Single},
};

use crate::entity::{
    death::{DEATH_COLOR, DEATH_FONT_SIZE, Death, TEXT_COLOR, random_pick},
    game_state::{Game, GameState},
};

pub fn show_death(mut game: Single<&mut Game, With<Game>>, mut commands: Commands) {
    if game.state == GameState::ItsOver {
        commands.spawn((
            Text::new(random_pick()),
            TextFont {
                font_size: DEATH_FONT_SIZE,
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
                    font_size: DEATH_FONT_SIZE,
                    ..default()
                },
                TextColor(DEATH_COLOR),
            )],
        ));

        game.state = GameState::ShowDeath;
    }
}
