use godot::prelude::*;
use godot::classes::{GltfState,GltfDocument,CollisionObject3D};
use godot::obj::NewGd;
use tokio::sync::{oneshot,mpsc};

/// Loaded 3d model from world yaml file
pub struct SuteraGltfObject{
    doc: Gd<GltfDocument>,
    state: Gd<GltfState>,
    transform: [f32;10],
    //追加予定：collider: Vec<CollisionObject3D>,
}

impl SuteraGltfObject{
    pub fn new(path:String,transform:[f32;10]) -> Self {
        let model_state = GltfState::new_gd();
        let mut model_doc = GltfDocument::new_gd();
        let fixed_path = SuteraGltfObject::path_solver(path);
        godot_print!("fixed_path: {}",fixed_path);
        let error = model_doc.append_from_file(fixed_path,model_state.clone());
        if error == godot::engine::global::Error::OK{
            Self{
                doc: model_doc,
                state: model_state,
                transform,
                //collider,
            }
        }
        //不正なpathやmodelが入ってきたとき
        else{
            panic!("Incorrect yaml or model file. error: {:?}",error); 
        }
    }

    pub fn generate_model(&mut self, root:&mut Gd<Node>){
        let node = self.doc.generate_scene(self.state.clone());
        match node{
            Some(value) => {
                let value = self.set_object(&value);
                root.add_child(value);
            },
            None => panic!("Couldn't read glTF file."),
        }
    }

    pub fn path_solver(path: String) -> GString{
        let header: String = String::from("res://models/");
        let path_str = &format!("{}{}",header,path);
        GString::from(path_str)
    }

     fn set_object(&self, obj:&Gd<Node>)-> Gd<Node>{
        let mut obj_3d = obj.clone().cast::<Node3D>();
        let obj_position:Vector3 = Vector3::new(self.transform[0],self.transform[1],self.transform[2]);
        godot_print!("set position:({}, {}, {})",self.transform[0],self.transform[1],self.transform[2]);
        let obj_rotation:Quaternion = Quaternion::new(self.transform[3],self.transform[4],self.transform[5],self.transform[6]);
        let obj_scale:Vector3 = Vector3::new(self.transform[7],self.transform[8],self.transform[9]);
        obj_3d.set_position(obj_position);
        obj_3d.set_quaternion(obj_rotation);
        obj_3d.set_scale(obj_scale);
        obj_3d.upcast::<Node>()
    }
}
