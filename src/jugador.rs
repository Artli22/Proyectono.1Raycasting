use raylib::prelude::*;

use crate::config::{PLAYER_RADIUS, PLAYER_ROTATION_SPEED, PLAYER_SPEED};
use crate::laberinto::Laberinto;
use crate::raycaster::normalize_angle;

pub struct Jugador {
    pub posicion: Vector2,
    pub angulo: f32,
    pub linterna_activa: bool,
    pub tiempo_total: f32,
}

impl Jugador {
    pub fn new(posicion: Vector2) -> Self {
        Self {
            posicion,
            angulo: 0.0,
            linterna_activa: false,
            tiempo_total: 0.0,
        }
    }

   
    pub fn actualizar(
        &mut self,
        raylib_handle: &RaylibHandle,
        laberinto: &Laberinto,
        frame_time: f32,
    ) {
        const GAMEPAD: i32 = 0;
        const DEADZONE: f32 = 0.2;

        let gamepad_ok = raylib_handle.is_gamepad_available(GAMEPAD);

        let boton_a = gamepad_ok
            && raylib_handle.is_gamepad_button_pressed(
                GAMEPAD,
                GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
            );
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_F) || boton_a {
            self.linterna_activa = !self.linterna_activa;
        }

        self.tiempo_total += frame_time;

        let gp_rot = if gamepad_ok {
            let v = raylib_handle
                .get_gamepad_axis_movement(GAMEPAD, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
            if v.abs() > DEADZONE { v } else { 0.0 }
        } else {
            0.0
        };

        if raylib_handle.is_key_down(KeyboardKey::KEY_A) {
            self.angulo -= PLAYER_ROTATION_SPEED * frame_time;
        }
        if raylib_handle.is_key_down(KeyboardKey::KEY_D) {
            self.angulo += PLAYER_ROTATION_SPEED * frame_time;
        }
        self.angulo += gp_rot * PLAYER_ROTATION_SPEED * frame_time;
        self.angulo = normalize_angle(self.angulo);

        let forward = Vector2::new(self.angulo.cos(), self.angulo.sin());
        let right = Vector2::new(-self.angulo.sin(), self.angulo.cos());

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

        if gamepad_ok {
            let lx = raylib_handle
                .get_gamepad_axis_movement(GAMEPAD, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
            let ly = raylib_handle
                .get_gamepad_axis_movement(GAMEPAD, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
            let lx = if lx.abs() > DEADZONE { lx } else { 0.0 };
            let ly = if ly.abs() > DEADZONE { ly } else { 0.0 };

            movement_x += forward.x * (-ly) + right.x * lx;
            movement_y += forward.y * (-ly) + right.y * lx;
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

        let proposed_x = Vector2::new(self.posicion.x + movement_x, self.posicion.y);

        if Self::puede_moverse_a(laberinto, proposed_x, PLAYER_RADIUS) {
            self.posicion.x = proposed_x.x;
        }

        let proposed_y = Vector2::new(self.posicion.x, self.posicion.y + movement_y);

        if Self::puede_moverse_a(laberinto, proposed_y, PLAYER_RADIUS) {
            self.posicion.y = proposed_y.y;
        }
    }

    fn puede_moverse_a(laberinto: &Laberinto, position: Vector2, radius: f32) -> bool {
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
