use crate::error::SpanErr;
use super::world_gltf::WorldGltfObject;
use crate::transform::SuteraTransform;
use super::world_format::SuteraWorldYamlFormat;
use godot::classes::*;
use godot::prelude::*;
use tracing::instrument;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

//ノードの定義
#[derive(GodotClass)]
#[class(init,base=Node)]
struct SuteraWorldLoader {
    base: Base<Node>,
    #[export]
    yaml_path: GString,
}

#[godot_api]
impl INode for SuteraWorldLoader {
    fn enter_tree(&mut self) {
        godot_print!("enter_tree");
        
        //------------ワールド読み込み------------
        let Ok(world_data) =  load_world_yaml(self.yaml_path.to_string()).map_err(|e| {
            tracing::error!("{}", e.error);
        }) else {
            return;
        };

        let Ok(_) = load_world(&world_data, &mut self.base_mut()).map_err(|e| {
            tracing::error!("{}", e.error);
        })else{
            return;
        };
        //------------ワールド読み込み終了------------
        
    }
}

#[instrument(name = "load_world", level = "trace")]
pub fn load_world(
    world: &SuteraWorldYamlFormat,
    parent_node: &mut Gd<Node>,
) -> Result<(), SpanErr<WorldLoadingError>> {
    let mut base_node = parent_node.clone();

    for obj_data in world.specs.objects.iter() {
        let transform = SuteraTransform::from(obj_data.model.transform.clone());
        let mut obj = WorldGltfObject::new(&obj_data.model.path, transform)?;
        let _ = obj.generate_model(&mut base_node);
    }

    Ok(())
}

#[instrument(name = "load_world", level = "trace")]
pub fn load_world_yaml(
    yaml_path: String,
) -> Result<SuteraWorldYamlFormat, SpanErr<WorldLoadingError>> {

    //yamlファイルのpathを開く
    let mut yaml_file = File::open(&yaml_path)
        .map_err(|e| SpanErr::from(WorldLoadingError::InvalidYamlPath(e.to_string())))?;
    let mut contents = String::new();
    yaml_file.read_to_string(&mut contents).map_err(|e| {
        SpanErr::from(WorldLoadingError::YamlFileOpen(
            e.to_string(),
            yaml_path.clone(),
        ))
    })?; //yamlファイルの内容をString型に

    let world:SuteraWorldYamlFormat = serde_yaml::from_str(&contents)  //yamlファイルをデシリアライズしてserdeのValue型(Enum型)を得る
        .map_err(|e| SpanErr::from(WorldLoadingError::SerdeYamlLoading(e.to_string())))?;
    Ok(world)
}

#[derive(Error, Debug)]
pub enum WorldLoadingError {
    #[error("This world file path ({0}) is invalid")]
    InvalidYamlPath(String),
    #[error("This yaml file ({1}) does not conform to SuteraWorldFormat. detail: {0}")]
    InvalidSuteraFormat(String, String),
    #[error("'{0}' key was not fount in {1}.")]
    KeysNotFound(String, String),
    #[error("This 3D object type '{0}' is not yet implemented.")]
    InvalidObjectType(String),
    #[error("This 3D object type '{0}' is None in {1}.")]
    NoneObjectType(String, String),
    #[error("Failed to load yaml file. Please check file. Reason: {0}")]
    SerdeYamlLoading(String),
    #[error("Failed to opening yaml file. Please check file ({1}). Reason: {0}")]
    YamlFileOpen(String, String),
    #[error("Failed to opening glTF file in {1}. Please check file. Reason: {0}")]
    GltfFileOpen(String, String),
    #[error("Failed to generate 3d model. Please check file or settings. Reason: {0}")]
    Generate3DModel(String),
}
