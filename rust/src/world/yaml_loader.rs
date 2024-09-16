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
    let mut yaml_file = File::open(&yaml_path)
        .map_err(|e| SpanErr::from(WorldLoadingError::InvalidYamlPath(e.to_string())))?;
    let mut contents = String::new();
    yaml_file
        .read_to_string(&mut contents)
        .map_err(|e| SpanErr::from(WorldLoadingError::YamlFileOpen(e.to_string(),yaml_path.clone())))?; //yamlファイルの内容をString型に

    let value = Value::deserialize(Deserializer::from_str(&contents))  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e| SpanErr::from(WorldLoadingError::SerdeYamlLoading(e.to_string())))?;

    let sequence = get_sequence(&value,yaml_path.clone())?; //objectsのsequenceを取得

    for obj in sequence.iter() {
        let Some(model) = obj.get("model") else{
            return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
                "model".to_string(), yaml_path
            )))
        };
        match model.get("type") {
            // model_type がgltfだった場合の処理
            Some(model_type) if model_type.as_str() == Some("gltf") => {
                let path = match model.get("path").and_then(|p| p.as_str()) {
                    Some(path) => path,
                    None => {
                        return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
                            "path".to_string(), yaml_path
                        )))
                    }
                };
                let transform = get_transform(model,yaml_path.clone())?;
                let mut gltf_obj = SuteraGltfObject::new(path.to_string(), transform)?;
                gltf_obj.generate_model(&mut base_node)?;
            }

            // model_type が対応しているものでなかった場合
            Some(model_type) => {

                // model_type が空だったときの処理
                let Some(type_str) = model_type.as_str() else {
                    let e = SpanErr::from(WorldLoadingError::NoneObjectType("model_type".to_string(),yaml_path));
                    return Err(e);
                };

                let e = SpanErr::from(WorldLoadingError::InvalidObjectType(
                    type_str.to_string()
                ));
                return Err(e);
            }
            None => {
                return Err(world_format_error("type",yaml_path));
            }
        }
    }
    Ok(())
}

//yamlから3dモデルの情報のsequence(Vector)を取得
#[instrument(skip_all, name = "get_sequence", level = "trace")]
fn get_sequence(value: &Value, yaml_path: String) -> Result<Sequence, SpanErr<WorldLoadingError>> {
    let Some(specs) = value.get("specs") else{
        return Err(SpanErr::from(WorldLoadingError::KeysNotFound("specs".to_string(), yaml_path)
    ))};
    let Some(objects) = specs.get("objects") else{
        return Err(SpanErr::from(WorldLoadingError::KeysNotFound(
            "objects".to_string(), yaml_path
        )))
    };
    match objects.as_sequence() {
        Some(sequence) => return Ok(sequence.clone()),
        None => {
            return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat(
                "couldn't get sequence from this object.".to_string(), yaml_path
            )))
        }
    };
}

#[instrument(skip_all, name = "world_format_error", level = "trace")]
fn world_format_error<'a, T: Into<Cow<'a, str>>>(key: T, yaml_path: String) -> SpanErr<WorldLoadingError> {
    let e = SpanErr::from(WorldLoadingError::KeysNotFound(key.into().to_string(), yaml_path));
    return e;
}

#[instrument(skip_all, name = "get_transform", level = "trace")]
pub fn get_transform(model: &Value, yaml_path: String) -> Result<(Vector3,Quaternion,Vector3), SpanErr<WorldLoadingError>> {
    let Some(transform_vec) = model.get("transform") else {return Err(SpanErr::from(WorldLoadingError::KeysNotFound("transform".to_string(), yaml_path)))};

    let position = get_position(transform_vec, yaml_path.clone())?;
    let rotation = get_rotation(transform_vec, yaml_path.clone())?;
    let scale = get_scale(transform_vec, yaml_path.clone())?;

    Ok((position,rotation,scale))
}

