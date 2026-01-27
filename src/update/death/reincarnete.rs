use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy_ecs::{
    entity::{ContainsEntity, Entity},
    query::With,
    system::{Commands, Res, Single},
};

use crate::entity::{
    death::{Death, spawn as death_spawn},
    edge_boy::{EdgeBoy, spawn as edge_boy_spaw},
    game_state::{Game, GameState},
};

pub fn reincarnate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    edge_boy: Single<Entity, With<EdgeBoy>>,
    death: Single<Entity, With<Death>>,
    mut game: Single<&mut Game, With<Game>>,
    mut commands: Commands,
) {
    if game.state == GameState::ShowDeath && keyboard_input.pressed(KeyCode::Space) {
        commands.entity(edge_boy.entity()).despawn();
        commands.entity(death.entity()).despawn();
        edge_boy_spaw(&mut commands);

        game.state = GameState::Playing;
    }
}
