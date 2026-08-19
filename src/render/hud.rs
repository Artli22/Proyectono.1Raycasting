use raylib::prelude::*;

use super::Framebuffer;

impl Framebuffer {
    pub fn draw_hud(
        &self,
        drawing: &mut RaylibDrawHandle,
        first_person_view: bool,
        player_position: Vector2,
        player_angle: f32,
    ) {
        let mode = if first_person_view {
            "Vista de estacas"
        } else {
            "Vista 2D"
        };

        drawing.draw_rectangle(0, 0, 660, 107, Color::new(0, 0, 0, 180));

        drawing.draw_text(&format!("Modo: {mode} | FPS: {}", drawing.get_fps()), 10, 8, 18, Color::WHITE);

        drawing.draw_text(
            "W/S: avanzar y retroceder | A/D: girar",
            10,
            30,
            16,
            Color::WHITE,
        );

        drawing.draw_text(
            "E: cambiar vista | F: linterna | F11: pantalla completa | Esc: cerrar",
            10,
            52,
            16,
            Color::WHITE,
        );

        drawing.draw_text(
            &format!(
                "Posicion: ({:.1}, {:.1}) | Angulo: {:.1} grados",
                player_position.x,
                player_position.y,
                player_angle.to_degrees(),
            ),
            10,
            76,
            16,
            Color::WHITE,
        );
    }

    pub fn draw_timer(
        &self,
        drawing: &mut RaylibDrawHandle,
        tiempo_restante: f32,
        screen_width: i32,
        screen_height: i32,
    ) {
        let minutos = (tiempo_restante / 60.0) as i32;
        let segundos = (tiempo_restante % 60.0) as i32;
        let texto = format!("{minutos}:{segundos:02}");

        let minimap_width = screen_width as f32 / 8.0;
        let minimap_height = screen_height as f32 / 8.0;
        let margin = 10.0;
        let minimap_x = screen_width as f32 - minimap_width - margin;

        let box_w = minimap_width;
        let box_h = minimap_height;
        let box_x = minimap_x - box_w - margin;
        let box_y = margin;

        drawing.draw_rectangle_rec(
            Rectangle::new(box_x, box_y, box_w, box_h),
            Color::new(0, 0, 0, 200),
        );
        drawing.draw_rectangle_lines_ex(
            Rectangle::new(box_x, box_y, box_w, box_h),
            2.0,
            Color::WHITE,
        );

        // Rojo en el último minuto como advertencia.
        let color = if tiempo_restante <= 60.0 { Color::RED } else { Color::WHITE };

        let font_size = (box_h / 2.5) as i32;
        let text_w = drawing.measure_text(&texto, font_size);
        let text_x = (box_x + (box_w - text_w as f32) / 2.0) as i32;
        let text_y = (box_y + (box_h - font_size as f32) / 2.0) as i32;

        drawing.draw_text(&texto, text_x, text_y, font_size, color);
    }

    pub fn draw_linterna(
        &self,
        drawing: &mut RaylibDrawHandle,
        linterna_activa: bool,
        tiempo: f32,
        screen_width: i32,
        screen_height: i32,
        tex_apagada: &Texture2D,
        tex_encendida: &Texture2D,
        tex_mano: &Texture2D,
    ) {
        let sw = screen_width as f32;
        let sh = screen_height as f32;
        let bob = (tiempo * 2.2_f32).sin() * 4.0;

        if linterna_activa {
            let flicker = 1.0_f32 + 0.04 * (tiempo * 14.0).sin();
            let beam_alpha = (85.0_f32 * flicker).clamp(0.0, 118.0) as u8;
            drawing.draw_circle_gradient(
                screen_width / 2,
                screen_height / 2,
                sw.min(sh) * 0.50,
                Color::new(255, 248, 200, beam_alpha),
                Color::new(0, 0, 0, 0),
            );
        }

        {
            let px = 10_i32;
            let py = screen_height - 112;

            drawing.draw_rectangle(px, py, 105, 104, Color::new(0, 0, 0, 195));
            drawing.draw_rectangle_lines(px, py, 105, 104, Color::new(90, 90, 90, 200));

            let label   = if linterna_activa { "LINTERNA: ON " } else { "LINTERNA: OFF" };
            let lbl_col = if linterna_activa { Color::new(255, 232, 70, 230) } else { Color::new(120, 120, 120, 200) };
            drawing.draw_text(label, px + 5, py + 5, 12, lbl_col);

            let tex = if linterna_activa { tex_encendida } else { tex_apagada };
            drawing.draw_texture_pro(
                tex,
                Rectangle::new(0.0, 0.0, tex.width() as f32, tex.height() as f32),
                Rectangle::new((px + 5) as f32, (py + 23) as f32, 95.0, 76.0),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        {
            let dst_w = sw * 0.42;
            let aspect = tex_mano.width() as f32 / tex_mano.height() as f32;
            let dst_h = dst_w / aspect;
            let dst_x = sw * 0.52;
            let dst_y = sh - dst_h + bob;

            drawing.draw_texture_pro(
                tex_mano,
                Rectangle::new(0.0, 0.0, tex_mano.width() as f32, tex_mano.height() as f32),
                Rectangle::new(dst_x, dst_y, dst_w, dst_h),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

    }
}
