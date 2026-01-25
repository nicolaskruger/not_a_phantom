use bevy::{
    input::{ButtonInput, keyboard::KeyCode},
    time::Time,
    transform::components::Transform,
};
use bevy_ecs::{
    query::With,
    system::{Res, Single},
};

use crate::entity::{
    edge_boy::{EDGE_BOY_SIZE, EDGE_BOY_SPEED, EdgeBoy},
    game_state::{Game, GameState},
    wall::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};

pub fn move_edge_boy(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut edge_boy: Single<&mut Transform, With<EdgeBoy>>,
    game: Single<&mut Game, With<Game>>,
    time: Res<Time>,
) {
    match game.state {
        GameState::Playing => {
            let mut direction = 0.0;

            if keyboard_input.pressed(KeyCode::KeyH) {
                direction -= 1.0;
            }

            if keyboard_input.pressed(KeyCode::KeyL) {
                direction += 1.0;
            }

            let new_edge_boy_position =
                edge_boy.translation.x + direction * EDGE_BOY_SPEED * time.delta_secs();

            let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + EDGE_BOY_SIZE.x / 2.0;
            let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - EDGE_BOY_SIZE.x / 2.0;

            edge_boy.translation.x = new_edge_boy_position.clamp(left_bound, right_bound);
        }
        GameState::ItsOver => {}
    }
}
