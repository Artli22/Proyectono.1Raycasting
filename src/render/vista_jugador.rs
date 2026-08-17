use raylib::prelude::*;

use crate::raycaster::RayHit;

use super::Framebuffer;

impl Framebuffer {
    pub fn draw_player_2d(
        &self,
        drawing: &mut RaylibDrawHandle,
        player_position: Vector2,
        player_angle: f32,
        scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        let center = Vector2::new(
            offset_x + player_position.x * scale,
            offset_y + player_position.y * scale,
        );

        let radius = self.cell_size * 0.32 * scale;

        drawing.draw_circle_v(center, radius, self.player_color);

        /*
         * Esta línea muestra hacia dónde
         * está mirando el jugador.
         */
        let direction_length = self.cell_size * 1.2 * scale;

        let direction_end = Vector2::new(
            center.x + player_angle.cos() * direction_length,
            center.y + player_angle.sin() * direction_length,
        );

        drawing.draw_line_ex(center, direction_end, (2.0 * scale).max(1.0), Color::ORANGE);
    }

    pub fn draw_rays_2d(
        &self,
        drawing: &mut RaylibDrawHandle,
        player_position: Vector2,
        rays: &[RayHit],
        scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        let start = Vector2::new(
            offset_x + player_position.x * scale,
            offset_y + player_position.y * scale,
        );

        for ray in rays.iter().step_by(4) {
            let end = Vector2::new(offset_x + ray.point.x * scale, offset_y + ray.point.y * scale);

            drawing.draw_line_ex(start, end, scale.max(1.0), self.ray_color);
        }

        
        if let Some(first_ray) = rays.first() {
            let end = Vector2::new(
                offset_x + first_ray.point.x * scale,
                offset_y + first_ray.point.y * scale,
            );

            drawing.draw_line_ex(start, end, (2.0 * scale).max(1.0), Color::YELLOW);
        }

        if let Some(last_ray) = rays.last() {
            let end = Vector2::new(
                offset_x + last_ray.point.x * scale,
                offset_y + last_ray.point.y * scale,
            );

            drawing.draw_line_ex(start, end, (2.0 * scale).max(1.0), Color::YELLOW);
        }
    }
}
