use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default, Component)]
pub enum GameState {
    #[default]
    Playing,
    ItsOver,
}

#[derive(Component)]
pub struct Game {
    pub state: GameState,
}

pub fn spawn(commands: &mut Commands) {
    let game = Game {
        state: GameState::Playing,
    };
    commands.spawn(game);
}
