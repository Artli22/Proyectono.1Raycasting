use raylib::prelude::Vector2;

use crate::config::{CELL_SIZE, ENEMY_RADIUS, VELOCIDAD_ENEMIGO};
use crate::laberinto::Laberinto;

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

    pub fn mover_hacia(&mut self, objetivo: Vector2, frame_time: f32, laberinto: &Laberinto) {
        let dx = objetivo.x - self.posicion.x;
        let dy = objetivo.y - self.posicion.y;
        let distancia = (dx * dx + dy * dy).sqrt();
        if distancia < 1.0 {
            return;
        }
        let dir_x = dx / distancia;
        let dir_y = dy / distancia;
        let paso = VELOCIDAD_ENEMIGO * frame_time;

        // Paso de escape > CELL_SIZE/2 para cruzar la frontera de cualquier celda de pared.
        if laberinto.es_pared(self.posicion) {
            let escape_paso = CELL_SIZE * 0.6;
            let salidas = [(escape_paso, 0.0f32), (-escape_paso, 0.0f32), (0.0f32, escape_paso), (0.0f32, -escape_paso)];
            for &(ex, ey) in &salidas {
                let escape = Vector2::new(self.posicion.x + ex, self.posicion.y + ey);
                if !laberinto.es_pared(escape) {
                    self.posicion = escape;
                    return;
                }
            }
            return;
        }

        let nueva_x = Vector2::new(self.posicion.x + dir_x * paso, self.posicion.y);
        if !laberinto.es_pared(nueva_x) {
            self.posicion.x = nueva_x.x;
        }
        let nueva_y = Vector2::new(self.posicion.x, self.posicion.y + dir_y * paso);
        if !laberinto.es_pared(nueva_y) {
            self.posicion.y = nueva_y.y;
        }
    }
}
