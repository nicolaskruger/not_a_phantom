use bevy::{
    input::{ButtonInput, keyboard::KeyCode},
    math::bounding::Aabb2d,
    time::Time,
    transform::components::Transform,
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, Res, Single},
};

use crate::{
    entity::{
        collider::Collider,
        edge_boy::{EDGE_BOY_SIZE, EDGE_BOY_SPEED, EdgeBoy},
        norm::Norm,
        wall::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
    },
    tool_kit::is_colliding::is_colliding,
};

pub fn collide_norm(
    norms_query: Query<(Entity, &Transform, Option<&Norm>), With<Collider>>,
    edge_boy_query: Single<(&mut Collider, &Transform), With<EdgeBoy>>,
) {
    let (_, edge_boy) = edge_boy_query.into_inner();

    let collision_edge_boy = Aabb2d::new(
        edge_boy.translation.truncate(),
        edge_boy.scale.truncate() / 2.,
    );

    norms_query.iter().for_each(|(_, norm, __)| {
        let collision_norm = Aabb2d::new(norm.translation.truncate(), norm.scale.truncate() / 2.);

        if is_colliding(&collision_edge_boy, &collision_norm) {
            print!("collide")
        }
    });
}
