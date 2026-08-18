use raylib::prelude::*;

use crate::laberinto::Laberinto;
use crate::raycaster::{normalize_angle, RayHit};

use super::Framebuffer;

impl Framebuffer {
    pub fn draw_first_person_view(
        &self,
        drawing: &mut RaylibDrawHandle,
        rays: &[RayHit],
        player_angle: f32,
        screen_width: i32,
        screen_height: i32,
        field_of_view: f32,
        wall_texture: &Texture2D,
        salida_texture: &Texture2D,
        jugador_pos: Vector2,
        laberinto: &Laberinto,
    ) {
        let width = screen_width as f32;
        let height = screen_height as f32;

        /*
         * Cielo.
         */
        drawing.draw_rectangle(0, 0, screen_width, screen_height / 2, Color::new(30, 45, 80, 255));

        // Suelo: color café en celdas 'c', gris oscuro en el resto.
        let floor_proj_dist = (width / 2.0) / (field_of_view / 2.0).tan();
        let cam_height = self.cell_size / 2.0;
        for floor_y in (screen_height / 2)..screen_height {
            let p = (floor_y as f32 - height / 2.0).max(0.1);
            let d = floor_proj_dist * cam_height / p;
            let sample_x = jugador_pos.x + d * player_angle.cos();
            let sample_y = jugador_pos.y + d * player_angle.sin();
            let cell_col = (sample_x / self.cell_size) as i32;
            let cell_row = (sample_y / self.cell_size) as i32;
            let es_cafe = cell_row >= 0
                && cell_col >= 0
                && laberinto.filas().get(cell_row as usize)
                    .and_then(|r| r.get(cell_col as usize))
                    == Some(&'c');
            let floor_color = if es_cafe {
                Color::new(101, 67, 33, 255)
            } else {
                Color::new(40, 40, 40, 255)
            };
            drawing.draw_rectangle(0, floor_y, screen_width, 1, floor_color);
        }

        if rays.is_empty() {
            return;
        }

        /*
         * Plano virtual sobre el que se proyectan
         * las paredes.
         */
        let projection_plane_distance = (width / 2.0) / (field_of_view / 2.0).tan();

        /*
         * Cada rayo ocupa una franja vertical
         * de la pantalla.
         */
        let stake_width = width / rays.len() as f32;

        for (index, ray) in rays.iter().enumerate() {
            /*
             * Corrección del efecto de ojo de pez.
             *
             * Sin esta corrección, las paredes
             * rectas parecerían curvas.
             */
            let angle_difference = normalize_angle(ray.angle - player_angle);

            let corrected_distance = (ray.distance * angle_difference.cos()).max(0.01);

            /*
             * El tamaño de la pared cambia
             * dinámicamente según su distancia.
             *
             * Cerca  -> estaca alta.
             * Lejos  -> estaca baja.
             */
            let stake_height = (self.cell_size * projection_plane_distance) / corrected_distance;

            /*
             * Se limita la altura máxima para
             * evitar rectángulos excesivos.
             */
            let visible_height = stake_height.min(height * 3.0);

            let stake_x = index as f32 * stake_width;
            let stake_y = height / 2.0 - visible_height / 2.0;

            /*
             * Las paredes lejanas son más oscuras.
             */
            let brightness = (255.0 / (1.0 + corrected_distance * 0.018)).clamp(45.0, 255.0) as u8;

            // Celda 's' → textura salida.png estacionaria (no rota hacia el jugador).
            let hit_col = (ray.point.x / self.cell_size) as i32;
            let hit_row = (ray.point.y / self.cell_size) as i32;
            let es_salida = hit_row >= 0
                && hit_col >= 0
                && laberinto.filas().get(hit_row as usize)
                    .and_then(|r| r.get(hit_col as usize))
                    == Some(&'s');

            let tint = Color::new(brightness, brightness, brightness, 255);

            if es_salida {
                let tex_w = salida_texture.width() as f32;
                let tex_h = salida_texture.height() as f32;
                let slice_w = tex_w / rays.len() as f32;
                let tex_x = (ray.wall_x * tex_w).clamp(0.0, tex_w - slice_w);
                drawing.draw_texture_pro(
                    salida_texture,
                    Rectangle::new(tex_x, 0.0, slice_w, tex_h),
                    Rectangle::new(stake_x, stake_y, stake_width + 1.0, visible_height),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    tint,
                );
            } else {
                let tex_w = wall_texture.width() as f32;
                let slice_w = tex_w / rays.len() as f32;
                let tex_x = (ray.wall_x * tex_w).clamp(0.0, tex_w - slice_w);
                drawing.draw_texture_pro(
                    wall_texture,
                    Rectangle::new(tex_x, 0.0, slice_w, wall_texture.height() as f32),
                    Rectangle::new(stake_x, stake_y, stake_width + 1.0, visible_height),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    tint,
                );
            }
        }
    }
}
