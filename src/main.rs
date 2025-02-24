use asset::plugin::{assets_pluin::AssetsPlugin, piece_sound_plugin::PieceSoundPlugin};
use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;
use game::plugin::game_plugin::GamePlugin;
use game_over::plugin::game_over_plugin::GameOverPlugin;
use high_score::plugin::high_score_plugin::HighScorePlugin;
use piece::plugin::piece_plugin::PiecePlugin;
use resource::grab_postion::GrabPostion;
use score::plugin::score_plugin::ScorePlugin;
use title::plugin::title_menu_plugin::TitlePlugin;

mod asset;
mod consts;
mod file;
mod game;
mod game_over;
mod high_score;
mod piece;
mod resource;
mod score;
mod title;
mod ui;

fn main() {
    let window = Window {
        title: "puzzle".to_string(),
        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
        resizable: false,
        ..default()
    };
    let primary_window = Some(window);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        }))
        .add_plugins(AssetsPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScorePlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(HighScorePlugin)
        .add_plugins(PieceSoundPlugin)
        .add_plugins(PiecePlugin)
        .add_plugins(TitlePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GrabPostion { x: 0.0 })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
