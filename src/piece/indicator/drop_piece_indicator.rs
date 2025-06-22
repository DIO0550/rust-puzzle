use bevy::ecs::component::Component;

use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    math::Vec2,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    piece::component::{active_piece::ActivePiece, falling::Falling},
    resource::drop_postion::DropPosition,
};

#[derive(Component)]
pub(crate) struct DropPieceIndicator;

pub(crate) fn spawn_drop_piece_indicator(
    mut commands: Commands,
    drop_piece_indicator: Query<Entity, With<DropPieceIndicator>>,
    drop_position: Res<DropPosition>,
    active_piece_query: Query<Entity, With<ActivePiece>>,
) {
    if active_piece_query.is_empty() {
        return;
    }

    // スポーン済み
    if !drop_piece_indicator.is_empty() {
        return;
    }

    let position_x = drop_position.x;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // 色を指定
                color: Color::rgba(1.0, 1.0, 0.0, 0.6),
                // サイズを指定（幅、高さ）
                custom_size: Some(Vec2::new(3.0, 600.0)), // 幅3px、高さ600pxの矩形
                // テクスチャの一部を切り取り（アトラス使用時）
                rect: None,
                // 水平/垂直反転
                flip_x: false,
                flip_y: false,
                // アンカーポイント（中心、左上など）
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            // 位置、回転、スケール
            transform: Transform::from_xyz(position_x, 0.0, 5.0),
            ..default()
        },
        DropPieceIndicator,
    ));
}

pub(crate) fn despawn_drop_piece_indicator(
    mut commands: Commands,
    falling_query: Query<&Falling>,
    query: Query<Entity, With<DropPieceIndicator>>,
) {
    if falling_query.is_empty() {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn update_drop_piece_indicator_position(
    mut transform_query: Query<&mut Transform, With<DropPieceIndicator>>,
    drop_position: Res<DropPosition>,
) {
    if transform_query.is_empty() {
        return;
    }

    let position_x = drop_position.x;

    for mut transform in transform_query.iter_mut() {
        transform.translation.x = position_x;
    }
}
