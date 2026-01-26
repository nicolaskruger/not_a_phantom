use bevy::{
    input::keyboard::KeyCode,
    text::{TextColor, TextFont, TextSpan},
    transform::components::Transform,
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
    edge_boy::EdgeBoy,
    game_state::{Game, GameState},
};

pub fn show_death(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut edge_boy: Single<&mut Transform, With<EdgeBoy>>,
    mut game: Single<&mut Game, With<Game>>,
    mut commands: Commands,
) {
    if game.state == GameState::ShowDeath && keyboard_input.pressed(KeyCode::Space) {
        game.state = GameState::Playing;
    }
}
