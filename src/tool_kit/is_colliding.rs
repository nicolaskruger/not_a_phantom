use bevy::math::bounding::{Aabb2d, BoundingVolume};

pub fn is_colliding(a: &Aabb2d, b: &Aabb2d) -> bool {
    a.contains(b)
}
