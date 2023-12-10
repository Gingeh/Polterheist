use bevy::ecs::system::{Command, RunSystemOnce};
use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{
    health::Health,
    player::{HurtPlayerEvent, Player, Sparks},
    projectile::{Projectile, ProjectileBundle, Radius, Team, Velocity},
    Game,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone, Copy)]
pub enum EnemyKind {
    Basic,
    Ranged,
}

#[derive(Component)]
struct Behaviour {
    homing_force: f32,
    separating_force: f32,
}

#[derive(Component, Deref, DerefMut)]
struct AttackTimer(Timer);

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
    attack_timer: AttackTimer,
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
                        custom_size: Some(Vec2 { x: 30.0, y: 30.0 }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                team: Team::Hostile,
                health: Health(1),
                radius: Radius(15.0),
                behaviour: Behaviour {
                    homing_force: 75.0,
                    separating_force: 75.0,
                },
                attack_timer: AttackTimer(Timer::from_seconds(0.0, TimerMode::Repeating)),
            },
            EnemyKind::Ranged => Self {
                enemy: Enemy,
                game: Game,
                kind,
                sprite: SpriteBundle {
                    texture: assets.ranged_enemy.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 40.0, y: 40.0 }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                team: Team::Hostile,
                health: Health(2),
                radius: Radius(20.0),
                behaviour: Behaviour {
                    homing_force: 60.0,
                    separating_force: 150.0,
                },
                attack_timer: AttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_health,
                fetch_positions.pipe(movement),
                handle_attacks,
            )
                .run_if(in_state(GameState::Playing)),
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

fn handle_attacks(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut AttackTimer, &EnemyKind)>,
    time: Res<Time>,
) {
    for (enemy, mut timer, &kind) in &mut enemy_query {
        if timer.tick(time.delta()).finished() {
            commands.add(AttackCommand { enemy, kind });
        }
    }
}

struct AttackCommand {
    enemy: Entity,
    kind: EnemyKind,
}

impl Command for AttackCommand {
    fn apply(self, world: &mut World) {
        match self.kind {
            EnemyKind::Basic => world.run_system_once_with(self.enemy, handle_basic_attack),
            EnemyKind::Ranged => world.run_system_once_with(self.enemy, handle_ranged_attack),
        }
    }
}

fn handle_basic_attack(
    In(enemy): In<Entity>,
    enemy_query: Query<(&Transform, &Radius), With<Enemy>>,
    player_query: Query<(&Transform, &Radius), With<Player>>,
    mut hurt_event_writer: EventWriter<HurtPlayerEvent>,
) {
    let Ok((enemy_transform, enemy_radius)) = enemy_query.get(enemy) else {
        return; // Enemy died before command was executed
    };
    let (player_transform, player_radius) = player_query.single();

    if enemy_transform
        .translation
        .distance_squared(player_transform.translation)
        <= (enemy_radius.0 + player_radius.0).powi(2)
    {
        hurt_event_writer.send(HurtPlayerEvent);
    }
}

fn handle_ranged_attack(
    In(enemy): In<Entity>,
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    assets: Res<GameAssets>,
) {
    let Ok(enemy_transform) = enemy_query.get(enemy) else {
        return; // Enemy died before command was executed
    };
    let player_transform = player_query.single();

    let mut projectile_transform = *enemy_transform;
    let direction = (player_transform.translation - enemy_transform.translation)
        .truncate()
        .normalize();
    projectile_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);

    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            transform: projectile_transform,
            ..Default::default()
        },
        velocity: Velocity(150.0),
        team: Team::Hostile,
        radius: Radius(10.0),
    });
}
