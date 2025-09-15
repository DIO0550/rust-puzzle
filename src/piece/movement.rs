use bevy::prelude::*;

use crate::{
    parameter::input::PlayerInput,
    piece::{
        component::{
            active_piece::ActivePiece, animal_piece::animal_piece_component::AnimalPieceComponent,
        },
        drop::DropPosition,
    },
};

const PIECE_SPEED: f32 = 500.0;

#[derive(Debug, Clone, Copy)]
pub struct PieceMovement {
    direction: f32,
    speed: f32,
    delta_time: f32,
}

impl PieceMovement {
    pub fn new(input: &PlayerInput, speed: f32, delta_time: f32) -> Self {
        PieceMovement {
            direction: match (input.is_key_pressed_left(), input.is_key_pressed_right()) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            },
            speed,
            delta_time,
        }
    }

    pub fn calculate_new_position(&self, current_x: f32) -> f32 {
        current_x + self.direction * self.speed * self.delta_time
    }

    pub fn is_moving(&self) -> bool {
        self.direction != 0.0
    }
}

/**
 * ピース移動
 */
pub fn move_piece(
    mut commands: Commands,
    input: PlayerInput,
    mut query: Query<(&mut Transform, &AnimalPieceComponent), With<ActivePiece>>,
    time: Res<Time>,
) {
    let Ok((mut transform, animal_piece_component)) = query.single_mut() else {
        return;
    };

    let piece_movement = PieceMovement::new(&input, PIECE_SPEED, time.delta_secs());
    if !piece_movement.is_moving() {
        return;
    }

    let new_position = piece_movement.calculate_new_position(transform.translation.x);
    let new_drop_position =
        DropPosition::new(new_position, animal_piece_component.animal_piece.as_ref());
    transform.translation.x = new_drop_position.x;
    commands.insert_resource(new_drop_position);
}
