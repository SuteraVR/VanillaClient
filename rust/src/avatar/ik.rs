use godot::prelude::*;
use godot::classes::*;

#[derive(GodotClass)]
#[class(init,tool,base=SkeletonIk3d)]
struct SuteraIKNode{
    base: Base<SkeletonIk3d>,
}

#[godot_api]
impl ISkeletonIk3d for SuteraIKNode{
    fn ready(&mut self){
        self.base_mut().start();
    }
}
