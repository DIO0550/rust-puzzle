use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

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
    ) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
        let mesh = self.meshes.add(Circle::new(size));
        let material = self.materials.add(ColorMaterial::from(image));
        (Mesh2d(mesh), MeshMaterial2d(material))
    }
}
