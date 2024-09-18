use godot::prelude::*;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
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

impl Serialize for SuteraTransform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("transform", 10)?;
        s.serialize_field("position_x", &self.position.x)?;
        s.serialize_field("position_y", &self.position.y)?;
        s.serialize_field("position_z", &self.position.z)?;
        s.serialize_field("rotation_x", &self.rotation.x)?;
        s.serialize_field("rotation_y", &self.rotation.y)?;
        s.serialize_field("rotation_z", &self.rotation.z)?;
        s.serialize_field("rotation_w", &self.rotation.w)?;
        s.serialize_field("position_x", &self.position.x)?;
        s.serialize_field("position_y", &self.position.y)?;
        s.serialize_field("position_z", &self.position.z)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for SuteraTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            PositionX,
            PositionY,
            PositionZ,
            RotationX,
            RotationY,
            RotationZ,
            RotationW,
            ScaleX,
            ScaleY,
            ScaleZ,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`position_x` or `position_y` or `position_z` or `rotation_x` or `rotation_y` or `rotation_z` or `rotation_w` or `scale_x` or `scale_y` or `scale_z`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "position_x" => Ok(Field::PositionX),
                            "position_y" => Ok(Field::PositionY),
                            "position_z" => Ok(Field::PositionZ),
                            "rotation_x" => Ok(Field::RotationX),
                            "rotation_y" => Ok(Field::RotationY),
                            "rotation_z" => Ok(Field::RotationZ),
                            "rotation_w" => Ok(Field::RotationW),
                            "scale_x" => Ok(Field::ScaleX),
                            "scale_y" => Ok(Field::ScaleY),
                            "scale_z" => Ok(Field::ScaleZ),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SuteraTransformVisitor;

        impl<'de> Visitor<'de> for SuteraTransformVisitor {
            type Value = SuteraTransform;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SuteraTransform")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SuteraTransform, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let position_x = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let position_y = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let position_z = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let position: Vector3 = Vector3::new(position_x, position_y, position_z);

                let rotation_x = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let rotation_y = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let rotation_z = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let rotation_w = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let rotation: Quaternion =
                    Quaternion::new(rotation_x, rotation_y, rotation_z, rotation_w);

                let scale_x = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let scale_y = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let scale_z = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let scale: Vector3 = Vector3::new(scale_x, scale_y, scale_z);

                Ok(SuteraTransform::new(position, rotation, scale))
            }

            fn visit_map<V>(self, mut map: V) -> Result<SuteraTransform, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut position_x = None;
                let mut position_y = None;
                let mut position_z = None;
                let mut rotation_x = None;
                let mut rotation_y = None;
                let mut rotation_z = None;
                let mut rotation_w = None;
                let mut scale_x = None;
                let mut scale_y = None;
                let mut scale_z = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::PositionX => {
                            if position_x.is_some() {
                                return Err(de::Error::duplicate_field("position_x"));
                            }
                            position_x = Some(map.next_value()?);
                        }
                        Field::PositionY => {
                            if position_y.is_some() {
                                return Err(de::Error::duplicate_field("position_y"));
                            }
                            position_y = Some(map.next_value()?);
                        }
                        Field::PositionZ => {
                            if position_z.is_some() {
                                return Err(de::Error::duplicate_field("position_z"));
                            }
                            position_z = Some(map.next_value()?);
                        }
                        Field::RotationX => {
                            if rotation_x.is_some() {
                                return Err(de::Error::duplicate_field("rotation_x"));
                            }
                            rotation_x = Some(map.next_value()?);
                        }
                        Field::RotationY => {
                            if rotation_y.is_some() {
                                return Err(de::Error::duplicate_field("rotation_y"));
                            }
                            rotation_y = Some(map.next_value()?);
                        }
                        Field::RotationZ => {
                            if rotation_z.is_some() {
                                return Err(de::Error::duplicate_field("rotation_z"));
                            }
                            rotation_z = Some(map.next_value()?);
                        }
                        Field::RotationW => {
                            if rotation_w.is_some() {
                                return Err(de::Error::duplicate_field("rotation_w"));
                            }
                            rotation_w = Some(map.next_value()?);
                        }
                        Field::ScaleX => {
                            if scale_x.is_some() {
                                return Err(de::Error::duplicate_field("scale_x"));
                            }
                            scale_x = Some(map.next_value()?);
                        }
                        Field::ScaleY => {
                            if scale_y.is_some() {
                                return Err(de::Error::duplicate_field("scale_y"));
                            }
                            scale_y = Some(map.next_value()?);
                        }
                        Field::ScaleZ => {
                            if scale_z.is_some() {
                                return Err(de::Error::duplicate_field("scale_z"));
                            }
                            scale_z = Some(map.next_value()?);
                        }
                    }
                }
                let position_x =
                    position_x.ok_or_else(|| de::Error::missing_field("position_x"))?;
                let position_y =
                    position_y.ok_or_else(|| de::Error::missing_field("position_y"))?;
                let position_z =
                    position_z.ok_or_else(|| de::Error::missing_field("position_z"))?;
                let position: Vector3 = Vector3::new(position_x, position_y, position_z);
                let rotation_x =
                    rotation_x.ok_or_else(|| de::Error::missing_field("rotation_x"))?;
                let rotation_y =
                    rotation_y.ok_or_else(|| de::Error::missing_field("rotation_y"))?;
                let rotation_z =
                    rotation_z.ok_or_else(|| de::Error::missing_field("rotation_z"))?;
                let rotation_w =
                    rotation_w.ok_or_else(|| de::Error::missing_field("rotation_w"))?;
                let rotation: Quaternion =
                    Quaternion::new(rotation_x, rotation_y, rotation_z, rotation_w);
                let scale_x = scale_x.ok_or_else(|| de::Error::missing_field("scale_x"))?;
                let scale_y = scale_y.ok_or_else(|| de::Error::missing_field("scale_y"))?;
                let scale_z = scale_z.ok_or_else(|| de::Error::missing_field("scale_z"))?;
                let scale: Vector3 = Vector3::new(scale_x, scale_y, scale_z);
                Ok(SuteraTransform::new(position, rotation, scale))
            }
        }
        const FIELDS: &[&str] = &[
            "position_x",
            "position_y",
            "position_z",
            "rotation_x",
            "rotation_y",
            "rotation_z",
            "rotation_w",
            "scale_x",
            "scale_y",
            "scale_z",
        ];
        deserializer.deserialize_struct("SuteraTransform", FIELDS, SuteraTransformVisitor)
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
