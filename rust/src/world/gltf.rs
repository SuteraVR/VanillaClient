use godot::prelude::*;
use godot::classes::{GltfState,GltfDocument,CollisionObject3D};
use godot::obj::NewGd;
use tokio::sync::{oneshot,mpsc};

/// Loaded 3d model from world yaml file
pub struct SuteraGltfObject{
    doc: Gd<GltfDocument>,
    state: Gd<GltfState>,
    transform: [f32;10],
    collider: Vec<CollisionObject3D>,
}

impl SuteraGltfObject{
    pub async fn new(&self,path:String,transform:[f32;10],colliders:Vec<CollisionObject3D>) -> Self {
        let model_state = GltfState::new_gd();
        let mut model_doc = GltfDocument::new_gd();
        let fixed_path = self.path_solver(path);
        let error = model_doc.append_from_file(fixed_path,model_state.clone());
        if error == godot::engine::global::Error::OK{
            Self{
                doc: model_doc,
                state: model_state,
                transform,
                collider: colliders
            }
        }
        //不正なpathやmodelが入ってきたとき
        else{
            panic!("Incorrect yaml or model file. error: {:?}",error); 
        }
    }

    pub async fn generate_model(&mut self, root:&mut Gd<Node>){
        let node = self.doc.generate_scene(self.state.clone());
        match node{
            Some(value) => {
                let value = self.set_object(&mut value).await();
                root.add_child(value.clone());
            },
            None => panic!("Couldn't read glTF file."),
        }
    }

    async fn path_solver(&self,path: String) -> GString{
        let header: String = String::from("res://models/");
        let path_str = &format!("{}{}",header,path);
        GString::from(path_str)
    }

    async fn set_object(self, obj:&mut Gd<Node>)-> Gd<Node>{
        let mut obj_3d = obj.cast::<Node3D>();
        let obj_position:Vector3 = Vector3::new(self.transform[0],self.transform[1],self.transform[2]);
        let obj_rotation:Quaternion = Queternion::new(self.transform[3],self.transform[4],self.transform[5],self.transform[6]);
        let obj_scale:Vector3 = Vector3::new(self.transform[7],self.transform[8],self.transform[9]);
        obj_3d.set_position(obj_rotation);
        obj_3d.set_rotation(obj_rotation);
        obj_3d.set_scale(obj_scale);
        obj_3d.upcast::<Node>()
    }
}
