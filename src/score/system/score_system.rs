use crate::{
    score::{component::score_text::ScoreText, resource::score::Score},
    ui::util::font::{FontName, FontResource, FontResourceTrait},
};
use ::bevy::prelude::*;

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                top: Val::Px(50.),

                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                TextBundle::from_sections([
                    TextSection::new(
                        "Score : ",
                        TextStyle {
                            font: FontResource::get_font_resouce(
                                &asset_server,
                                &FontName::HachiMaruPopReg,
                            ),
                            font_size: 50.,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "",
                        TextStyle {
                            font: asset_server.load("Roboto-Regular.ttf"),
                            font_size: 50.,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                ]),
            ));
        });
}

pub fn update_score(puzzle_score_res: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    // let mut text = query.single_mut();
    // text.sections[1].value = puzzle_score_res.0.to_string();
}
