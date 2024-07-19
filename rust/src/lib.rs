pub mod world;
use godot::prelude::*;
use godot::engine::*;

//create godot entrypoint
struct SuteraExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SuteraExtension{}


//this node is interface among rust and Godot.
#[derive(GodotClass)]
#[class(base=Node)]
struct SuteraWorldLoader{
    once: u32,
    base: Base<Node>
}

#[godot_api]
impl INode for SuteraWorldLoader{
    fn init(base: Base<Node>) -> Self{
        let once = 0;
        Self{once,base}
    }
    
    fn input(&mut self, event: Gd<InputEvent>){
        if event.get_class() == GString::from("InputEventKey")&&self.once==0{
            let yaml_path = String::from("../godot/models/world/world.yaml");
            let _ = world::yaml_loader::world_loader(yaml_path,&mut self.base_mut()).expect("Yaml failed");
            self.once += 1;
        }
    }
}
