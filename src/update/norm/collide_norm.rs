use bevy::{math::bounding::Aabb2d, transform::components::Transform};
use bevy_ecs::{
    query::With,
    system::{Query, Single},
};

use crate::{
    entity::{collider::Collider, edge_boy::EdgeBoy},
    tool_kit::is_colliding::is_colliding,
};

pub fn collide_norm(
    norms_query: Query<&Transform, With<Collider>>,
    edge_boy_query: Single<&Transform, With<EdgeBoy>>,
) {
    let edge_boy = edge_boy_query.into_inner();

    let collision_edge_boy = Aabb2d::new(
        edge_boy.translation.truncate(),
        edge_boy.scale.truncate() / 2.,
    );

    norms_query.iter().for_each(|norm| {
        let collision_norm = Aabb2d::new(norm.translation.truncate(), norm.scale.truncate() / 2.);

        if is_colliding(&collision_edge_boy, &collision_norm) {
            print!("collide")
        }
    });
}
