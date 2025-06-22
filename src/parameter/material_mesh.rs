use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::render::mesh::shape::Circle;
use bevy::sprite::MaterialMesh2dBundle;

// MeshとMaterialを一緒に管理するSystemParam
#[derive(SystemParam)]
pub struct MeshMaterial<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<ColorMaterial>>,
}

impl<'w> MeshMaterial<'w> {
    // 円形のピースを作成するメソッド
    pub fn create_circle_material_mesh(
        &mut self,
        size: f32,
        image: Handle<Image>,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(self.meshes.add(Circle::new(size).into())),
            material: self.materials.add(image.into()),
            ..default()
        }
    }
}
