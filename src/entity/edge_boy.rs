use bevy::{
    color::Color,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_ecs::{component::Component, system::Commands};

use crate::entity::{
    collider::Collider,
    wall::{BOTTOM_WALL, WALL_THICKNESS},
};

#[derive(Component)]
pub struct EdgeBoy;

pub const EDGE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
pub const EDGE_BOY_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const EDGE_BOY_PADDING: f32 = 10.0;
pub const EDGE_BOY_SPEED: f32 = 500.0;

pub fn spawn(commands: &mut Commands) {
    let paddle_y = BOTTOM_WALL + EDGE_BOY_SIZE.y - WALL_THICKNESS / 2.;

    commands.spawn((
        Sprite::from_color(EDGE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: EDGE_BOY_SIZE.extend(1.0),
            ..default()
        },
        EdgeBoy,
        Collider,
    ));
}
