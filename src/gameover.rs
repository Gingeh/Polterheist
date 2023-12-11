use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{game::score::Score, utils, GameAssets, GameState};

#[derive(Component)]
struct GameOver;

#[derive(Component)]
enum GameOverButton {
    Retry,
    Quit,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), utils::despawn_with::<GameOver>);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, score: Res<Score>) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 40.0,
        color: Color::BLACK,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            GameOver,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections(vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: score.score.to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
            ]));
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Auto,
                    ..Default::default()
                },
                image: UiImage::new(assets.broken_staff.clone()),
                ..Default::default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..Default::default()
                            },
                            GameOverButton::Retry,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Retry", text_style.clone()));
                        });
                    #[cfg(not(target_family = "wasm"))]
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..Default::default()
                            },
                            GameOverButton::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", text_style.clone()));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &GameOverButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverButton::Retry => app_state.set(GameState::Playing),
                GameOverButton::Quit => app_exit_writer.send(AppExit),
            }
        }
    }
}
