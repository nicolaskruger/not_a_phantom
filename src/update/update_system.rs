use bevy::app::{App, FixedUpdate};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::update::{
    death::show_death::show_death, edge_boy::move_edge_boy::move_edge_boy,
    norm::collide_norm::collide_norm,
};

pub fn update_system(mut app: App) -> App {
    app.add_systems(
        FixedUpdate,
        (move_edge_boy, collide_norm, show_death).chain(),
    );
    app
}
