use bevy::{
    ecs::system::{Res, SystemParam},
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
};

#[derive(SystemParam)]
pub struct PlayerInput<'w> {
    keyboard_input: Res<'w, ButtonInput<KeyCode>>,
    mouse_button_input: Res<'w, ButtonInput<MouseButton>>,
}

impl PlayerInput<'_> {
    pub fn is_key_pressed_left(&self) -> bool {
        self.keyboard_input.pressed(KeyCode::ArrowLeft)
    }

    pub fn is_key_pressed_right(&self) -> bool {
        self.keyboard_input.pressed(KeyCode::ArrowRight)
    }

    pub fn is_key_just_released_space(&self) -> bool {
        self.keyboard_input.just_released(KeyCode::Space)
    }
}
