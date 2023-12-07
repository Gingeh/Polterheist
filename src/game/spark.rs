use std::f32::consts::TAU;

use bevy::ecs::system::SystemId;
use bevy::prelude::*;

use crate::GameAssets;

use super::{
    enemy::{Enemy, EnemyKind},
    health::Health,
    player::Player,
    projectile::{Projectile, ProjectileBundle, Radius, Team, Velocity},
    Game,
};

pub struct SparkPlugin;

impl Plugin for SparkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SparkCallbacks>();
    }
}

#[derive(Resource, Deref)]
pub struct SparkCallbacks(Box<dyn Fn(Option<EnemyKind>) -> SystemId + Send + Sync>);

impl FromWorld for SparkCallbacks {
    fn from_world(world: &mut World) -> Self {
        let punch = world.register_system(handle_punch);
        let basic = world.register_system(handle_basic);
        let ranged = world.register_system(handle_ranged);

        Self(Box::new(move |kind| match kind {
            None => punch,
            Some(EnemyKind::Basic) => basic,
            Some(EnemyKind::Ranged) => ranged,
        }))
    }
}

//TODO: Make CAST_DIST a field on Player
fn handle_punch(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &Radius, &mut Health), With<Enemy>>,
) {
    const CAST_DIST: f32 = 80.0;

    let player = player_query.single();

    for (enemy, radius, mut health) in &mut enemy_query {
        let player_to_enemy = enemy.translation - player.translation;
        let closest_point = player_to_enemy.dot(player.local_y());
        if ((0.0..=CAST_DIST).contains(&closest_point)
            && (player.local_y() * closest_point).distance_squared(player_to_enemy)
                <= radius.powi(2))
            || (player.local_y() * CAST_DIST).distance_squared(player_to_enemy) <= radius.powi(2)
            || player_to_enemy.length_squared() <= radius.powi(2)
        {
            **health -= 1;
        }
    }
}

fn handle_basic(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    assets: Res<GameAssets>,
) {
    let transform = *player_query.single();
    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(800.0),
        team: Team::Friendly,
        radius: Radius(10.0),
    });
}

fn handle_ranged(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    assets: Res<GameAssets>,
) {
    let mut transform = *player_query.single();
    let angle = TAU / 32.0;

    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(800.0),
        team: Team::Friendly,
        radius: Radius(10.0),
    });

    transform.rotate_local_z(-angle / 2.0);
    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(800.0),
        team: Team::Friendly,
        radius: Radius(10.0),
    });

    transform.rotate_local_z(angle);
    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(800.0),
        team: Team::Friendly,
        radius: Radius(10.0),
    });
}
