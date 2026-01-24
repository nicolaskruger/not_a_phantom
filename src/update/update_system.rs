use bevy::app::{App, FixedUpdate};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::update::edge_boy::move_edge_boy::move_edge_boy;

pub fn update_system(mut app: App) -> App {
    app.add_systems(FixedUpdate, (move_edge_boy).chain());
    app
}
