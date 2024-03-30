use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    text::Font,
};

pub enum FontName {
    HachiMaruPopReg,
}

impl FontName {
    fn font_resouce_path(&self) -> String {
        match self {
            Self::HachiMaruPopReg => "font/Hachi_Maru_Pop/HachiMaruPop-Regular".to_string(),
        }
    }
}

pub trait FontResourceTrait {
    fn get_font_resouce(asset_server: &Res<AssetServer>, font_name: &FontName) -> Handle<Font>;
}
pub struct FontResource {}
impl FontResourceTrait for FontResource {
    fn get_font_resouce(asset_server: &Res<AssetServer>, font_name: &FontName) -> Handle<Font> {
        asset_server.load(font_name.font_resouce_path())
    }
}
