use raylib::prelude::*;

use crate::config::FLASHLIGHT_CONE_HALF;
use crate::laberinto::Laberinto;
use crate::raycaster::{normalize_angle, RayHit};

use super::Framebuffer;

impl Framebuffer {
    // Dibujo de la vista en primera persona del jugador, con iluminacion y texturas
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
        laberinto: &Laberinto,
        linterna_activa: bool,
    ) {
        let width = screen_width as f32;
        let height = screen_height as f32;

        drawing.draw_rectangle(0, 0, screen_width, screen_height / 2, Color::new(40, 40, 40, 255));
        drawing.draw_rectangle(0, screen_height / 2, screen_width, screen_height / 2, Color::new(40, 40, 40, 255));

        if rays.is_empty() {
            return;
        }

        let projection_plane_distance = (width / 2.0) / (field_of_view / 2.0).tan();
        let stake_width = width / rays.len() as f32;

        for (index, ray) in rays.iter().enumerate() {
            let angle_difference = normalize_angle(ray.angle - player_angle);

            let corrected_distance = (ray.distance * angle_difference.cos()).max(0.01);

            let stake_height = (self.cell_size * projection_plane_distance) / corrected_distance;
            let visible_height = stake_height.min(height * 3.0);

            let stake_x = index as f32 * stake_width;
            let stake_y = height / 2.0 - visible_height / 2.0;

            let ambient = (255.0 / (1.0 + corrected_distance * 0.018)).clamp(45.0, 255.0);
            let flashlight_boost = if linterna_activa {
                let ray_offset = normalize_angle(ray.angle - player_angle);
                if ray_offset.abs() < FLASHLIGHT_CONE_HALF {
                    let cone_factor = 1.0 - ray_offset.abs() / FLASHLIGHT_CONE_HALF;
                    let dist_factor = (1.0 / (1.0 + corrected_distance * 0.012)).clamp(0.0, 1.0);
                    cone_factor * dist_factor * 150.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            let brightness = (ambient + flashlight_boost).clamp(0.0, 255.0) as u8;

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
