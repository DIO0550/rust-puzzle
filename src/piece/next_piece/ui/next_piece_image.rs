use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    ui::{Style, Val},
};

use crate::{
    asset::image::piece_image_assets::PieceImageAssets,
    piece::next_piece::resource::next_piece::NextPiece,
    ui::image::{game_image_bundle::GameImageBundle, piece_image::PieceImage},
};
const PIECE_IMAGE_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct NextPieceImage;

impl NextPieceImage {
    fn style() -> Style {
        let image_size = 100.0;

        let style = Style {
            height: Val::Px(image_size),
            width: Val::Px(image_size),
            ..Default::default()
        };

        return style;
    }

    pub fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        piece_image_assets: &Res<PieceImageAssets>,
        next_piece_res: &Res<NextPiece>,
    ) {
        let piece_image_bundle =
            PieceImage::image_bundle(next_piece_res.0, piece_image_assets, &PIECE_IMAGE_SIZE);

        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn((piece_image_bundle, NextPieceImage));
        });
    }
}
