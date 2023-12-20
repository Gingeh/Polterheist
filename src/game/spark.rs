use std::f32::consts::TAU;

use bevy::ecs::system::SystemId;
use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{
    enemy::{Enemy, EnemyKind},
    health::Health,
    player::{Player, PunchCooldown, Sparks},
    projectile::{Projectile, ProjectileBundle, Radius, Team, Velocity},
    Game,
};

#[derive(Component)]
struct SparkDisplay;

pub struct SparkPlugin;

impl Plugin for SparkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SparkCallbacks>()
            .add_systems(OnEnter(GameState::Playing), spawn_spark_display)
            .add_systems(
                Update,
                update_spark_display.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_spark_display(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        },
        SparkDisplay,
        Game,
    ));

    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            image: UiImage::new(assets.next_spark_ring.clone()),
            z_index: ZIndex::Global(99),
            ..Default::default()
        },
        Game,
    ));
}

fn update_spark_display(
    mut commands: Commands,
    sparks_query: Query<&Sparks, (Changed<Sparks>, With<Player>)>,
    spark_display_query: Query<Entity, With<SparkDisplay>>,
    assets: Res<GameAssets>,
) {
    let Ok(sparks) = sparks_query.get_single() else {
        return; // health didn't change, don't bother updating
    };

    let spark_display = spark_display_query.single();

    commands
        .entity(spark_display)
        .despawn_descendants()
        .with_children(|parent| {
            if sparks.0.is_empty() {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(80.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        ..Default::default()
                    },
                    image: UiImage::new(assets.punch_spark.clone()),
                    ..Default::default()
                });
            } else {
                for spark in sparks.0.iter() {
                    let sprite = match spark {
                        EnemyKind::Basic => assets.basic_spark.clone(),
                        EnemyKind::Ranged => assets.ranged_spark.clone(),
                    };
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::top(Val::Px(20.0)),
                            ..Default::default()
                        },
                        image: UiImage::new(sprite),
                        ..Default::default()
                    });
                }
            }
        });
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

fn handle_punch(
    mut player_query: Query<(&Transform, &mut PunchCooldown, &Player)>,
    mut enemy_query: Query<(&Transform, &Radius, &mut Health), With<Enemy>>,
) {
    let (player, mut timer, player_settings) = player_query.single_mut();
    let cast_dist = player_settings.punch_distance;

    if !timer.finished() {
        return;
    }

    for (enemy, radius, mut health) in &mut enemy_query {
        let player_to_enemy = enemy.translation - player.translation;
        let closest_point = player_to_enemy.dot(player.local_y());
        if ((0.0..=cast_dist).contains(&closest_point)
            && (player.local_y() * closest_point).distance_squared(player_to_enemy)
                <= radius.powi(2))
            || (player.local_y() * cast_dist).distance_squared(player_to_enemy) <= radius.powi(2)
            || player_to_enemy.length_squared() <= radius.powi(2)
        {
            **health -= 1;
            timer.reset();
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
                custom_size: Some(Vec2 { x: 19.0, y: 19.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(600.0),
        team: Team::Friendly,
        radius: Radius(9.5),
    });
    commands.spawn(AudioBundle {
        source: assets.shoot.clone(),
        ..Default::default()
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
                custom_size: Some(Vec2 { x: 19.0, y: 19.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(600.0),
        team: Team::Friendly,
        radius: Radius(9.5),
    });

    transform.rotate_local_z(-angle / 2.0);
    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 19.0, y: 19.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(600.0),
        team: Team::Friendly,
        radius: Radius(9.5),
    });

    transform.rotate_local_z(angle);
    commands.spawn(ProjectileBundle {
        projectile: Projectile,
        game: Game,
        sprite: SpriteBundle {
            texture: assets.bullet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 19.0, y: 19.0 }),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
        velocity: Velocity(600.0),
        team: Team::Friendly,
        radius: Radius(9.5),
    });

    commands.spawn(AudioBundle {
        source: assets.shoot.clone(),
        ..Default::default()
    });
}
