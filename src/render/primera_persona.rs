use raylib::prelude::*;

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
    ) {
        let width = screen_width as f32;
        let height = screen_height as f32;

        /*
         * Cielo.
         */
        drawing.draw_rectangle(0, 0, screen_width, screen_height / 2, Color::new(30, 45, 80, 255));

        /*
         * Suelo.
         */
        drawing.draw_rectangle(
            0,
            screen_height / 2,
            screen_width,
            screen_height / 2,
            Color::new(40, 40, 40, 255),
        );

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

            let tint = Color::new(brightness, brightness, brightness, 255);

            let tex_slice_width = wall_texture.width() as f32 / rays.len() as f32;

            let tex_x = (ray.wall_x * wall_texture.width() as f32)
                .clamp(0.0, wall_texture.width() as f32 - tex_slice_width);

            drawing.draw_texture_pro(
                wall_texture,
                Rectangle::new(tex_x, 0.0, tex_slice_width, wall_texture.height() as f32),
                Rectangle::new(stake_x, stake_y, stake_width + 1.0, visible_height),
                Vector2::new(0.0, 0.0),
                0.0,
                tint,
            );
        }
    }
}