#[instrument(skip_all, name = "get_position", level = "trace")]
pub fn get_position(transform_vec:&Value, yaml_path: String) -> Result<Vector3,SpanErr<WorldLoadingError>>{
    let Some(position_x) = transform_vec.get("position_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_x".to_string(),yaml_path)))};
    let Some(position_x) = position_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("'position_x' is not f32 value".to_string(),yaml_path)))};
    let Some(position_y) = transform_vec.get("position_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_y".to_string(),yaml_path)))};
    let Some(position_y) = position_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'position_y' is not f32 value".to_string(),yaml_path)))};
    let Some(position_z) = transform_vec.get("position_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_z".to_string(),yaml_path)))};
    let Some(position_z) = position_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'position_z' is not f32 value".to_string(),yaml_path)))};

    let position:Vector3 = Vector3::new(position_x as f32,position_y as f32,position_z as f32);
    return Ok(position)
}

#[instrument(skip_all, name = "get_rotation", level = "trace")]
pub fn get_rotation(transform_vec:&Value, yaml_path: String) -> Result<Quaternion,SpanErr<WorldLoadingError>>{
    let Some(rotation_x) = transform_vec.get("rotation_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_x".to_string(),yaml_path)))};
    let Some(rotation_x) = rotation_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("'rotation_x' is not f32 value".to_string(),yaml_path)))};
    let Some(rotation_y) = transform_vec.get("rotation_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_y".to_string(),yaml_path)))};
    let Some(rotation_y) = rotation_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_y' is not f32 value".to_string(),yaml_path)))};
    let Some(rotation_z) = transform_vec.get("rotation_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_z".to_string(),yaml_path)))};
    let Some(rotation_z) = rotation_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_z' is not f32 value".to_string(),yaml_path)))};
    let Some(rotation_w) = transform_vec.get("rotation_w") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_w".to_string(),yaml_path)))};
    let Some(rotation_w) = rotation_w.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_w' is not f32 value".to_string(),yaml_path)))};

    let rotation:Quaternion = Quaternion::new(rotation_x as f32,rotation_y as f32,rotation_z as f32,rotation_w as f32);
    return Ok(rotation)
}

#[instrument(skip_all, name = "get_scale", level = "trace")]
pub fn get_scale(transform_vec:&Value, yaml_path: String) -> Result<Vector3,SpanErr<WorldLoadingError>>{
    let Some(scale_x) = transform_vec.get("scale_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_x".to_string(),yaml_path)))};
    let Some(scale_x) = scale_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("'scale_x' is not f32 value".to_string(),yaml_path)))};
    let Some(scale_y) = transform_vec.get("scale_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_y".to_string(),yaml_path)))};
    let Some(scale_y) = scale_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'scale_y' is not f32 value".to_string(),yaml_path)))};
    let Some(scale_z) = transform_vec.get("scale_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_z".to_string(),yaml_path)))};
    let Some(scale_z) = scale_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'scale_z' is not f32 value".to_string(),yaml_path)))};

    let scale:Vector3 = Vector3::new(scale_x as f32,scale_y as f32,scale_z as f32);
    return Ok(scale)
}

#[derive(Error, Debug)]
pub enum WorldLoadingError {
    #[error("This world file path ({0}) is invalid")]
    InvalidYamlPath(String),
    #[error("This yaml file ({1}) does not conform to SuteraWorldFormat. detail: {0}")]
    InvalidSuteraFormat(String,String),
    #[error("'{0}' key was not fount in {1}.")]
    KeysNotFound(String,String),
    #[error("This 3D object type '{0}' is not yet implemented.")]
    InvalidObjectType(String),
    #[error("This 3D object type '{0}' is None in {1}.")]
    NoneObjectType(String,String),
    #[error("Failed to load yaml file. Please check file. Reason: {0}")]
    SerdeYamlLoading(String),
    #[error("Failed to opening yaml file. Please check file ({1}). Reason: {0}")]
    YamlFileOpen(String,String),
    #[error("Failed to opening glTF file in {1}. Please check file. Reason: {0}")]
    GltfFileOpen(String,String),
    #[error("Failed to generate 3d model. Please check file or settings. Reason: {0}")]
    Generate3DModel(String),
}
