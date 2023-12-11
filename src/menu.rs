use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{utils, GameAssets, GameState};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), utils::despawn_with::<MainMenu>);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>) {
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
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..Default::default()
                    },
                    MenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", text_style.clone()));
                });
            #[cfg(not(target_family = "wasm"))]
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..Default::default()
                    },
                    MenuButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Quit", text_style.clone()));
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButton::Play => app_state.set(GameState::Playing),
                MenuButton::Quit => app_exit_writer.send(AppExit),
            }
        }
    }
}
