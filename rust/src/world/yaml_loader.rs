use super::error::SpanErr;
use super::gltf::SuteraGltfObject;
use super::world_format::SuteraWorldYamlFormat;
use super::transform::SuteraTransform;
use godot::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use thiserror::Error;
use tracing::instrument;

#[instrument(name = "load_world", level = "trace")]
pub fn load_world(
    yaml_path: String,
    parent_node: &mut Gd<Node>,
) -> Result<(), SpanErr<WorldLoadingError>> {
    let mut base_node = parent_node.clone();

    //yamlファイルのpathを開く
    let mut yaml_file = File::open(&yaml_path)
        .map_err(|e| SpanErr::from(WorldLoadingError::InvalidYamlPath(e.to_string())))?;
    let mut contents = String::new();
    yaml_file.read_to_string(&mut contents).map_err(|e| {
        SpanErr::from(WorldLoadingError::YamlFileOpen(
            e.to_string(),
            yaml_path.clone(),
        ))
    })?; //yamlファイルの内容をString型に

    let world:SuteraWorldYamlFormat = serde_yaml::from_str(&contents)  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e| SpanErr::from(WorldLoadingError::SerdeYamlLoading(e.to_string())))?;

    for obj_data in world.specs.objects.iter() {
        let transform = SuteraTransform::from(obj_data.model.transform.clone());
        let mut obj = SuteraGltfObject::new(&obj_data.model.path, transform)?;
        let _ = obj.generate_model(&mut base_node);
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum WorldLoadingError {
    #[error("This world file path ({0}) is invalid")]
    InvalidYamlPath(String),
    #[error("This yaml file ({1}) does not conform to SuteraWorldFormat. detail: {0}")]
    InvalidSuteraFormat(String, String),
    #[error("'{0}' key was not fount in {1}.")]
    KeysNotFound(String, String),
    #[error("This 3D object type '{0}' is not yet implemented.")]
    InvalidObjectType(String),
    #[error("This 3D object type '{0}' is None in {1}.")]
    NoneObjectType(String, String),
    #[error("Failed to load yaml file. Please check file. Reason: {0}")]
    SerdeYamlLoading(String),
    #[error("Failed to opening yaml file. Please check file ({1}). Reason: {0}")]
    YamlFileOpen(String, String),
    #[error("Failed to opening glTF file in {1}. Please check file. Reason: {0}")]
    GltfFileOpen(String, String),
    #[error("Failed to generate 3d model. Please check file or settings. Reason: {0}")]
    Generate3DModel(String),
}
