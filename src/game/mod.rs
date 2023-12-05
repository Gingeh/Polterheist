use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::{utils, GameAssets, GameState};

use self::enemy::{EnemyBundle, EnemyKind};

mod enemy;
mod player;
mod projectile;
mod spark;

#[derive(Component, Deref, DerefMut)]
pub struct Health(usize);

#[derive(Component)]
struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            spark::SparkPlugin,
            enemy::EnemyPlugin,
            projectile::ProjectilePlugin,
        ))
        .add_systems(OnExit(GameState::Playing), utils::despawn_with::<Game>)
        .add_systems(
            Update,
            spawn_enemy
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_secs(1))),
        );
    }
}

fn spawn_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(EnemyBundle::new(EnemyKind::Basic, &assets))
        .insert(Transform::from_xyz(450.0, 450.0, 0.0));
}
