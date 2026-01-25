use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameStateEnum {
    #[default]
    Playing,
    ItsOver,
}

#[derive(Component)]
pub struct GameState(GameStateEnum);

pub fn spawn(commands: &mut Commands) {
    commands.spawn((GameState(GameStateEnum::Playing)));
}
