// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_heart.rs
use macroquad::{math::Vec3, models::Mesh};
use bevy_ecs::prelude::*;

/// It is vector of your 3D models.
pub struct Acorn3DAssetDatabase {
    pub meshes: Vec<Mesh>
}

#[derive(Component)]
pub struct Entity3DTransform {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(Component)]
pub struct Entity3DModel {
    pub mesh_id: usize // instead of Mesh
}