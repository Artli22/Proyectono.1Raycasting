use raylib::prelude::*;

use crate::config::{PLAYER_RADIUS, PLAYER_ROTATION_SPEED, PLAYER_SPEED};
use crate::laberinto::Laberinto;
use crate::raycaster::normalize_angle;

pub struct Jugador {
    pub posicion: Vector2,
    pub angulo: f32,
}

impl Jugador {
    pub fn new(posicion: Vector2) -> Self {
        Self {
            posicion,
            angulo: 0.0,
        }
    }

    /*
     * Movimiento direccional del jugador.
     *
     * W y S mueven según el ángulo actual.
     * A y D modifican el ángulo.
     */
    pub fn actualizar(
        &mut self,
        raylib_handle: &RaylibHandle,
        laberinto: &Laberinto,
        frame_time: f32,
    ) {
        if raylib_handle.is_key_down(KeyboardKey::KEY_A) {
            self.angulo -= PLAYER_ROTATION_SPEED * frame_time;
        }

        if raylib_handle.is_key_down(KeyboardKey::KEY_D) {
            self.angulo += PLAYER_ROTATION_SPEED * frame_time;
        }

        /*
         * Mantiene el ángulo aproximadamente entre
         * -PI y PI.
         */
        self.angulo = normalize_angle(self.angulo);

        /*
         * Dirección hacia la que mira el jugador.
         */
        let forward = Vector2::new(self.angulo.cos(), self.angulo.sin());

        let mut movement_x: f32 = 0.0;
        let mut movement_y: f32 = 0.0;

        if raylib_handle.is_key_down(KeyboardKey::KEY_W) {
            movement_x += forward.x;
            movement_y += forward.y;
        }

        if raylib_handle.is_key_down(KeyboardKey::KEY_S) {
            movement_x -= forward.x;
            movement_y -= forward.y;
        }

        let movement_length = (movement_x * movement_x + movement_y * movement_y).sqrt();

        if movement_length == 0.0 {
            return;
        }

        movement_x /= movement_length;
        movement_y /= movement_length;

        let movement_distance = PLAYER_SPEED * frame_time;

        movement_x *= movement_distance;
        movement_y *= movement_distance;

        /*
         * Se comprueban los ejes por separado para
         * poder deslizarse junto a una pared.
         */
        let proposed_x = Vector2::new(self.posicion.x + movement_x, self.posicion.y);

        if Self::puede_moverse_a(laberinto, proposed_x, PLAYER_RADIUS) {
            self.posicion.x = proposed_x.x;
        }

        let proposed_y = Vector2::new(self.posicion.x, self.posicion.y + movement_y);

        if Self::puede_moverse_a(laberinto, proposed_y, PLAYER_RADIUS) {
            self.posicion.y = proposed_y.y;
        }
    }

    /*
     * Comprueba varios puntos alrededor del jugador
     * para aproximar una colisión circular.
     */
    fn puede_moverse_a(laberinto: &Laberinto, position: Vector2, radius: f32) -> bool {
        /*
         * Aproximación de radius / sqrt(2).
         */
        let diagonal_radius = radius * 0.7071;

        let test_points = [
            Vector2::new(position.x, position.y),
            Vector2::new(position.x - radius, position.y),
            Vector2::new(position.x + radius, position.y),
            Vector2::new(position.x, position.y - radius),
            Vector2::new(position.x, position.y + radius),
            Vector2::new(position.x - diagonal_radius, position.y - diagonal_radius),
            Vector2::new(position.x + diagonal_radius, position.y - diagonal_radius),
            Vector2::new(position.x - diagonal_radius, position.y + diagonal_radius),
            Vector2::new(position.x + diagonal_radius, position.y + diagonal_radius),
        ];

        test_points.iter().all(|point| !laberinto.es_pared(*point))
    }
}
