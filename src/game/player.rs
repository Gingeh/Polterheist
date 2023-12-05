use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{enemy::EnemyKind, spark::SparkCallbacks, Game, projectile::Team};

#[derive(Component)]
pub struct Player {
    move_speed: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct Sparks(VecDeque<EnemyKind>);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    game: Game,
    #[bundle()]
    sprite: SpriteBundle,
    team: Team,
    sparks: Sparks,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (move_player, turn_player, handle_use).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(PlayerBundle {
        player: Player { move_speed: 400.0 },
        game: Game,
        sprite: SpriteBundle {
            texture: assets.player.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 40.0, y: 40.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        team: Team::Friendly,
        sparks: Sparks(VecDeque::new()),
    });
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;
    if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.y -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.x += 1.0;
    }

    if movement != Vec3::ZERO {
        let (mut transform, player) = player_query.single_mut();
        transform.translation += movement.normalize() * player.move_speed * time.delta_seconds();
    }
}

fn turn_player(
    window_query: Query<&Window>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_pos) = window_query
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };
    let mut transform = player_query.single_mut();
    let direction = (cursor_pos - transform.translation.truncate()).normalize();
    transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);
}

fn handle_use(
    mut commands: Commands,
    mut sparks_query: Query<&mut Sparks>,
    spark_callbacks: Res<SparkCallbacks>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut sparks = sparks_query.single_mut();
        let callback = spark_callbacks(sparks.pop_front());
        commands.run_system(callback);
    }
}
