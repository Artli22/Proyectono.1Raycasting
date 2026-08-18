use raylib::prelude::Vector2;

use crate::config::ENEMY_RADIUS;

pub struct Enemigo {
    pub posicion: Vector2,
}

impl Enemigo {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            posicion: Vector2::new(x, y),
        }
    }

    pub fn colisiona_con(&self, jugador_pos: Vector2, radio_jugador: f32) -> bool {
        let dx = self.posicion.x - jugador_pos.x;
        let dy = self.posicion.y - jugador_pos.y;
        let radio_suma = ENEMY_RADIUS + radio_jugador;
        dx * dx + dy * dy < radio_suma * radio_suma
    }
}
