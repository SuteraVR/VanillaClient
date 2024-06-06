use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

//This program is a tutorial program to check the operation of gdext-rust.
//See description in (https://godot-rust.github.io/book/intro/hello-world.html)
#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player{
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player{
    fn init(base: Base<Sprite2D>) -> Self{
        godot_print!("Hello, World!");

        Self{
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64){
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
