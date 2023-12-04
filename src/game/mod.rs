use bevy::prelude::*;

use crate::{utils, GameState};

mod enemy;
mod player;

#[derive(Component)]
struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, enemy::EnemyPlugin))
            .add_systems(OnExit(GameState::Playing), utils::despawn_with::<Game>);
    }
}
