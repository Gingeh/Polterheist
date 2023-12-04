use bevy::prelude::*;

use crate::{utils, GameState};

mod enemy;
mod player;
mod spark;

#[derive(Component, Deref, DerefMut)]
pub struct Health(usize);

#[derive(Component)]
struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, spark::SparkPlugin, enemy::EnemyPlugin))
            .add_systems(OnExit(GameState::Playing), utils::despawn_with::<Game>);
    }
}
