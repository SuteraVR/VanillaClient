use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use serde_yaml::{Value,Deserializer};
use godot::prelude::*;

pub fn world_loader(yaml_path: String)->Result<(), serde_yaml::Error>{
    let mut f = File::open(yaml_path).expect("yaml file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");
    let value = Value::deserialize(Deserializer::from_str(&contents))?;
    if let Some(specs) = value.get("specs"){
        if let Some(objects) = specs.get("objects"){
            godot_print!("{:?}", objects);
        }
        else{godot_error!("This yaml file is invalid format from Sutera world format.");}
    }
    else{godot_error!("This yaml file is invalid format from Sutera world format.");}
    Ok(())
}
