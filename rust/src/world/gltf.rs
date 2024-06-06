use godot::prelude::*;
use godot::classes::{GltfState,GltfDocument,CollisionObject3D};
use std::path::Path;

pub struct SuteraGltfObject{
    state: Gd<GltfState>,
    transform: Transform3D,
    collider: Vec<CollisionObject3D>,
}

impl SuteraGltfObject{
    pub fn new(&self,path:&Path,transform:[f64;10],colliders:Vec<CollisionObject3D>) -> Self {
        let model_state = GltfState::new_gd();
        let mut model_doc = GltfDocument::new_gd();
        let fixed_path = self.path_solver(&path);
        let error = model_doc.append_from_file(fixed_path,model_state.clone());
        let model_transform = self.array_to_transform(transform);
        if error == godot::engine::global::Error::OK{
            Self{
                state: model_state,
                transform: model_transform,
                collider: colliders
            }
        }
        else{
            panic!();
        }
    }

    fn path_solver(&self,path: &Path) -> GString{
        let path_str: &str;
        GString::from(path_str)
    }

    fn array_to_transform(&self,transform: [f64;10]) -> Transform3D {
        let a = Vector3::new();
        let b = Vector3::new();
        let c = Vector3::new();
        let basis:Basis = {rows:[a,b,c]};
        let origin = Vector3::new();
        Transform3D{basis,origin}
    }
}
