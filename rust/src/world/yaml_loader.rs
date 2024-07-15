use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use serde_yaml::{Value,Deserializer,Sequence};
use godot::prelude::*;
use super::gltf::SuteraGltfObject;
use std::option::Option;
use tracing::{instrument,Span,field::Empty};
use thiserror::Error;

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
                },
                Some(model_type)=>{
                    godot_error!("This type is not yet implemented.")
                },
                None => godot_error!("Object's type value is invalid")
            }
        }
    }
    Ok(())
}

#[instrument(skip_all,name="get_sequence",level = "trace")]
fn get_sequence(value:Value) -> Option<Sequence>{
    value.get("specs")?.get("objects")?.as_sequence().cloned()
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
enum worldLoadingError{
    #[error("Calling {0} is not allowed!")]
    CallingThisNameIsNotAllowed(String),
}
