use bevy_ecs::system::Commands;

use crate::entity::layer::layer_one::{FirstLayer, FirstLayerImpl};

pub mod layer_one;

pub trait Layer: Send + Sync {
    fn dispose(&self);
    fn reset(&self);
    fn load(&self);
}

pub fn spawn(commands: &mut Commands) {
    let first_layer: Box<dyn Layer> = Box::new(FirstLayerImpl {});
    let first_layer = FirstLayer { layer: first_layer };
    commands.spawn(first_layer);
}
