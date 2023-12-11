use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::{utils, GameAssets, GameState};

use self::{
    enemy::{EnemyBundle, EnemyKind},
    score::Score,
};

mod enemy;
mod health;
mod player;
mod pointer;
mod projectile;
pub mod score;
mod spark;

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
            score::ScorePlugin,
            health::HealhPlugin,
            pointer::PointerPlugin,
        ))
        .add_systems(OnExit(GameState::Playing), utils::despawn_with::<Game>)
        .add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    score: Res<Score>,
) {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, 1.0);

    for (kind, per_second) in [
        (EnemyKind::Basic, 1.0),
        (EnemyKind::Ranged, 0.01 * score.score as f32),
    ] {
        if uniform.sample(&mut rng) <= per_second * time.delta_seconds() {
            let position =
                Quat::from_rotation_z(uniform.sample(&mut rng) * TAU).mul_vec3(Vec3::Y * 600.0);

            commands
                .spawn(EnemyBundle::new(kind, &assets))
                .insert(Transform::from_translation(position));
        }
    }
}
