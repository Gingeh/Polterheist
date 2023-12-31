use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, window::WindowResolution};

use bevy_asset_loader::prelude::*;
use bevy_nine_slice_ui::NineSliceUiPlugin;

mod game;
mod gameover;
mod menu;
mod splash;
mod utils;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Playing,
    GameOver,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "bevy.png")]
    bevy_logo: Handle<Image>,
    #[asset(path = "background.png")]
    background: Handle<Image>,
    #[asset(path = "Overpass-SemiBold.ttf")]
    font: Handle<Font>,
    #[asset(path = "player.png")]
    player: Handle<Image>,
    #[asset(path = "wand.png")]
    wand: Handle<Image>,
    #[asset(path = "basic-enemy.png")]
    basic_enemy: Handle<Image>,
    #[asset(path = "ranged-enemy.png")]
    ranged_enemy: Handle<Image>,
    #[asset(path = "punch-spark.png")]
    punch_spark: Handle<Image>,
    #[asset(path = "basic-spark.png")]
    basic_spark: Handle<Image>,
    #[asset(path = "ranged-spark.png")]
    ranged_spark: Handle<Image>,
    #[asset(path = "next-spark-ring.png")]
    next_spark_ring: Handle<Image>,
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
    #[asset(path = "heart.png")]
    heart: Handle<Image>,
    #[asset(path = "pointer.png")]
    pointer: Handle<Image>,
    #[asset(path = "shoot.ogg")]
    shoot: Handle<AudioSource>,
    #[asset(path = "hit_enemy.ogg")]
    hit_enemy: Handle<AudioSource>,
    #[asset(path = "hurt.ogg")]
    hurt: Handle<AudioSource>,
    #[asset(path = "broken-staff.png")]
    broken_staff: Handle<Image>,
    #[asset(path = "puff.png")]
    puff: Handle<Image>,
    #[asset(path = "button-ninepatch.png")]
    button_ninepatch: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.298,
            green: 0.271,
            blue: 0.247,
            alpha: 1.0,
        }))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game".to_string(),
                resolution: WindowResolution::new(800.0, 800.0),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(NineSliceUiPlugin::default())
        .add_state::<GameState>()
        .init_collection::<GameAssets>()
        .add_plugins((
            splash::SplashPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            gameover::GameOverPlugin,
        ))
        .add_systems(Startup, setup);

    #[cfg(feature = "inspect")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assets.background.clone(),
        transform: Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: -100.0,
        }),
        ..Default::default()
    });
}
