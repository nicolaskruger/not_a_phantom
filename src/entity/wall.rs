use bevy::{
    color::Color, math::Vec2, sprite::Sprite, transform::components::Transform, utils::default,
};
use bevy_ecs::{component::Component, system::Commands};

use crate::entity::collider::Collider;

pub const BOTTOM_WALL: f32 = -300.;
pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const TOP_WALL: f32 = 300.;
pub const WALL_THICKNESS: f32 = 10.0;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

pub fn spawn(commands: &mut Commands) {
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
}
