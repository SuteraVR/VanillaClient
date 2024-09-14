use super::error::SpanErr;
use super::gltf::SuteraGltfObject;
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
    let mut yaml_file = File::open(yaml_path)
        .map_err(|e| SpanErr::from(WorldLoadingError::InvalidYamlPathErr(e.to_string())))?;
    let mut contents = String::new();
    yaml_file
        .read_to_string(&mut contents)
        .map_err(|e| SpanErr::from(WorldLoadingError::YamlFileOpenError(e.to_string())))?; //yamlファイルの内容をString型に

    let value = Value::deserialize(Deserializer::from_str(&contents))  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e|{SpanErr::from(WorldLoadingError::SerdeYamlLoadingError(e.to_string()))})?;

    let sequence = get_sequence(&value)?; //objectsのsequenceを取得

    for obj in sequence.iter() {
        let Some(model) = obj.get("model") else{
            return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
                "'model' key was not found.".to_string(),
            )))
        };
        match model.get("type") {
            // model_type がgltfだった場合の処理
            Some(model_type) if model_type.as_str() == Some("gltf") => {
                let path = match model.get("path").and_then(|p| p.as_str()) {
                    Some(path) => path,
                    None => {
                        return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
                            "'path' key was not found.".to_string(),
                        )))
                    }
                };
                let transform = get_transform(model)?;
                let mut gltf_obj = SuteraGltfObject::new(path.to_string(), transform)?;
                gltf_obj.generate_model(&mut base_node)?;
            }

            // model_type が対応しているものでなかった場合
            Some(model_type) => {

                // model_type が空だったときの処理
                let Some(type_str) = model_type.as_str() else {
                    let e = SpanErr::from(WorldLoadingError::InvalidObjectTypeErr("model_type was None.".to_string()));
                    return Err(e);
                };

                let e = SpanErr::from(WorldLoadingError::InvalidObjectTypeErr(
                    type_str.to_string()
                ));
                return Err(e);
            }
            None => {
                return Err(SpanErr::from(worldformat_error("type").error));
            }
        }
    }
    Ok(())
}

//yamlから3dモデルの情報のsequence(Vector)を取得
#[instrument(skip_all, name = "get_sequence", level = "trace")]
fn get_sequence(value: &Value) -> Result<Sequence, SpanErr<WorldLoadingError>> {
    let Some(specs) = value.get("specs") else{
        return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr("'specs' key was not found.".to_string(),)
    ))};
    let Some(objects) = specs.get("objects") else{
        return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
            "'objects' key was not found.".to_string(),
        )))
    };
    match objects.as_sequence() {
        Some(sequence) => return Ok(sequence.clone()),
        None => {
            return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
                "couldn't get sequence from this object.".to_string(),
            )))
        }
    };
}

#[instrument(skip_all, name = "worldformaterror", level = "trace")]
fn worldformat_error<'a, T: Into<Cow<'a, str>>>(key: T) -> SpanErr<WorldLoadingError> {
    let reason = format!("'{}' key was not found ", key.into());
    let e = SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(reason));
    return e;
}

#[instrument(skip_all, name = "get_transform", level = "trace")]
fn get_transform(model: &Value) -> Result<[f32; 10], SpanErr<WorldLoadingError>> {
    let mut transform: [f32; 10] = [0.0; 10];
    if let Some(transform_vec) = model.get("transform").and_then(|s| s.as_sequence()) {
        if transform_vec.len() != 10 {
            Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
                "Object transform need 10 numbers:(x,y,z,rx,ry,rz,rw,sx,sy,sz)".to_string(),
            )))
        } else {
            for i in 0..10 {
                if let Some(element) = transform_vec[i].as_f64() {
                    transform[i] = element as f32;
                }
            }
            Ok(transform)
        }
    } else {
        Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(
            "'transform' key was not found.".to_string(),
        )))
    }
}

#[derive(Error, Debug)]
pub enum WorldLoadingError {
    #[error("This world file path ({0}) is invalid")]
    InvalidYamlPathErr(String),
    #[error("This yaml file does not conform to SuteraWorldFormat. detail: {0}")]
    InvalidSuteraFormatErr(String),
    #[error("This 3Dobject type '{0}' is not yet implemented.")]
    InvalidObjectTypeErr(String),
    #[error("Failed to load yaml file. Please check file. Reason: {0}")]
    SerdeYamlLoadingError(String),
    #[error("Failed to opening yaml file. Please check file. Reason: {0}")]
    YamlFileOpenError(String),
    #[error("Failed to opening glTF file. Please check file. Reason: {0}")]
    GltfFileOpenError(String),
    #[error("Failed to generate 3d model. Please check file or settings. Reason: {0}")]
    Generate3DModelError(String),
}
