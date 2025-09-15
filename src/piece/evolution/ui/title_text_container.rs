use bevy::prelude::*;

use crate::asset::font::font_assets::FontAssets;

struct PieceEvolveTitleText;

impl PieceEvolveTitleText {
    fn spawn(font_assets: &Res<FontAssets>) -> (Text, TextFont, TextColor) {
        (
            Text::new(Self::value()),
            TextFont {
                font: font_assets.hachi_maru_pop_regular.clone(),
                font_size: 50.,
                ..default()
            },
            TextColor(Color::BLACK),
        )
    }

    fn value() -> String {
        "進化順".to_string()
    }
}

pub(crate) struct PieceEvolveTitleTextContainer;
impl PieceEvolveTitleTextContainer {
    fn style() -> Node {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    pub(crate) fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        font_assets: &Res<FontAssets>,
    ) {
        commands.entity(parent_entity).with_children(|parent| {
            parent
                .spawn((Node { ..Self::style() },))
                .with_children(|parent| {
                    parent.spawn(PieceEvolveTitleText::spawn(font_assets));
                });
        });
    }
}
