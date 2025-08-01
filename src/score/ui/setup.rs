use crate::{
    asset::{font::font_assets::FontAssets, image::game_image_assets::GameImageAssets},
    score::ui::{
        container::ScoreTextContainer, title_container::ScoreTitleTextContainer,
        value_container::ScoreValueTextContainer,
    },
};
use ::bevy::prelude::*;

pub fn setup_score(
    mut commands: Commands,
    game_image_assets: Res<GameImageAssets>,
    font_assets: Res<FontAssets>,
) {
    let score_text_container = ScoreTextContainer::spawn(&mut commands, &game_image_assets);
    ScoreTitleTextContainer::spawn_as_child(&mut commands, score_text_container, &font_assets);
    ScoreValueTextContainer::spawn_as_child(&mut commands, score_text_container, &font_assets);
}
