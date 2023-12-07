use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_asset_loader::prelude::*;

mod game;
mod menu;
mod splash;
mod utils;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "bevy.png")]
    bevy_logo: Handle<Image>,
    #[asset(path = "Overpass-SemiBold.ttf")]
    font: Handle<Font>,
    #[asset(path = "player.png")]
    player: Handle<Image>,
    #[asset(path = "basic-enemy.png")]
    basic_enemy: Handle<Image>,
    #[asset(path = "ranged-enemy.png")]
    ranged_enemy: Handle<Image>,
    #[asset(path = "basic-spark.png")]
    basic_spark: Handle<Image>,
    #[asset(path = "ranged-spark.png")]
    ranged_spark: Handle<Image>,
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
    #[asset(path = "heart.png")]
    heart: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Game".to_string(),
            resolution: WindowResolution::new(800.0, 800.0),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_state::<GameState>()
    .init_collection::<GameAssets>()
    .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
    .add_systems(Startup, setup);

    #[cfg(feature = "inspect")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
