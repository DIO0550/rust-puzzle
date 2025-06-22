use bevy::text::Font;

use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum FontName {
    HachiMaruPopReg,
}

impl AssetNameTrait for FontName {
    fn asset_path(&self) -> String {
        match self {
            Self::HachiMaruPopReg => "font/Hachi_Maru_Pop/HachiMaruPop-Regular.ttf".to_string(),
        }
    }
}

pub struct FontAsset {}
impl AssetTrait<FontName, Font> for FontAsset {}
