use super::avatar_format;
use super::avatar_gltf::AvatarGltfObject;
use super::super::world::world_format::SpawnPoint;
use godot::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use thiserror::Error;
use crate::error::SpanErr;

pub fn load_avatar(yaml_path: String,spawn_point:SpawnPoint, parent_node: &mut Gd<Node>) -> Result<(), SpanErr<AvatarLoadingError>> {

    let mut avatar_yaml = File::open(&yaml_path).map_err(|e| SpanErr::from(AvatarLoadingError::InvalidYamlPath(e.to_string())))?;    
    let mut yaml_content = String::new(); 
    let _ = avatar_yaml.read_to_string(&mut yaml_content).map_err(|e| SpanErr::from(AvatarLoadingError::YamlFileOpen(e.to_string(),yaml_content.clone())));
    let yaml_content = serde_yaml::from_str::<avatar_format::SuteraAvatarYamlFormat>(&yaml_content).map_err(|e| SpanErr::from(AvatarLoadingError::InvalidYamlPath(e.to_string())))?;
    let mut avatar = AvatarGltfObject::new(&yaml_content.specs.avatar_model.path)?;
    let _ = avatar.spawn_avatar(spawn_point, parent_node)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum AvatarLoadingError {
    #[error("This avatar yaml file path ({0}) is invalid")]
    InvalidYamlPath(String),
    #[error("Failed to opening yaml file. Please check file ({1}). Reason: {0}")]
    YamlFileOpen(String, String),
    #[error("Failed to load yaml file. Please check file. Reason: {0}")]
    SerdeYamlLoading(String),
    #[error("Failed to opening gltf file. Please check file ({1}). Reason: {0}")]
    GltfFileOpen(String, String),
    #[error("Failed to generate 3d model. Please check file or settings. Reason: {0}")]
    Generate3DModel(String),
}
