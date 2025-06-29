use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource, SystemParam},
    },
    math::Vec2,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    parameter::input::PlayerInput,
    piece::{
        component::{
            active_piece::ActivePiece,
            animal_piece::{
                animal_piece::AnimalPiece, animal_piece_component::AnimalPieceComponent,
            },
            falling::{Falling, PieceFaller},
        },
        ext::commands_ext::PieceCommandsExt,
    },
    BOX_SIZE_WIDTH, BOX_THICKNESS, UNIT_WIDTH,
};

#[derive(Resource)]
pub struct DropPosition {
    pub x: f32,
}

impl DropPosition {
    pub fn new(position: f32, animal_piece: &dyn AnimalPiece) -> Self {
        let piece_size = animal_piece.get_size().to_f32() * UNIT_WIDTH * 2.0;
        let range = BOX_SIZE_WIDTH / 2.0 - BOX_THICKNESS;
        let max = range - piece_size;
        let min = piece_size - range;

        if position < min {
            let new_position = min;
            return DropPosition { x: new_position };
        }

        if max < position {
            let new_position = max;
            return DropPosition { x: new_position };
        }

        return DropPosition { x: position };
    }
}

#[derive(SystemParam)]
pub struct DropPositionController<'w> {
    pub grab_position: ResMut<'w, DropPosition>,
}

impl<'w> DropPositionController<'w> {
    pub fn set_grab_position(&mut self, animal_piece: &dyn AnimalPiece) {
        self.grab_position.x = DropPosition::new(self.grab_position.x, animal_piece).x;
    }
}

/**
 * ピースを離す
 */
pub fn drop_piece(
    mut commnads: Commands,
    mut piece_faller: PieceFaller,
    input: PlayerInput,
    mut query: Query<(Entity, &AnimalPieceComponent), With<ActivePiece>>,
) {
    if !input.is_key_just_released_space() {
        return;
    }

    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    commnads.convert_to_physical(entity, piece);
    piece_faller.make_falling(entity);
}

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
