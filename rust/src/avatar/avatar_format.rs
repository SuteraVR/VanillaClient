use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SuteraAvatarYamlFormat{
    pub metadata: AvatarMetadata,
    pub specs: AvatarSpecs,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AvatarMetadata{
    pub avatar_format: String,
    pub avatar_type: String,
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AvatarSpecs{
    pub avatar_model: Avatar3DModel,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Avatar3DModel {
    pub model_type: String,
    pub path: String,
}
