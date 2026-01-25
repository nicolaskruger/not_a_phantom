use bevy::math::bounding::{Aabb2d, BoundingVolume};

pub fn is_colliding(a: &Aabb2d, b: &Aabb2d) -> bool {
    let ax = a.center().x;
    let a_half_size_x = a.half_size().x;

    let bx = b.center().x;
    let b_half_size_x = b.half_size().x;

    let ay = a.center().y;
    let a_half_size_y = a.half_size().y;

    let by = b.center().y;
    let b_half_size_y = b.half_size().y;

    let x_collide = ((ax + a_half_size_x) >= (bx - b_half_size_x)
        && (ax + a_half_size_x) <= (bx + b_half_size_x))
        || ((ax - a_half_size_x) >= (bx - b_half_size_x)
            && (ax - a_half_size_x) <= (bx + b_half_size_x));

    let y_collide = ((ay + a_half_size_y) >= (by - b_half_size_y)
        && (ay + a_half_size_y) <= (by + b_half_size_y))
        || ((ay - a_half_size_y) >= (by - b_half_size_y)
            && (ay - a_half_size_y) <= (by + b_half_size_y));

    x_collide && y_collide
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use super::*;

    #[test]
    fn do_not_collite() {
        let a_center = Vec2::new(0., 0.);
        let a_size = Vec2::new(1., 1.);
        let a = Aabb2d::new(a_center, a_size);

        let b_center = Vec2::new(3., 3.);
        let b_size = Vec2::new(1., 1.);
        let b = Aabb2d::new(b_center, b_size);

        assert!(!is_colliding(&a, &b));
    }

    #[test]
    fn do_not_same_y() {
        let a_center = Vec2::new(0., 0.);
        let a_size = Vec2::new(1., 1.);
        let a = Aabb2d::new(a_center, a_size);

        let b_center = Vec2::new(3., 0.);
        let b_size = Vec2::new(1., 1.);
        let b = Aabb2d::new(b_center, b_size);

        assert!(!is_colliding(&a, &b));
    }
    #[test]
    fn collite() {
        let a_center = Vec2::new(0., 0.);
        let a_size = Vec2::new(1., 1.);
        let a = Aabb2d::new(a_center, a_size);

        let b_center = Vec2::new(0., 0.);
        let b_size = Vec2::new(1., 1.);
        let b = Aabb2d::new(b_center, b_size);

        assert!(is_colliding(&a, &b));
    }
}
