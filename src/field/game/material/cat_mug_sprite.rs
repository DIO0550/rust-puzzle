use bevy::{
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider};

use crate::{
    asset::image::game_image_assets::GameImageAssets,
    consts::consts::{BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS},
};

#[derive(Component)]
pub(crate) struct CatMugSprite;

impl CatMugSprite {
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
                y: 0.0,
                z: -1.0,
            },
            ..default()
        }
    }

    fn collider() -> Collider {
        return Collider::compound(vec![
            // 左
            (
                Vec2 {
                    x: -BOX_SIZE_WIDTH / 2.0,
                    y: 0.0,
                },
                0.0,
                Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT / 2.0),
            ),
            // 真ん中
            (
                Vec2 {
                    x: 0.0,
                    y: -BOX_SIZE_HEIHT / 2.0,
                },
                0.0,
                Collider::cuboid(BOX_SIZE_WIDTH / 2.0 + BOX_THICKNESS, BOX_THICKNESS),
            ),
            // 右
            (
                Vec2 {
                    x: BOX_SIZE_WIDTH / 2.0,
                    y: 0.0,
                },
                0.0,
                Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT / 2.0),
            ),
        ]);
    }

    pub(crate) fn spawn(commands: &mut Commands, game_image_assets: &Res<GameImageAssets>) {
        let cat_mug_image = game_image_assets.cat_mug.clone();
        let cat_mug_bundle = SpriteBundle {
            sprite: Self::sprite(Some(Vec2 {
                x: BOX_SIZE_WIDTH + BOX_THICKNESS * 2.0,
                y: BOX_SIZE_HEIHT + BOX_THICKNESS * 2.0,
            })),
            texture: cat_mug_image,
            transform: Self::transform(),
            ..default()
        };
        commands
            .spawn(Self::collider())
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(cat_mug_bundle);
    }
}
