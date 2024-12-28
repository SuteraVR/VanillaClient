use godot::prelude::*;
use godot::classes::{GltfDocument, GltfState};
use crate::error::SpanErr;
use super::avatar_loader::AvatarLoadingError;
use crate::world::world_format::SpawnPoint;

pub struct AvatarGltfObject{
    doc: Gd<GltfDocument>,
    state: Gd<GltfState>,
}

impl AvatarGltfObject{
    pub fn new(path:&String)->Result<Self,SpanErr<AvatarLoadingError>>{
        let model_state = GltfState::new_gd();
        let mut model_doc = GltfDocument::new_gd();
        let fixed_path = AvatarGltfObject::path_solver(path.to_string());
        godot_print!("fixed_path: {}", fixed_path);
        match model_doc.append_from_file(fixed_path.clone(), model_state.clone()) {
            godot::global::Error::OK => Ok(Self {
                doc: model_doc,
                state: model_state,
            }),
            _ => Err(SpanErr::from(AvatarLoadingError::GltfFileOpen(
                "Incorrect gltf path or this gltf file is broken.".to_string(),
                fixed_path.to_string(),
            ))),
        }
    }

    pub fn spawn_avatar(&mut self, spawn_point:SpawnPoint, parent_node: &mut Gd<Node>)->Result<(),SpanErr<AvatarLoadingError>>{
        let Some(node) = self.doc.generate_scene(self.state.clone()) else {
            return Err(SpanErr::from(AvatarLoadingError::Generate3DModel(
                "Couldn't generate scene from gltf file. Gltf file is probably empty.".to_string(),
            )));
        };
        let node = self.set_object(&node, spawn_point);
        parent_node.add_child(node);
        Ok(())
    }

    pub fn set_object(&self, obj: &Gd<Node>, spawn_point:SpawnPoint) -> Gd<Node> {
        let mut obj_3d = obj.clone().cast::<Node3D>();
        obj_3d.set_position(Vector3::new(spawn_point.x, spawn_point.y, spawn_point.z));
        obj_3d.set_rotation(Vector3::new(spawn_point.pitch, spawn_point.yaw, 0.0));
        obj_3d.upcast::<Node>()
    }

    pub fn path_solver(path: String) -> GString {
        let header: String = String::from("res://");
        let path_str = &format!("{}{}", header, path);
        GString::from(path_str)
    }
}
