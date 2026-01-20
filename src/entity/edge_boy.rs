use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_ecs::{component::Component, system::Commands};

use crate::entity::{collider::Collider, wall::BOTTOM_WALL};

#[derive(Component)]
pub struct EdgeBoy;

const EDGE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);

pub fn spawn(commands: &mut Commands) {
    let paddle_y = BOTTOM_WALL;

    commands.spawn((
        Sprite::from_color(EDGE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        EdgeBoy,
        Collider,
    ));
}
