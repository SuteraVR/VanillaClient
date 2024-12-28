use godot::prelude::*;
use godot::classes::*;

#[derive(GodotClass)]
#[class(base=XrOrigin3D)]
pub struct AvatarController {
    base: Base<XrOrigin3D>,
    right_controller: Option<Gd<XrController3D>>,
    left_controller: Option<Gd<XrController3D>>,
}

#[godot_api]
impl IXrOrigin3D for AvatarController {
    fn init(base: Base<XrOrigin3D>) -> Self {
        Self {
            base,
            right_controller: None,
            left_controller: None,
        }
    }

    fn enter_tree(&mut self) {
        godot_print!("enter_tree");
        let children = self.base_mut().get_children();

        self.right_controller = children.iter_shared().find(|child| {
            child.get_name() == StringName::from("RightController")
        }).map(|child| child.cast::<XrController3D>());

        self.left_controller = children.iter_shared().find(|child| {
            child.get_name() == StringName::from("LeftController")
        }).map(|child| child.cast::<XrController3D>());
    }

    fn process(&mut self, delta: f64) {
        let left_controller_clone = self.left_controller.clone();
        left_controller_clone.map(|controller|{
            let stick_vector = controller.get_vector2("primary".into());

            self.base_mut().translate(Vector3::new(stick_vector.x*delta as f32, 0.0, stick_vector.y*delta as f32));
        });
        let right_controller_clone = self.right_controller.clone();
        right_controller_clone.map(|controller|{
            let stick_vector = controller.get_vector2("primary".into());
            self.base_mut().rotate_y(-stick_vector.x*delta as f32);
        });
    }
}
