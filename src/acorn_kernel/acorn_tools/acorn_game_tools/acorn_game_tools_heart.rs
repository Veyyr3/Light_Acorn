// src/acorn_kernel/acorn_tools/acorn_game_tools/acorn_game_tools_heart.rs

/// It is your 3D model.
/// Rules of Macroquad: you must use u16 for indices (it's strict).
pub struct AcornMeshAsset {
    pub vertices: Vec<f32>, // How many dots in your 3D model. Hold on... What? Imagine: 0.6 dots in your triangle.
    pub indices: Vec<u16> // You say to GPU: how link your dots by pack with 3 vertices
}

/// It is vector of your 3D models.
pub struct AssetDatabase {
    pub meshes: Vec<AcornMeshAsset>
}