use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{
    enemy::EnemyKind,
    health::Health,
    projectile::{Radius, Team},
    spark::SparkCallbacks,
    Game,
};

#[derive(Component)]
pub struct Player {
    move_speed: f32,
}

#[derive(Component)]
struct Wand;

#[derive(Component, Deref, DerefMut)]
pub struct Sparks(pub VecDeque<EnemyKind>);

#[derive(Component, Deref, DerefMut)]
pub struct PunchCooldown(Timer);

#[derive(Component, Deref, DerefMut)]
struct HurtCooldown(Timer);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    game: Game,
    #[bundle()]
    sprite: SpriteBundle,
    team: Team,
    radius: Radius,
    sparks: Sparks,
    health: Health,
    punch_cooldown: PunchCooldown,
    hurt_cooldown: HurtCooldown,
}

#[derive(Event)]
pub struct HurtPlayerEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HurtPlayerEvent>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    move_player,
                    turn_player,
                    handle_use,
                    handle_hurt_events,
                    show_wand,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(PlayerBundle {
            player: Player { move_speed: 300.0 },
            game: Game,
            sprite: SpriteBundle {
                texture: assets.player.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 38.0, y: 38.0 }),
                    ..Default::default()
                },
                ..Default::default()
            },
            team: Team::Friendly,
            radius: Radius(19.0),
            sparks: Sparks(VecDeque::new()),
            health: Health(3),
            punch_cooldown: PunchCooldown(Timer::from_seconds(0.5, TimerMode::Once)),
            hurt_cooldown: HurtCooldown(Timer::from_seconds(0.5, TimerMode::Once)),
        })
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: assets.wand.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 16.0, y: 39.0 }),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 29.0, 1.0),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                Wand,
            ));
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
    mut player_query: Query<(&mut Sparks, &mut PunchCooldown)>,
    spark_callbacks: Res<SparkCallbacks>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let (mut sparks, mut timer) = player_query.single_mut();
    timer.tick(time.delta());

    if mouse.just_pressed(MouseButton::Left) {
        let callback = spark_callbacks(sparks.pop_front());
        commands.run_system(callback);
    }
}

fn handle_hurt_events(
    mut commands: Commands,
    mut hurt_events: EventReader<HurtPlayerEvent>,
    mut player_query: Query<(&mut Health, &mut HurtCooldown)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    assets: Res<GameAssets>,
) {
    let (mut health, mut cooldown) = player_query.single_mut();

    if cooldown.tick(time.delta()).finished() && !hurt_events.is_empty() {
        hurt_events.clear();
        **health -= 1;
        cooldown.reset();
        commands.spawn(AudioBundle {
            source: assets.hurt.clone(),
            ..Default::default()
        });
    }

    if **health == 0 {
        *next_state = NextState(Some(GameState::GameOver));
    }
}

fn show_wand(mut wand_query: Query<&mut Visibility, With<Wand>>, mouse: Res<Input<MouseButton>>) {
    let mut visibility = wand_query.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        *visibility = Visibility::Visible;
    } else if mouse.just_released(MouseButton::Left) {
        *visibility = Visibility::Hidden;
    }
}
