use bevy::{
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    math::{Vec2, Vec3},
    prelude::*,
    utils::default,
};

use crate::{
    asset::image::game_image_assets::GameImageAssets,
    consts::consts::{BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS},
};

#[derive(Component)]
#[require(Transform::from_translation(Vec3 {
                x: 0.0,
                y: BOX_SIZE_HEIHT * 3.0,
                z: 1.0,
            }))]
pub(crate) struct CatMugEarSprite;

impl CatMugEarSprite {
    pub(crate) fn spawn(commands: &mut Commands, game_image_assets: &Res<GameImageAssets>) {
        let cat_mug_ear_image = game_image_assets.cat_mug_ear.clone();

        commands.spawn(CatMugEarSprite).insert(Sprite {
            custom_size: Some(Vec2 {
                x: (BOX_SIZE_WIDTH + BOX_THICKNESS) * 3.0 / 4.0,
                y: BOX_SIZE_HEIHT / 5.0,
            }),
            image: cat_mug_ear_image,
            ..default()
        });
    }
}
