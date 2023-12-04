use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::Game;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
pub enum EnemyKind {
    Basic,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    game: Game,
    #[bundle()]
    sprite: SpriteBundle,
    kind: EnemyKind,
}

impl EnemyBundle {
    pub fn new(kind: EnemyKind, assets: &GameAssets) -> Self {
        let sprite = match kind {
            EnemyKind::Basic => SpriteBundle {
                texture: assets.basic_enemy.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 40.0, y: 40.0 }),
                    ..Default::default()
                },
                ..Default::default()
            },
        };

        Self {
            enemy: Enemy,
            game: Game,
            sprite,
            kind,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_test_enemy);
    }
}

fn spawn_test_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(EnemyBundle::new(EnemyKind::Basic, &assets))
        .insert(Transform::from_xyz(0.0, 200.0, 0.0));
}
