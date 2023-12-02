use bevy::prelude::*;

use crate::{utils, GameAssets, GameState};

#[derive(Component)]
struct SplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup_splash)
            .add_systems(
                Update,
                countdown_splash_timer.run_if(in_state(GameState::Splash)),
            )
            .add_systems(
                OnExit(GameState::Splash),
                utils::despawn_with::<SplashScreen>,
            );
    }
}

fn setup_splash(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            SplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(assets.bevy_logo.clone()),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "Made with Bevy",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown_splash_timer(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
