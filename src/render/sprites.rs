use raylib::prelude::*;

use crate::raycaster::{normalize_angle, RayHit};

use super::Framebuffer;

impl Framebuffer {
    pub fn draw_enemies(
        &self,
        drawing: &mut RaylibDrawHandle,
        rays: &[RayHit],
        player_position: Vector2,
        player_angle: f32,
        screen_width: i32,
        screen_height: i32,
        field_of_view: f32,
        sprite_positions: &[Vector2],
        texture: &Texture2D,
    ) {
        if rays.is_empty() || sprite_positions.is_empty() {
            return;
        }

        let width = screen_width as f32;
        let height = screen_height as f32;
        let num_rays = rays.len();

        // Distancia corregida por rayo para z-buffer
        let depth_buffer: Vec<f32> = rays
            .iter()
            .map(|ray| {
                let diff = normalize_angle(ray.angle - player_angle);
                (ray.distance * diff.cos()).max(0.01)
            })
            .collect();

        let projection_plane_distance = (width / 2.0) / (field_of_view / 2.0).tan();

        for &sprite_pos in sprite_positions {
            let dx = sprite_pos.x - player_position.x;
            let dy = sprite_pos.y - player_position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < 0.1 {
                continue;
            }

            let sprite_angle = dy.atan2(dx);
            let angle_diff = normalize_angle(sprite_angle - player_angle);

            // Margen extra para sprites parcialmente visibles en el borde del FOV
            if angle_diff.abs() > field_of_view / 2.0 + 0.3 {
                continue;
            }

            let screen_x = (width / 2.0) + (angle_diff / (field_of_view / 2.0)) * (width / 2.0);

            let sprite_height = (self.cell_size * projection_plane_distance / distance).min(height * 3.0);

            let aspect = texture.width() as f32 / texture.height() as f32;

            let sprite_width = sprite_height * aspect;

            let sprite_top = height / 2.0 - sprite_height / 2.0;
            let sprite_left = screen_x - sprite_width / 2.0;

            let tex_w = texture.width() as f32;
            let tex_h = texture.height() as f32;

            let col_start = (sprite_left as i32).max(0);
            let col_end = ((sprite_left + sprite_width) as i32).min(screen_width - 1);

            for col in col_start..=col_end {
                let ray_col = ((col as f32 / width) * num_rays as f32) as usize;
                let ray_col = ray_col.min(num_rays - 1);

                if depth_buffer[ray_col] <= distance {
                    continue;
                }

                let tex_x = ((col as f32 - sprite_left) / sprite_width * tex_w).clamp(0.0, tex_w - 1.0);

                drawing.draw_texture_pro(
                    texture,
                    Rectangle::new(tex_x, 0.0, 1.0, tex_h),
                    Rectangle::new(col as f32, sprite_top, 1.0, sprite_height),
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
}
