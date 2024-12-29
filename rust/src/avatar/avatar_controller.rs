use godot::prelude::*;
use godot::classes::*;

#[derive(GodotClass)]
#[class(base=XrCamera3D)]
pub struct AvatarController {
    base: Base<XrCamera3D>,
    right_controller: Option<Gd<XrController3D>>,
    left_controller: Option<Gd<XrController3D>>,
    xr_origin: Option<Gd<XrOrigin3D>>,
    speed: f32,
}

#[godot_api]
impl IXrCamera3D for AvatarController {
    fn init(base: Base<XrCamera3D>) -> Self {
        Self {
            base,
            right_controller: None,
            left_controller: None,
            xr_origin: None,
            speed: 2.5,
        }
    }

    fn enter_tree(&mut self) {
        godot_print!("enter_tree");
        let Some(xr_origin_parent) = self.base_mut().get_parent() else{
            godot_print!("No parent node found");
            return;
        };
        self.xr_origin = Some(xr_origin_parent.clone().cast::<XrOrigin3D>());

        let children = xr_origin_parent.get_children();

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
            let speed = self.speed;
            let mut xr_origin = self.xr_origin.clone().unwrap();
            xr_origin.translate(Vector3::new(-1.0 * stick_vector.x * speed * delta as f32, 0.0, stick_vector.y * speed * delta as f32));
        });
        let right_controller_clone = self.right_controller.clone();
        right_controller_clone.map(|controller|{
            let stick_vector = controller.get_vector2("primary".into());
            let speed = self.speed;
            let mut xr_origin = self.xr_origin.clone().unwrap();
            xr_origin.rotate_y(-stick_vector.x * speed * delta as f32);
        });
    }
}
