use asset::plugin::assets_pluin::AssetsPlugin;
use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;

use piece::{
    evolution::plugin::PieceEvolvePlugin, next::plugin::NextPiecePlugin, plugin::PiecePlugin,
};

use title::plugin::TitlePlugin;

use crate::{
    field::game::plugin::GameFieldPlugin,
    game::plugin::GamePlugin,
    game_over::plugin::GameOverPlugin,
    score::{high_score::plugin::HighScorePlugin, plugin::ScorePlugin},
};

mod asset;
mod consts;
mod field;
mod file;
mod game;
mod game_over;
mod parameter;
mod piece;

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
        .add_plugins(GameFieldPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScorePlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(HighScorePlugin)
        .add_plugins(NextPiecePlugin)
        .add_plugins(PieceEvolvePlugin)
        .add_plugins(PiecePlugin)
        .add_plugins(TitlePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
