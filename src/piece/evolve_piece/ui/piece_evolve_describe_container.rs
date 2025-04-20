use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    ui::{node_bundles::NodeBundle, AlignItems, FlexDirection, JustifyContent, Style, Val},
    utils::default,
};

use crate::{
    asset::image::piece_image_assets::PieceImageAssets,
    piece::component::animal_piece::piece_type::PieceType,
    ui::image::{game_image_bundle::GameImageBundle, piece_image::PieceImage},
};

struct PieceEvolveDescribeImageRow;
impl PieceEvolveDescribeImageRow {
    fn style() -> Style {
        Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    fn spawn_as_child(
        parent: &mut ChildBuilder,
        piece_image_assets: &Res<PieceImageAssets>,
        piece_types: Vec<PieceType>,
        image_size: f32,
    ) {
        for piece_type in piece_types {
            let bundle = PieceImage::image_bundle(piece_type, &piece_image_assets, &image_size);

            parent.spawn(bundle);
        }
    }
}

pub(crate) struct PieceEvolveDescribeContainer;

impl PieceEvolveDescribeContainer {
    fn style() -> Style {
        Style {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }
    }

    pub(crate) fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        piece_image_assets: &Res<PieceImageAssets>,
    ) {
        let piece_type_groups: Vec<Vec<PieceType>> = vec![
            vec![PieceType::Rat, PieceType::Cat, PieceType::Dog],
            vec![PieceType::Elephant, PieceType::Penguin],
            vec![PieceType::Horse, PieceType::Panda, PieceType::Giraffe],
        ];

        commands.entity(parent_entity).with_children(|parent| {
            for piece_image_types in piece_type_groups {
                parent
                    .spawn(NodeBundle {
                        style: Self::style(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // 子エンティティを生成
                        PieceEvolveDescribeImageRow::spawn_as_child(
                            parent,
                            piece_image_assets,
                            piece_image_types,
                            50.0,
                        );
                    });
            }
        });
    }
}
