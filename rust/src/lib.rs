pub mod world;
use godot::prelude::*;

//create godot entrypoint
struct SuteraExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SuteraExtension{}


//this node is interface among rust and Godot.
#[derive(GodotClass)]
#[class(base=Node)]
struct SuteraWorldLoader{
    base: Base<Node>
}

#[godot_api]
impl INode for SuteraWorldLoader{
    fn init(base: Base<Node>) -> Self{
        let yaml_path = String::from("../godot/world/world.yaml");
        world::yaml_loader::world_loader(yaml_path);
        Self{base}
    }
}
