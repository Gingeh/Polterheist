use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{
    player::{Player, Sparks},
    projectile::{Radius, Team},
    Game, Health,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone, Copy)]
pub enum EnemyKind {
    Basic,
}

#[derive(Component)]
struct Behaviour {
    homing_force: f32,
    separating_force: f32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    game: Game,
    kind: EnemyKind,
    #[bundle()]
    sprite: SpriteBundle,
    team: Team,
    health: Health,
    radius: Radius,
    behaviour: Behaviour,
}

impl EnemyBundle {
    pub fn new(kind: EnemyKind, assets: &GameAssets) -> Self {
        match kind {
            EnemyKind::Basic => Self {
                enemy: Enemy,
                game: Game,
                kind,
                sprite: SpriteBundle {
                    texture: assets.basic_enemy.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 40.0, y: 40.0 }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                team: Team::Hostile,
                health: Health(1),
                radius: Radius(20.0),
                behaviour: Behaviour {
                    homing_force: 100.0,
                    separating_force: 100.0,
                },
            },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_health, fetch_positions.pipe(movement)).run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_health(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Health, &EnemyKind), Changed<Health>>,
    mut player_query: Query<&mut Sparks>,
) {
    let mut sparks = player_query.single_mut();
    for (entity, health, kind) in &enemy_query {
        if **health != 0 {
            continue;
        }
        commands.entity(entity).despawn_recursive();
        sparks.push_back(*kind);
    }
}

fn fetch_positions(enemy_query: Query<&Transform, With<Enemy>>) -> Vec<Vec3> {
    enemy_query
        .iter()
        .map(|transform| transform.translation)
        .collect()
}

fn movement(
    In(other_positions): In<Vec<Vec3>>,
    mut enemy_query: Query<(&mut Transform, &Behaviour), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_position = player_query.single().translation;
    for (mut transform, behaviour) in &mut enemy_query {
        let homing = (player_position - transform.translation).normalize();
        let separation = other_positions
            .iter()
            .filter_map(|&pos| {
                if pos == transform.translation {
                    return None;
                }
                let away = transform.translation - pos;
                let distance = away.length_squared();
                Some(away.normalize() * distance.recip())
            })
            .sum::<Vec3>()
            .clamp_length_max(behaviour.separating_force.recip());

        transform.translation += (homing * behaviour.homing_force
            + separation * behaviour.separating_force.powi(2))
            * time.delta_seconds();
    }
}
