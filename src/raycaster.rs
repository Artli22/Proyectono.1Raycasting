use raylib::prelude::Vector2;

use crate::config::{CELL_SIZE, FIELD_OF_VIEW, NUMBER_OF_RAYS, RAY_STEP_SIZE};
use crate::laberinto::Laberinto;

pub struct RayHit {
    pub point: Vector2,
    pub distance: f32,
    pub angle: f32,
    pub wall_x: f32,
}

// Calcular el rango de vision del jugador para dibujarlo en pantalla
pub fn cast_field_of_view(
    laberinto: &Laberinto,
    player_position: Vector2,
    player_angle: f32,
) -> Vec<RayHit> {
    let mut rays = Vec::with_capacity(NUMBER_OF_RAYS);
    let first_angle = player_angle - FIELD_OF_VIEW / 2.0;

    
    let angle_step = if NUMBER_OF_RAYS > 1 {
        FIELD_OF_VIEW / (NUMBER_OF_RAYS - 1) as f32
    } else {
        0.0
    };

    for ray_index in 0..NUMBER_OF_RAYS {
        let ray_angle = first_angle + ray_index as f32 * angle_step;

        rays.push(cast_ray(laberinto, player_position, ray_angle));
    }

    rays
}

// Calcular la distancia de la pared mas cercana al jugador y el punto de colision con la pared
pub fn cast_ray(laberinto: &Laberinto, start: Vector2, angle: f32) -> RayHit {
    let direction_x = angle.cos();
    let direction_y = angle.sin();

    let mut current_point = start;
    let mut distance: f32 = 0.0;

    let filas = laberinto.filas();

    let maximum_columns = filas.iter().map(|line| line.len()).max().unwrap_or(1);

    
    let maximum_distance = (filas.len() as f32 + maximum_columns as f32) * CELL_SIZE;

    while distance < maximum_distance {
        current_point.x += direction_x * RAY_STEP_SIZE;
        current_point.y += direction_y * RAY_STEP_SIZE;

        distance += RAY_STEP_SIZE;

        if laberinto.es_pared(current_point) {

            let frac_x = current_point.x.rem_euclid(CELL_SIZE) / CELL_SIZE;
            let frac_y = current_point.y.rem_euclid(CELL_SIZE) / CELL_SIZE;

            let dist_x = frac_x.min(1.0 - frac_x);
            let dist_y = frac_y.min(1.0 - frac_y);

            let wall_x = if dist_x < dist_y { frac_y } else { frac_x };

            return RayHit {
                point: current_point,
                distance,
                angle,
                wall_x,
            };
        }
    }

    RayHit {
        point: current_point,
        distance,
        angle,
        wall_x: 0.0,
    }
}

pub fn normalize_angle(mut angle: f32) -> f32 {
    let full_rotation = std::f32::consts::PI * 2.0;

    while angle > std::f32::consts::PI {
        angle -= full_rotation;
    }

    while angle < -std::f32::consts::PI {
        angle += full_rotation;
    }

    angle
}
