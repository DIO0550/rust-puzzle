use bevy::{
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::{Commands, Res},
        world::World,
    },
    input::{keyboard::KeyCode, Input},
};

pub trait MenuItemSelectAction: Send + Sync {
    fn excecute(&self, world: &mut World);
}

pub fn select_menu_item_action<T>(world: &mut World)
where
    T: MenuItemSelectAction + Default,
{
    let Some(key_input) = world.get_resource_mut::<Input<KeyCode>>() else {
        return;
    };

    // キー入力のチェックを追加（任意）
    if !key_input.just_released(KeyCode::Space) {
        return;
    }

    let action = T::default();
    action.excecute(world);
}
