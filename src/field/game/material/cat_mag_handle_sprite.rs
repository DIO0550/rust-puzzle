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

pub(crate) struct CatMagHandleSprite;

impl CatMagHandleSprite {
    fn sprite(custom_size: Option<Vec2>) -> Sprite {
        Sprite {
            custom_size: custom_size,
            ..default()
        }
    }

    fn transform(image_size: f32) -> Transform {
        Transform {
            translation: Vec3 {
                x: (BOX_SIZE_WIDTH / 2.0 + (image_size / 2.0) - BOX_THICKNESS),
                y: (0.0),
                z: (5.0),
            },
            ..default()
        }
    }

    pub(crate) fn spawn(commands: &mut Commands, game_image_assets: Res<GameImageAssets>) {
        let cat_mug_handle_image = game_image_assets.cat_mug_handle.clone();
        let image_width = BOX_SIZE_WIDTH / 4.0;
        let cat_mug_handle_bundle = SpriteBundle {
            sprite: Self::sprite(Some(Vec2 {
                x: image_width,
                y: BOX_SIZE_HEIHT,
            })),
            texture: cat_mug_handle_image,
            transform: Self::transform(image_width),
            ..default()
        };

        commands.spawn(cat_mug_handle_bundle);
    }
}
