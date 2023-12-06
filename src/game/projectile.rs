use bevy::prelude::*;

use crate::GameState;

use super::{Game, Health, player::HurtPlayerEvent};

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Deref, Clone, Copy)]
pub struct Velocity(pub f32);

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Friendly,
    Hostile,
}

#[derive(Component, Deref)]
pub struct Radius(pub f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub game: Game,
    #[bundle()]
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub team: Team,
    pub radius: Radius,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_projectile, check_collisions).run_if(in_state(GameState::Playing)),
        );
    }
}

fn move_projectile(mut projectile_query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (&velocity, mut transform) in &mut projectile_query {
        let forward = transform.local_y();
        transform.translation += forward * *velocity * time.delta_seconds();
    }
}

fn check_collisions(
    mut commands: Commands,
    projectile_query: Query<(&Transform, &Radius, &Team, Entity), With<Projectile>>,
    mut target_query: Query<(&Transform, &Radius, &Team, &mut Health)>,
    mut hurt_event_writer: EventWriter<HurtPlayerEvent>,
) {
    for (projectile_transform, projectile_radius, projectile_team, projectile) in &projectile_query
    {
        for (target_transform, target_radius, target_team, mut target_health) in &mut target_query {
            if projectile_team == target_team {
                continue; // Skip if both are on the same team
            }
            if projectile_transform
                .translation
                .distance_squared(target_transform.translation)
                > (projectile_radius.0 + target_radius.0).powi(2)
            {
                continue; // Skip if too far apart
            }

            commands.entity(projectile).despawn_recursive();
            match target_team {
                Team::Friendly => hurt_event_writer.send(HurtPlayerEvent),
                Team::Hostile => **target_health = target_health.saturating_sub(1),
            }
        }
    }
}
