use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{player::Player, Game};

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub usize);

#[derive(Component)]
struct HealthDisplay;

pub struct HealhPlugin;

impl Plugin for HealhPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_health_display)
            .add_systems(
                Update,
                update_health_display.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_health_display(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        HealthDisplay,
        Game,
    ));
}

fn update_health_display(
    mut commands: Commands,
    health_query: Query<&Health, (Changed<Health>, With<Player>)>,
    health_display_query: Query<Entity, With<HealthDisplay>>,
    assets: Res<GameAssets>,
) {
    let Ok(health) = health_query.get_single() else {
        return; // health didn't change, don't bother updating
    };

    let health_display = health_display_query.single();

    commands
        .entity(health_display)
        .despawn_descendants()
        .with_children(|parent| {
            for _ in 0..health.0 {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        margin: UiRect::left(Val::Px(10.0)),
                        ..Default::default()
                    },
                    image: UiImage::new(assets.heart.clone()),
                    ..Default::default()
                });
            }
        });
}
