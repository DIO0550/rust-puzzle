use bevy::{
    asset::{AssetServer, Assets},
    math::{Vec2, Vec3},
    prelude::{default, shape::Quad, Commands, Mesh, Res, ResMut, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::{
    asset::{
        asset::AssetTrait,
        image::image::{ImageAsset, ImageName},
    },
    score::resource::score::Score,
    BOX_MARGIN_BOTTOM, BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS,
};

pub fn restart(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    commands.insert_resource(Score(0));
    config.physics_pipeline_active = true;
}

pub fn setup_cat_mug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let image = ImageAsset::asset(&asset_server, &ImageName::CatMug);
    let material = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(
            meshes.add(
                Quad::new(Vec2 {
                    x: (BOX_SIZE_WIDTH * 3.5 - BOX_THICKNESS * 4.0),
                    y: (BOX_SIZE_HEIHT * 2.5 - 5.0),
                })
                .into(),
            ),
        ),
        transform: Transform {
            translation: Vec3 {
                x: (55.0),
                y: (-BOX_MARGIN_BOTTOM * 2.0 - 17.0),
                z: (-1.00),
            },
            ..default()
        },
        material: materials.add(image.into()),
        ..default()
    };

    // commands.spawn(material);
}
