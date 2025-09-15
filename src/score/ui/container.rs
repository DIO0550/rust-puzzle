use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    prelude::{AlignItems, FlexDirection, Node, PositionType, Val},
};

use crate::{
    asset::image::{game_image_assets::GameImageAssets, image::ImageName},
    ui::image::{game_image::GameImage, game_image_bundle::GameImageBundleWithStyle},
};

#[derive(Component)]
pub struct ScoreTextContainer;

impl ScoreTextContainer {
    fn style() -> Node {
        let image_size = 250.0;
        return Node {
            position_type: PositionType::Absolute,
            left: Val::Px(50.),
            top: Val::Px(50.),
            width: Val::Px(image_size),
            height: Val::Px(image_size),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        };
    }

    pub fn spawn(commands: &mut Commands, game_image_assets: &Res<GameImageAssets>) -> Entity {
        let cat_hund_image =
            GameImage::image_bundle(ImageName::CatHand, &game_image_assets, Self::style());

        let entity = commands.spawn((cat_hund_image, ScoreTextContainer)).id();

        return entity;
    }
}
