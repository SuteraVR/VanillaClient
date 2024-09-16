use super::error::SpanErr;
use super::yaml_loader::WorldLoadingError;
use godot::prelude::*;
use serde_yaml::Value;
use tracing::instrument;
use std::fmt;

pub struct SuteraTransform{
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl SuteraTransform{
    #[instrument(skip_all, name = "transfrom_new", level = "trace")]
    pub fn new(pos_x:f32,pos_y:f32,pos_z:f32,rot_x:f32,rot_y:f32,rot_z:f32,rot_w:f32,sca_x:f32,sca_y:f32,sca_z:f32) -> Self{
        let position = Vector3::new(pos_x,pos_y,pos_z);
        let rotation = Quaternion::new(rot_x,rot_y,rot_z,rot_w);
        let scale = Vector3::new(sca_x,sca_y,sca_z);
        return Self{position,rotation,scale};
    }

    #[instrument(skip_all, name = "transform_from_yaml", level = "trace")]
    pub fn from_yaml(transform_value: &Value, yaml_path: String) -> Result<Self, SpanErr<WorldLoadingError>> {
        let position = Self::get_position(transform_value, yaml_path.clone())?;
        let rotation = Self::get_rotation(transform_value, yaml_path.clone())?;
        let scale = Self::get_scale(transform_value, yaml_path.clone())?;

        Ok(Self{position,rotation,scale})
    }

    #[instrument(skip_all, name = "get_position", level = "trace")]
    fn get_position(transform_value:&Value, yaml_path: String) -> Result<Vector3,SpanErr<WorldLoadingError>>{
        let Some(position_x) = transform_value.get("position_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_x".to_string(),yaml_path)))}; 
        let Some(position_x) = position_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'position_x' is not f32 value".to_string(),yaml_path)))};
        let Some(position_y) = transform_value.get("position_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_y".to_string(),yaml_path)))};
        let Some(position_y) = position_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'position_y' is not f32 value".to_string(),yaml_path)))};
        let Some(position_z) = transform_value.get("position_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("position_z".to_string(),yaml_path)))};
        let Some(position_z) = position_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'position_z' is not f32 value".to_string(),yaml_path)))};

        let position:Vector3 = Vector3::new(position_x as f32,position_y as f32,position_z as f32);
        return Ok(position)
    }

    #[instrument(skip_all, name = "get_rotation", level = "trace")]
    fn get_rotation(transform_value:&Value, yaml_path: String) -> Result<Quaternion,SpanErr<WorldLoadingError>>{
        let Some(rotation_x) = transform_value.get("rotation_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_x".to_string(),yaml_path)))};
        let Some(rotation_x) = rotation_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_x' is not f32 value".to_string(),yaml_path)))};
        let Some(rotation_y) = transform_value.get("rotation_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_y".to_string(),yaml_path)))};
        let Some(rotation_y) = rotation_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_y' is not f32 value".to_string(),yaml_path)))};
        let Some(rotation_z) = transform_value.get("rotation_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_z".to_string(),yaml_path)))};
        let Some(rotation_z) = rotation_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_z' is not f32 value".to_string(),yaml_path)))};
        let Some(rotation_w) = transform_value.get("rotation_w") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("rotation_w".to_string(),yaml_path)))};
        let Some(rotation_w) = rotation_w.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'rotation_w' is not f32 value".to_string(),yaml_path)))};

        let rotation:Quaternion = Quaternion::new(rotation_x as f32,rotation_y as f32,rotation_z as f32,rotation_w as f32);
        return Ok(rotation)
    }

    #[instrument(skip_all, name = "get_scale", level = "trace")]
    fn get_scale(transform_value:&Value, yaml_path: String) -> Result<Vector3,SpanErr<WorldLoadingError>>{
        let Some(scale_x) = transform_value.get("scale_x") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_x".to_string(),yaml_path)))};
        let Some(scale_x) = scale_x.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'scale_x' is not f32 value".to_string(),yaml_path)))};
        let Some(scale_y) = transform_value.get("scale_y") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_y".to_string(),yaml_path)))};
        let Some(scale_y) = scale_y.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'scale_y' is not f32 value".to_string(),yaml_path)))};
        let Some(scale_z) = transform_value.get("scale_z") else{return Err(SpanErr::from(WorldLoadingError::KeysNotFound("scale_z".to_string(),yaml_path)))};
        let Some(scale_z) = scale_z.as_f64() else{return Err(SpanErr::from(WorldLoadingError::InvalidSuteraFormat("'scale_z' is not f32 value".to_string(),yaml_path)))};

        let scale:Vector3 = Vector3::new(scale_x as f32,scale_y as f32,scale_z as f32);
        return Ok(scale)
    }
}

impl fmt::Display for SuteraTransform{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        write!(f, "position:{}\nrotation(Quaternion):{},\nscale:{}",self.position,self.rotation,self.scale)
    }
}

pub trait SetSuteraTransform{
    fn set_sutera_transform(&mut self,transform:&SuteraTransform);
}

impl SetSuteraTransform for Node3D{
    fn set_sutera_transform(&mut self,transform:&SuteraTransform) {
        self.set_position(transform.position);
        godot_print!("set position:{}",transform.position);
        self.set_quaternion(transform.rotation);
        godot_print!("set rotation:{}",transform.rotation);
        self.set_scale(transform.scale);
        godot_print!("set scale:{}",transform.scale);
    }
}
