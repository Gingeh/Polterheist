use bevy::ecs::system::SystemId;
use bevy::prelude::*;

use super::{
    enemy::{Enemy, EnemyKind},
    player::Player,
    Health,
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
        let basic = world.register_system(|| todo!());
        let punch = world.register_system(handle_punch);

        Self(Box::new(move |kind| match kind {
            Some(EnemyKind::Basic) => basic,
            None => punch,
        }))
    }
}

//TODO: Make CAST_DIST a field on Player
//TODO: and get RADIUS from the enemy somehow
fn handle_punch(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
) {
    const CAST_DIST: f32 = 40.0;
    const RADIUS: f32 = 20.0;

    let player = player_query.single();

    for (enemy, mut health) in &mut enemy_query {
        let player_to_enemy = enemy.translation - player.translation;
        let closest_point = player_to_enemy.dot(player.local_y());
        if ((0.0..=CAST_DIST).contains(&closest_point)
            && (player.local_y() * closest_point).distance_squared(player_to_enemy)
                <= RADIUS.powi(2))
            || (player.local_y() * CAST_DIST).distance_squared(player_to_enemy) <= RADIUS.powi(2)
            || player_to_enemy.length_squared() <= RADIUS.powi(2)
        {
            **health -= 1;
        }
    }
}
