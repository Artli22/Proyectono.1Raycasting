use raylib::prelude::Vector2;

pub struct Enemigo {
    pub posicion: Vector2,
}

impl Enemigo {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            posicion: Vector2::new(x, y),
        }
    }
}
