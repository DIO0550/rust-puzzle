use bevy::{
    ecs::system::{Res, SystemParam},
    input::{keyboard::KeyCode, mouse::MouseButton, Input},
};

#[derive(SystemParam)]
pub struct PlayerInput<'w> {
    keyboard_input: Res<'w, Input<KeyCode>>,
    mouse_button_input: Res<'w, Input<MouseButton>>,
}

impl PlayerInput<'_> {
    pub fn is_key_pressed_left(&self) -> bool {
        self.keyboard_input.pressed(KeyCode::Left)
    }

    pub fn is_key_pressed_right(&self) -> bool {
        self.keyboard_input.pressed(KeyCode::Right)
    }

    pub fn is_key_just_released_space(&self) -> bool {
        self.keyboard_input.just_released(KeyCode::Space)
    }
}
