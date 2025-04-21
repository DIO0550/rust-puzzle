use bevy::{
    a11y::accesskit::Node,
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    ui::{
        node_bundles::NodeBundle, AlignItems, Display, FlexDirection, JustifyContent, Style,
        UiRect, Val,
    },
    utils::default,
};

use crate::{
    asset::image::{self, piece_image_assets::PieceImageAssets},
    piece::component::animal_piece::piece_type::PieceType,
    ui::image::{game_image_bundle::GameImageBundle, piece_image::PieceImage},
};

struct PieceEvolveDescribeImageRow;
impl PieceEvolveDescribeImageRow {
    fn style() -> Style {
        Style {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }
    }

    fn spawn_as_child(
        parent: &mut ChildBuilder,
        piece_image_assets: &Res<PieceImageAssets>,
        piece_types: Vec<PieceType>,
        image_size: f32,
    ) {
        parent
            .spawn(NodeBundle {
                style: Self::style(),
                ..default()
            })
            .with_children(|parent| {
                for piece_type in piece_types {
                    let bundle =
                        PieceImage::image_bundle(piece_type, &piece_image_assets, &image_size);

                    parent.spawn(bundle);
                }
            });
    }
}

pub(crate) struct PieceEvolveDescribeContainer;

impl PieceEvolveDescribeContainer {
    fn style() -> Style {
        Style {
            margin: UiRect {
                left: (Val::Px(50.0)),
                right: (Val::Px(42.0)),
                top: (Val::Px(40.0)),
                bottom: (Val::Px(0.0)),
            },
            row_gap: Val::Px(60.0),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,

            ..default()
        }
    }

    pub(crate) fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        piece_image_assets: &Res<PieceImageAssets>,
        image_size: f32,
    ) {
        let piece_type_groups: Vec<Vec<PieceType>> = vec![
            vec![PieceType::Rat, PieceType::Cat, PieceType::Dog],
            vec![PieceType::Elephant, PieceType::Penguin],
            vec![PieceType::Giraffe, PieceType::Panda, PieceType::Horse],
        ];

        commands.entity(parent_entity).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Self::style(),
                    ..default()
                })
                .with_children(|parent| {
                    for piece_image_types in piece_type_groups {
                        // 子エンティティを生成
                        PieceEvolveDescribeImageRow::spawn_as_child(
                            parent,
                            piece_image_assets,
                            piece_image_types,
                            image_size,
                        );
                    }
                });
        });
    }
}
