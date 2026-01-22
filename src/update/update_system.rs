use crate::update::edge_boy::move_edge_boy::move_edge_boy;

pub fn update_system() -> fn(
    bevy_ecs::system::Res<'_, bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>,
    bevy_ecs::system::Single<
        '_,
        '_,
        &mut bevy::transform::components::Transform,
        bevy_ecs::query::With<crate::entity::edge_boy::EdgeBoy>,
    >,
    bevy_ecs::system::Res<'_, bevy::time::Time>,
) {
    (move_edge_boy)
}
