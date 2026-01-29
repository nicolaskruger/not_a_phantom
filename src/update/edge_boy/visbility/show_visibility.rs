use bevy::sprite::Sprite;
use bevy_ecs::{query::With, system::Single};

use crate::entity::edge_boy::{EDGE_COLOR, EDGE_INVISIBLE_COLOR, EdgeBoy};

pub fn show_visibility(edge_boy: Single<(&mut Sprite, &EdgeBoy), With<EdgeBoy>>) {
    let (mut sprite, entity) = edge_boy.into_inner();

    sprite.color = match entity.is_visible {
        true => EDGE_COLOR,
        false => EDGE_INVISIBLE_COLOR,
    }
}
