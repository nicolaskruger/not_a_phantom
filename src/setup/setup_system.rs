use bevy::app::{App, Startup};

use crate::setup::setup;

pub fn setup_system(mut app: App) -> App {
    app.add_systems(Startup, setup::setup);
    app
}
