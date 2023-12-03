use bevy::prelude::*;

use crate::{utils, GameState};

mod player;

#[derive(Component)]
struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_systems(OnExit(GameState::Playing), utils::despawn_with::<Game>);
    }
}
