use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, Style,
    },
    utils::default,
};

use crate::asset::font::font_assets::FontAssets;

struct PieceEvolveTitleText;

impl PieceEvolveTitleText {
    fn text_style(font_assets: &Res<FontAssets>) -> TextStyle {
        TextStyle {
            font: font_assets.hachi_maru_pop_regular.clone(),
            font_size: 50.,
            color: Color::BLACK,
            ..default()
        }
    }

    fn value() -> String {
        "進化順".to_string()
    }

    fn spawn(font_assets: &Res<FontAssets>) -> TextBundle {
        TextBundle::from_section(Self::value(), Self::text_style(font_assets))
    }
}

pub(crate) struct PieceEvolveTitleTextContainer;
impl PieceEvolveTitleTextContainer {
    fn style() -> Style {
        Style {
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
                .spawn((NodeBundle {
                    style: Self::style(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(PieceEvolveTitleText::spawn(font_assets));
                });
        });
    }
}
