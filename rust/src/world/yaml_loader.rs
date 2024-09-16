use super::error::SpanErr;
use super::gltf::SuteraGltfObject;
use super::transform::SuteraTransform;
use godot::prelude::*;
use serde::Deserialize;
use serde_yaml::{Deserializer, Sequence, Value};
use std::borrow::Cow;
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

    let value = Value::deserialize(Deserializer::from_str(&contents))  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e| SpanErr::from(WorldLoadingError::SerdeYamlLoading(e.to_string())))?;

    let sequence = get_sequence(&value, yaml_path.clone())?; //objectsのsequenceを取得

    for obj in sequence.iter() {
        let Some(model) = obj.get("model") else {
            return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
                "model".to_string(),
                yaml_path,
            )));
        };
        match model.get("type") {
            // model_type がgltfだった場合の処理
            Some(model_type) if model_type.as_str() == Some("gltf") => {
                let path = match model.get("path").and_then(|p| p.as_str()) {
                    Some(path) => path,
                    None => {
                        return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
                            "path".to_string(),
                            yaml_path,
                        )))
                    }
                };
                let Some(transform_value) = model.get("transform") else {
                    return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
                        "transform".to_string(),
                        yaml_path,
                    )));
                };
                let transform = SuteraTransform::from_yaml(transform_value, yaml_path.clone())?;
                let mut gltf_obj = SuteraGltfObject::new(path.to_string(), transform)?;
                gltf_obj.generate_model(&mut base_node)?;
            }

            // model_type が対応しているものでなかった場合
            Some(model_type) => {
                // model_type が空だったときの処理
                let Some(type_str) = model_type.as_str() else {
                    let e = SpanErr::from(WorldLoadingError::NoneObjectType(
                        "model_type".to_string(),
                        yaml_path,
                    ));
                    return Err(e);
                };

                let e = SpanErr::from(WorldLoadingError::InvalidObjectType(type_str.to_string()));
                return Err(e);
            }
            None => {
                return Err(world_format_error("type", yaml_path));
            }
        }
    }
    Ok(())
}

//yamlから3dモデルの情報のsequence(Vector)を取得
#[instrument(skip_all, name = "get_sequence", level = "trace")]
fn get_sequence(value: &Value, yaml_path: String) -> Result<Sequence, SpanErr<WorldLoadingError>> {
    let Some(specs) = value.get("specs") else {
        return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
            "specs".to_string(),
            yaml_path,
        )));
    };
    let Some(objects) = specs.get("objects") else {
        return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
            "objects".to_string(),
            yaml_path,
        )));
    };
    match objects.as_sequence() {
        Some(sequence) => return Ok(sequence.clone()),
        None => {
            return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat(
                "couldn't get sequence from this object.".to_string(),
                yaml_path,
            )))
        }
    };
}

#[instrument(skip_all, name = "world_format_error", level = "trace")]
fn world_format_error<'a, T: Into<Cow<'a, str>>>(
    key: T,
    yaml_path: String,
) -> SpanErr<WorldLoadingError> {
    let e = SpanErr::from(WorldLoadingError::KeysNotFound(
        key.into().to_string(),
        yaml_path,
    ));
    return e;
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
