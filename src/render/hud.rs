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
            "E: cambiar vista | F11: pantalla completa | Esc: cerrar",
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
}
