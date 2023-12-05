use bevy::prelude::*;

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
        .add_systems(OnEnter(GameState::Playing), spawn_test_enemies);
    }
}

fn spawn_test_enemies(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(EnemyBundle::new(EnemyKind::Basic, &assets))
        .insert(Transform::from_xyz(100.0, 200.0, 0.0));

    commands
        .spawn(EnemyBundle::new(EnemyKind::Basic, &assets))
        .insert(Transform::from_xyz(-100.0, 200.0, 0.0));
}
