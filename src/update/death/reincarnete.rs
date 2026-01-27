use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Res, Single},
};

use crate::entity::{
    death::Death,
    edge_boy::{EdgeBoy, spawn as edge_boy_spaw},
    game_state::{Game, GameState},
    layer::{Layer, layer_one::FirstLayer},
};

pub fn reincarnate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    edge_boy: Single<Entity, With<EdgeBoy>>,
    death: Single<Entity, With<Death>>,
    layer_one: Single<&mut FirstLayer, With<FirstLayer>>,
    mut game: Single<&mut Game, With<Game>>,
    mut commands: Commands,
) {
    if game.state == GameState::ShowDeath && keyboard_input.pressed(KeyCode::Space) {
        layer_one.dispose(edge_boy, death, &mut commands, &mut game);

        edge_boy_spaw(&mut commands);

        game.state = GameState::Playing;
    }
}
