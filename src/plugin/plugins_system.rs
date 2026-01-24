use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Update},
    ui::percent,
};

use crate::stepping;

pub fn plugin_system(mut app: App) -> App {
    app.add_plugins(DefaultPlugins).add_plugins(
        stepping::SteppingPlugin::default()
            .add_schedule(Update)
            .add_schedule(FixedUpdate)
            .at(percent(35), percent(50)),
    );
    app
}
