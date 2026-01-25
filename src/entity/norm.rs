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
pub struct Norm;

pub const NORM_COLOR: Color = Color::srgb(0.3, 0.0, 0.0);
pub const NORM_BOY_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const NORM_BOY_PADDING: f32 = 10.0;
pub const NORM_BOY_SPEED: f32 = 500.0;

pub fn spawn(commands: &mut Commands) {
    let paddle_y = BOTTOM_WALL + NORM_BOY_SIZE.y - WALL_THICKNESS / 2.;

    commands.spawn((
        Sprite::from_color(NORM_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: NORM_BOY_SIZE.extend(1.0),
            ..default()
        },
        Norm,
        Collider,
    ));
}
