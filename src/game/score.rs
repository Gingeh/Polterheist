use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{enemy::Enemy, health::Health};

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct HighScoreDisplay;

pub struct ScorePlugin;

#[derive(Resource, Default)]
pub struct Score {
    pub score: usize,
    pub high_score: usize,
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnExit(GameState::Splash), spawn_scoreboard)
            .add_systems(OnEnter(GameState::Playing), (reset_score, show_scoreboard))
            .add_systems(OnExit(GameState::Playing), hide_scoreboard)
            .add_systems(
                Update,
                (update_score, update_scoreboard).run_if(in_state(GameState::Playing)),
            );
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.score = 0;
}

fn update_score(
    mut commands: Commands,
    mut score: ResMut<Score>,
    enemy_query: Query<Ref<Health>, (Changed<Health>, With<Enemy>)>,
    assets: Res<GameAssets>,
) {
    let hurt_enemies = enemy_query.iter().filter(|x| !x.is_added()).count();

    score.score += hurt_enemies;
    if score.score > score.high_score {
        score.high_score = score.score;
    }

    if hurt_enemies != 0 {
        commands.spawn(AudioBundle {
            source: assets.hit_enemy.clone(),
            ..Default::default()
        });
    }
}

fn spawn_scoreboard(mut commands: Commands, assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections(vec![
                    TextSection {
                        value: "High Score: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: text_style.clone(),
                    },
                ]),
                HighScoreDisplay,
            ));

            parent.spawn((
                TextBundle::from_sections(vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: text_style.clone(),
                    },
                ])
                .with_style(Style {
                    display: Display::None,
                    ..Default::default()
                }),
                ScoreDisplay,
            ));
        });
}

fn show_scoreboard(mut scoreboard_query: Query<&mut Style, With<ScoreDisplay>>) {
    let mut style = scoreboard_query.single_mut();
    style.display = Display::Flex;
}

fn hide_scoreboard(mut scoreboard_query: Query<&mut Style, With<ScoreDisplay>>) {
    let mut style = scoreboard_query.single_mut();
    style.display = Display::None;
}

fn update_scoreboard(
    score: Res<Score>,
    mut score_query: Query<&mut Text, (With<ScoreDisplay>, Without<HighScoreDisplay>)>,
    mut high_score_query: Query<&mut Text, (With<HighScoreDisplay>, Without<ScoreDisplay>)>,
) {
    if !score.is_changed() {
        return;
    }

    let mut score_text = score_query.single_mut();
    score_text.sections[1].value = score.score.to_string();

    let mut high_score_text = high_score_query.single_mut();
    high_score_text.sections[1].value = score.high_score.to_string();
}
