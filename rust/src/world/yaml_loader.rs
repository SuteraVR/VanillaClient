use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use serde_yaml::{Value,Deserializer,Sequence};
use godot::prelude::*;
use super::gltf::SuteraGltfObject;
use super::error::SpanErr;
use std::option::Option;
use tracing::{instrument,Span,field::Empty};
use thiserror::Error;
use std::borrow::Cow;

#[instrument(skip_all,name="world_loader",level = "trace")]
pub fn world_loader(yaml_path: String)->Result<(), serde_yaml::Error>{
    let mut f = File::open(yaml_path).expect("yaml file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");
    let value = Value::deserialize(Deserializer::from_str(&contents))?;
    let sequence = get_sequence(value).unwrap();
    for obj in sequence.iter(){
        if let Some(model) = obj.get("model"){
            match model.get("type"){
                Some(model_type) if model_type.as_str().unwrap()=="gltf" => {
                    if let Some(path) = model.get("path").and_then(|p| p.as_str()){
                        let transform:[f32;10] = get_transform(&model);
                        SuteraGltfObject::new(path.to_string(),transform);
                    }
                    else{worldformat_error("path");}
                },
                Some(model_type)=>{
                    let e = SpanErr::from(WorldLoadingError::InvalidObjectTypeErr(model_type.as_str().unwrap().to_string()));
                    tracing::error!("{}",e.error);
                    eprintln!("{}",color_spantrace::colorize(&e.span));
                },
                None => worldformat_error("type"),
            }
        }
        else{
            worldformat_error("model");
        }
    }
    Ok(())
}

#[instrument(skip_all,name="get_sequence",level = "trace")]
fn get_sequence(value:Value) -> Result<Sequence,SpanErr<WorldLoadingError>>{
    if let Some(value) = value.get("specs").clone(){
        if let Some(value) = value.get("objects").clone(){
            if let Some(sequence) = value.as_sequence(){
                Ok(sequence.clone())
            }
            else{Err(WorldLoadingError::InvalidSuteraFormatErr("getting sequence".to_string()).into())}
        }
        else{Err(WorldLoadingError::InvalidSuteraFormatErr("objects".to_string()).into())}
    }
    else{Err(WorldLoadingError::InvalidSuteraFormatErr("specs".to_string()).into())}
}

#[instrument(skip_all,name="worldformaterror",level="trace")]
fn worldformat_error<'a, T:Into<Cow<'a, str>>>(key:T){
    let key: String = key.into().to_string();
    let e = SpanErr::from(WorldLoadingError::InvalidSuteraFormatErr(key.clone()));
    tracing::error!("{}",e.error);
    eprintln!("{}",color_spantrace::colorize(&e.span));
}


#[instrument(skip_all,name="get_transform",level = "trace")]
fn get_transform(model:&Value) -> [f32;10]{
    let mut transform:[f32;10] = [0.0;10];
    if let Some(transform_vec) = model.get("transform").and_then(|s| s.as_sequence()){
        if transform_vec.len() != 10{
            return transform;
        }
        else{
            for i in 0..10{
                if let Some(element) = transform_vec[i].as_f64(){
                    transform[i] = element as f32;
                }
            }
            return transform;
        }
    }
    else{return transform;}
}

#[derive(Error,Debug)]
pub enum WorldLoadingError{
    #[error("This yaml file does not conform to SuteraWorldFormat. detail: {0} key was not founded.")]
    InvalidSuteraFormatErr(String),
    #[error("This 3Dobject type '{0}' is not yet implemented.")]
    InvalidObjectTypeErr(String),
    #[error("Failed to load yaml file. Please check file.")]
    SerdeYamlLoadingError,
}
