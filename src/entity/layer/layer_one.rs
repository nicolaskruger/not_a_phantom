use bevy_ecs::{
    component::Component,
    entity::{ContainsEntity, Entity},
    query::With,
    system::{Commands, Single},
};

use crate::entity::{death::Death, edge_boy::EdgeBoy, game_state::Game, layer::Layer};

use crate::entity::{edge_boy::spawn as edge_boy_spaw, game_state::GameState};

pub struct FirstLayerImpl {}

impl Layer for FirstLayerImpl {
    fn dispose(
        &self,
        edge_boy: &Single<Entity, With<EdgeBoy>>,
        death: &Single<Entity, With<Death>>,
        commands: &mut Commands,
        _: &mut Game,
    ) {
        commands.entity(edge_boy.entity()).despawn();
        commands.entity(death.entity()).despawn();
    }

    fn reset(
        &self,
        edge_boy: Single<Entity, With<EdgeBoy>>,
        death: Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    ) {
        self.dispose(&edge_boy, &death, commands, game);
        self.load(edge_boy, death, commands, game);
    }

    fn load(
        &self,
        _: Single<Entity, With<EdgeBoy>>,
        _: Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    ) {
        edge_boy_spaw(commands);
        game.state = GameState::Playing;
    }
}

#[derive(Component)]
pub struct FirstLayer {
    layer: Box<dyn Layer>,
}

impl FirstLayer {
    pub fn new(layer: Box<dyn Layer>) -> Self {
        Self { layer }
    }
}

impl Layer for FirstLayer {
    fn dispose(
        &self,
        edge_boy: &Single<Entity, With<EdgeBoy>>,
        death: &Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    ) {
        self.layer.dispose(edge_boy, death, commands, game);
    }
    fn reset(
        &self,
        edge_boy: Single<Entity, With<EdgeBoy>>,
        death: Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    ) {
        self.layer.reset(edge_boy, death, commands, game);
    }
    fn load(
        &self,
        edge_boy: Single<Entity, With<EdgeBoy>>,
        death: Single<Entity, With<Death>>,
        commands: &mut Commands,
        game: &mut Game,
    ) {
        self.layer.load(edge_boy, death, commands, game);
    }
}
