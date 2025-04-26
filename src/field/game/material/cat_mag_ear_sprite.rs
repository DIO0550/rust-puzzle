use bevy::{
    ecs::system::{Commands, Res},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    asset::image::game_image_assets::GameImageAssets,
    consts::consts::{BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS},
};

pub(crate) struct CatMagEarSprite;

impl CatMagEarSprite {
    fn sprite(custom_size: Option<Vec2>) -> Sprite {
        Sprite {
            custom_size: custom_size,
            ..default()
        }
    }

    fn transform() -> Transform {
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: BOX_SIZE_HEIHT / 2.0 + BOX_SIZE_HEIHT / 10.0 - BOX_THICKNESS * 2.0,
                z: -2.0,
            },
            ..default()
        }
    }

    pub(crate) fn spawn(commands: &mut Commands, game_image_assets: Res<GameImageAssets>) {
        let cat_mug_ear_image = game_image_assets.cat_mug_ear.clone();
        let cat_mug_ear_bundle = SpriteBundle {
            sprite: Self::sprite(Some(Vec2 {
                x: (BOX_SIZE_WIDTH + BOX_THICKNESS) * 3.0 / 4.0,
                y: BOX_SIZE_HEIHT / 5.0,
            })),
            texture: cat_mug_ear_image,
            transform: Self::transform(),
            ..default()
        };

        commands.spawn(cat_mug_ear_bundle);
    }
}
