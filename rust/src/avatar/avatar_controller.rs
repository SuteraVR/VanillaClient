use godot::prelude::*;
use godot::classes::*;

#[derive(GodotClass)]
#[class(base=XrOrigin3D)]
pub struct AvatarController {
    base: Base<XrOrigin3D>,
    right_controller: Option<Gd<XrController3D>>,
    left_controller: Option<Gd<XrController3D>>,
    xr_camera: Option<Gd<XrCamera3D>>,
    speed: f32,
}

#[godot_api]
impl IXrOrigin3D for AvatarController {
    fn init(base: Base<XrOrigin3D>) -> Self {
        Self {
            base,
            right_controller: None,
            left_controller: None,
            xr_camera: None,
            speed: 2.5,
        }
    }

    fn enter_tree(&mut self) {
        let children = self.base_mut().get_children();

        self.right_controller = children.iter_shared().find(|child| {
            child.get_name() == StringName::from("RightController")
        }).map(|child| child.cast::<XrController3D>());

        self.left_controller = children.iter_shared().find(|child| {
            child.get_name() == StringName::from("LeftController")
        }).map(|child| child.cast::<XrController3D>());

        self.xr_camera = children.iter_shared().find(|child| {
            child.get_name() == StringName::from("XrCamera")
        }).map(|child| child.cast::<XrCamera3D>());
    }

    fn process(&mut self, delta: f64) {

        let left_controller_clone = self.left_controller.clone();
        if let Some(controller) = left_controller_clone{
            let stick_vector = controller.get_vector2("primary".into());
            let speed = self.speed;
            let move_vector = Vector2::new(-1.0 * stick_vector.x * speed * delta as f32, stick_vector.y * speed * delta as f32);
            let Some(xr_camera) = self.xr_camera.clone() else{
                godot_error!("No XrCamera found");
                return;
            };
            let global_vector = AvatarController::rotate_matrix(move_vector.x, move_vector.y, xr_camera.get_global_rotation().y);
            self.base_mut().translate(Vector3::new(global_vector.x, 0.0, global_vector.y));
        }

        let right_controller_clone = self.right_controller.clone();
        if let Some(controller) = right_controller_clone{
            let stick_vector = controller.get_vector2("primary".into());
            let speed = self.speed;
            self.base_mut().rotate_y(-stick_vector.x * speed * delta as f32);
        }
    }

}

impl AvatarController{
    fn rotate_matrix(x:f32,y:f32,theta: f32)->Vector2{
        let x = x * theta.cos() - y * theta.sin();
        let y = x * theta.sin() + y * theta.cos();
        Vector2::new(x,y)
    }
}
