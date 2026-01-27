use bevy::{
    camera::Camera2d,
    color::Color,
    math::{Vec2, Vec3},
    mesh::Mesh,
    sprite::Sprite,
    sprite_render::ColorMaterial,
    transform::components::Transform,
    ui::Val,
    utils::default,
};
use bevy_asset::{AssetServer, Assets};
use bevy_ecs::{
    component::Component,
    system::{Commands, Res, ResMut},
};

use crate::entity::{death, edge_boy, game_state, layer, norm, wall};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    game_state::spawn(&mut commands);
    death::spawn(&mut commands);
    edge_boy::spawn(&mut commands);
    norm::spawn(&mut commands);
    wall::spawn(&mut commands);
    layer::spawn(&mut commands);
}
