use super::world_format::SuteraTransformSpecs;
use godot::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SuteraTransform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl SuteraTransform {
    pub fn new(position: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl fmt::Display for SuteraTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "position:{}\nrotation(Quaternion):{},\nscale:{}",
            self.position, self.rotation, self.scale
        )
    }
}

impl From<SuteraTransformSpecs> for SuteraTransform{
    fn from(value: SuteraTransformSpecs) -> Self {
        let position: Vector3 = Vector3::new(value.pos_x,value.pos_y,value.pos_z);
        let rotation: Quaternion = Quaternion::new(value.rot_x,value.rot_y,value.rot_z,value.rot_w);
        let scale: Vector3 = Vector3::new(value.scale_x,value.scale_y,value.scale_z)
        return Self{
            position,
            rotation,
            scale,
        }
    }
}

pub trait SetSuteraTransform {
    fn set_sutera_transform(&mut self, transform: &SuteraTransform);
}

impl SetSuteraTransform for Node3D {
    fn set_sutera_transform(&mut self, transform: &SuteraTransform) {
        self.set_position(transform.position);
        godot_print!("set position:{}", transform.position);
        self.set_quaternion(transform.rotation);
        godot_print!("set rotation:{}", transform.rotation);
        self.set_scale(transform.scale);
        godot_print!("set scale:{}", transform.scale);
    }
}
