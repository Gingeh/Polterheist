use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{player::Sparks, Game, Health};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone, Copy)]
pub enum EnemyKind {
    Basic,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    game: Game,
    kind: EnemyKind,
    #[bundle()]
    sprite: SpriteBundle,
    health: Health,
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
                health: Health(1),
            },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_test_enemy)
            .add_systems(Update, handle_health.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_test_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(EnemyBundle::new(EnemyKind::Basic, &assets))
        .insert(Transform::from_xyz(0.0, 200.0, 0.0));
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
