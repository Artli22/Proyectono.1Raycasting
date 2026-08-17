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

        drawing.draw_text(&format!("Modo: {mode}"), 10, 8, 18, Color::WHITE);

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
}
