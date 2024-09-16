use super::error::SpanErr;
use super::yaml_loader::WorldLoadingError;
#[allow(unused_imports)]
use godot::classes::{CollisionObject3D, GltfDocument, GltfState};
use godot::obj::NewGd;
use godot::prelude::*;

/// Loaded 3d model from world yaml file
pub struct SuteraGltfObject {
    doc: Gd<GltfDocument>,
    state: Gd<GltfState>,
    transform: [f32; 10],
    //追加予定：collider: Vec<CollisionObject3D>,
}

impl SuteraGltfObject {
    pub fn new(path: String, transform: [f32; 10]) -> Result<Self, SpanErr<WorldLoadingError>> {
        let model_state = GltfState::new_gd();
        let mut model_doc = GltfDocument::new_gd();
        let fixed_path = SuteraGltfObject::path_solver(path);
        godot_print!("fixed_path: {}", fixed_path);
        match model_doc.append_from_file(fixed_path.clone(), model_state.clone()) {
            godot::global::Error::OK => Ok(Self {
                doc: model_doc,
                state: model_state,
                transform,
                //collider,
            }),
            _ => Err(SpanErr::from(WorldLoadingError::GltfFileOpen(
                "Incorrect gltf path or this gltf file is broken.".to_string(),fixed_path.to_string()
            ))),
        }
    }

    pub fn generate_model(
        &mut self,
        root: &mut Gd<Node>,
    ) -> Result<(), SpanErr<WorldLoadingError>> {
        let Some(node) = self.doc.generate_scene(self.state.clone()) else {return Err(SpanErr::from(WorldLoadingError::Generate3DModel(
            "Couldn't generate scene from gltf file. Gltf file is probably empty.".to_string(),
        )))};
        let node = self.set_object(&node);
        root.add_child(node);
        Ok(())
    }

    pub fn path_solver(path: String) -> GString {
        let header: String = String::from("res://models/");
        let path_str = &format!("{}{}", header, path);
        GString::from(path_str)
    }

    fn set_object(&self, obj: &Gd<Node>) -> Gd<Node> {
        let mut obj_3d = obj.clone().cast::<Node3D>();
        let obj_position: Vector3 =
            Vector3::new(self.transform[0], self.transform[1], self.transform[2]);
        obj_3d.set_position(obj_position);
        godot_print!(
            "set position:{}",obj_position
        );
        let obj_rotation: Quaternion = Quaternion::new(
            self.transform[3],
            self.transform[4],
            self.transform[5],
            self.transform[6],
        );
        obj_3d.set_quaternion(obj_rotation);
        godot_print!(
            "set rotation:{}",obj_rotation
        );
        let obj_scale: Vector3 =
            Vector3::new(self.transform[7], self.transform[8], self.transform[9]);
        obj_3d.set_scale(obj_scale);
        godot_print!(
            "set scale:{}",obj_scale
        );
        obj_3d.upcast::<Node>()
    }
}
