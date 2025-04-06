use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    ui::{AlignItems, FlexDirection, PositionType, Style, Val},
};

use crate::{
    asset::image::{game_image_assets::GameImageAssets, image::ImageName},
    ui::image::{game_image::GameImage, game_image_bundle::GameImageBundleWithStyle},
};

#[derive(Component)]
pub struct NextPieceContainer;

impl NextPieceContainer {
    pub fn style() -> Style {
        let image_size = 250.0;

        let style = Style {
            position_type: PositionType::Absolute,
            right: Val::Px(50.),
            top: Val::Px(65.),
            height: Val::Px(image_size),
            width: Val::Px(image_size),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        };
        return style;
    }

    pub fn spawn(commands: &mut Commands, game_image_assets: &Res<GameImageAssets>) -> Entity {
        let cat_silhouette_image =
            GameImage::image_bundle(ImageName::CatSilhouette, &game_image_assets, Self::style());

        return commands.spawn(cat_silhouette_image).id();
    }
}
