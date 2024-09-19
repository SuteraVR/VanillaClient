use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SuteraWorldYamlFormat {
    pub metadata: WorldMetadata,
    pub specs: WorldSpecs,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldMetadata {
    pub world_format: String,
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldSpecs {
    pub spawnpoint: SpawnPoint,
    pub world_border: WorldBorder,
    pub objects: Vec<WorldObject>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldBorder {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorldObject {
    pub name: String,
    pub model: Sutera3DModel,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Sutera3DModel {
    pub model_type: String,
    pub path: String,
    pub transform: SuteraTransformSpecs,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SuteraTransformSpecs {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub rotation_w: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}
