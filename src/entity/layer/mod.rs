use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Single},
};

use crate::entity::{
    death::Death,
    edge_boy::EdgeBoy,
    game_state::Game,
    layer::layer_one::{FirstLayer, FirstLayerImpl},
};

pub mod layer_one;

pub trait Layer: Send + Sync {
    fn dispose(
        &self,
        edge_boy: &Single<Entity, With<EdgeBoy>>,
        death: &Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    );
    fn reset(
        &self,
        edge_boy: &Single<Entity, With<EdgeBoy>>,
        death: &Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    );
    fn load(
        &self,
        edge_boy: &Single<Entity, With<EdgeBoy>>,
        death: &Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    );
}

pub fn spawn(commands: &mut Commands) {
    let first_layer: Box<dyn Layer> = Box::new(FirstLayerImpl {});
    let first_layer = FirstLayer::new(first_layer);
    commands.spawn(first_layer);
}
