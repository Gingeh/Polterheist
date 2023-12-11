use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{player::Player, Game};

#[derive(Component)]
struct Pointer;

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_pointer)
            .add_systems(Update, move_pointer.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pointer(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.pointer.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        Pointer,
        Game,
    ));
}

fn move_pointer(
    mut pointer_query: Query<(&mut Transform, &mut Visibility), With<Pointer>>,
    player_query: Query<&Transform, (With<Player>, Without<Pointer>)>,
) {
    let player = player_query.single();
    let (mut pointer_transform, mut pointer_visibility) = pointer_query.single_mut();

    if player.translation.abs().max_element() >= 380.0 {
        *pointer_visibility = Visibility::Visible;

        pointer_transform.translation = player
            .translation
            .clamp(Vec3::splat(-380.0), Vec3::splat(380.0));

        pointer_transform.rotation =
            Quat::from_rotation_arc(Vec3::Y, player.translation.normalize());
    } else {
        *pointer_visibility = Visibility::Hidden;
    }
}
