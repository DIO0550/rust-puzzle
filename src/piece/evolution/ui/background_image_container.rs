use bevy::prelude::*;

use crate::{
    asset::image::{game_image_assets::GameImageAssets, image::ImageName},
    ui::image::{game_image::GameImage, game_image_bundle::GameImageBundleWithStyle as _},
};

pub(crate) struct PieceEvolveBackgroundImageContainer;
impl PieceEvolveBackgroundImageContainer {
    fn style(image_size: f32) -> Node {
        Node {
            width: Val::Px(image_size),
            height: Val::Px(image_size),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            ..default()
        }
    }

    pub(crate) fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        game_image_assets: &Res<GameImageAssets>,
        image_size: f32,
    ) -> Entity {
        let mut child_entity = Entity::PLACEHOLDER;

        let piece_evolve_image_bundle = GameImage::image_bundle(
            ImageName::PieceEvolve,
            game_image_assets,
            Self::style(image_size),
        );

        commands.entity(parent_entity).with_children(|parent| {
            child_entity = parent.spawn((piece_evolve_image_bundle,)).id();
        });

        child_entity
    }
}
