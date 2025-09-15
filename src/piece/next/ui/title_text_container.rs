use bevy::prelude::*;

use crate::asset::font::font_assets::FontAssets;

struct NextPieceTitleText;
impl NextPieceTitleText {
    fn spawn(font_assets: &Res<FontAssets>) -> (Text, TextFont, TextColor) {
        (
            Text::new("Next".to_string()),
            TextFont {
                font: font_assets.hachi_maru_pop_regular.clone(),
                font_size: 50.,
                ..default()
            },
            TextColor(Color::WHITE),
        )
    }
}

pub struct NextPieceTitleTextContainer;

impl NextPieceTitleTextContainer {
    fn style() -> Node {
        Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(65.0),
                bottom: Val::Px(0.0),
            },
            ..default()
        }
    }

    pub fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        font_assets: &Res<FontAssets>,
    ) {
        commands.entity(parent_entity).with_children(|parent| {
            parent
                .spawn((Node { ..Self::style() },))
                .with_children(|parent| {
                    let (text, font, color) = NextPieceTitleText::spawn(font_assets);
                    parent.spawn((text, font, color));
                });
        });
    }
}
