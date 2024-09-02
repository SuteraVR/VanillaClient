use super::error::SpanErr;
use super::gltf::SuteraGltfObject;
use godot::prelude::*;
use serde::Deserialize;
use serde_yaml::{Deserializer, Sequence, Value};
use std::borrow::Cow;
use std::fs::File;
use std::io::prelude::*;
use thiserror::Error;
use tracing::{field::Empty, instrument, Span};

#[instrument(name="load_world", level = "trace")]
pub fn load_world(yaml_path: String, parent_node: &mut Gd<Node>)->Result<(),WorldLoadingError>{
    let mut base_node = parent_node.clone();

    //yamlファイルのpathを開く
    let mut yaml_file = File::open(yaml_path).map_err(|e|{WorldLoadingError::YamlFileOpenError(e.to_string())})?;
    let mut contents = String::new();
    yaml_file.read_to_string(&mut contents).map_err(|e|{WorldLoadingError::YamlFileOpenError(e.to_string())})?; //yamlファイルの内容をString型に

    let value = Value::deserialize(Deserializer::from_str(&contents))  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e|{WorldLoadingError::SerdeYamlLoadingError(e.to_string())})?;

    let sequence = get_sequence(&value)   //objectsのsequenceを取得
        .map_err(|e|{WorldLoadingError::SerdeYamlLoadingError(e.to_string())})?;

    for obj in sequence.iter() {
        let model = match obj.get("model"){
            Some(model) => model,
            None => {return Err(WorldLoadingError::InvalidSuteraFormatErr("model".to_string()))}
        };
        match model.get("type") {
            Some(model_type) if model_type.as_str().unwrap() == "gltf" => {
                let path = match model.get("path").and_then(|p| p.as_str()){
                    Some(path) => path,
                    None => {return Err(WorldLoadingError::InvalidSuteraFormatErr("path".to_string()))},
                };
                let transform = get_transform(&model)?;
                let mut gltf_obj = SuteraGltfObject::new(path.to_string(), transform);
                gltf_obj.generate_model(&mut base_node);
            }
            Some(model_type) => {
                let e = SpanErr::from(WorldLoadingError::InvalidObjectTypeErr(
                    model_type.as_str().unwrap().to_string(),
                ));
                tracing::error!("{}", e.error);
                eprintln!("{}", color_spantrace::colorize(&e.span));
                return Err(e.error);
            }
            None => {return Err(worldformat_error("type").error);},
        }
    }
    Ok(())
}

//yamlから3dモデルの情報のsequence(Vector)を取得
#[instrument(skip_all, name = "get_sequence", level = "trace")]
fn get_sequence(value: &Value) -> Result<Sequence, WorldLoadingError> {
    if let Some(specs) = value.get("specs").clone() {   //specsのデータを取得
        if let Some(value) = specs.get("objects").clone() {
            if let Some(sequence) = value.as_sequence() {
                Ok(sequence.clone())
            } else {
                Err(
                    WorldLoadingError::InvalidSuteraFormatErr("getting sequence".to_string())
                        .into(),
                )
            }
        } else {
            Err(WorldLoadingError::InvalidSuteraFormatErr("objects".to_string()).into())
        }
    } else {
        Err(WorldLoadingError::InvalidSuteraFormatErr("specs".to_string()).into())
    }
}

#[instrument(skip_all, name = "worldformaterror", level = "trace")]
fn worldformat_error<'a, T: Into<Cow<'a, str>>>(key: T)->SpanErr<WorldLoadingError> {
    let key: String = key.into().to_string();
    let e = SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(key.clone()));
    tracing::error!("{}", e.error);
    eprintln!("{}", color_spantrace::colorize(&e.span));
    return e;
}

#[instrument(skip_all, name = "get_transform", level = "trace")]
fn get_transform(model: &Value) -> Result<[f32; 10], WorldLoadingError> {
    let mut transform: [f32; 10] = [0.0; 10];
    if let Some(transform_vec) = model.get("transform").and_then(|s| s.as_sequence()) {
        if transform_vec.len() != 10 {
            Err(WorldLoadingError::InvalidSuteraFormatErr(
                "Object transform need 10 numbers:(x,y,z,rx,ry,rz,rw,sx,sy,sz)".to_string(),
            )
            .into())
        } else {
            for i in 0..10 {
                if let Some(element) = transform_vec[i].as_f64() {
                    transform[i] = element as f32;
                }
            }
            Ok(transform)
        }
    } else {
        Err(
            WorldLoadingError::InvalidSuteraFormatErr("'transform' key was not found.".to_string())
        )
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
}
