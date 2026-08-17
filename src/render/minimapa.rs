use raylib::prelude::*;

use super::Framebuffer;

impl Framebuffer {
    pub fn draw_minimap(
        &self,
        drawing: &mut RaylibDrawHandle,
        maze: &[Vec<char>],
        player_position: Vector2,
        screen_width: i32,
        screen_height: i32,
    ) {
        let minimap_width = screen_width as f32 / 8.0;
        let minimap_height = screen_height as f32 / 8.0;
        let minimap_margin = 10.0;

        // Posición en la esquina superior derecha
        let minimap_x = screen_width as f32 - minimap_width - minimap_margin;
        let minimap_y = minimap_margin;

        // Fondo del mini mapa
        drawing.draw_rectangle_rec(
            Rectangle::new(minimap_x, minimap_y, minimap_width, minimap_height),
            Color::new(0, 0, 0, 200),
        );

        // Borde del mini mapa
        drawing.draw_rectangle_lines_ex(
            Rectangle::new(minimap_x, minimap_y, minimap_width, minimap_height),
            2.0,
            Color::WHITE,
        );

        // Calcular escala del mini mapa
        let maze_width = maze.get(0).map(|r| r.len()).unwrap_or(0) as f32 * self.cell_size;
        let maze_height = maze.len() as f32 * self.cell_size;

        let scale_x = minimap_width / maze_width;
        let scale_y = minimap_height / maze_height;
        let minimap_scale = scale_x.min(scale_y);

        // Dibujar el laberinto en miniatura
        for (row, line) in maze.iter().enumerate() {
            for (column, symbol) in line.iter().enumerate() {
                let x = minimap_x + column as f32 * self.cell_size * minimap_scale;
                let y = minimap_y + row as f32 * self.cell_size * minimap_scale;

                match symbol {
                    '+' => {
                        drawing.draw_rectangle_rec(
                            Rectangle::new(
                                x,
                                y,
                                self.cell_size * minimap_scale,
                                self.cell_size * minimap_scale,
                            ),
                            Color::BLUE,
                        );
                    }

                    '-' | '|' => {
                        drawing.draw_rectangle_rec(
                            Rectangle::new(
                                x,
                                y,
                                self.cell_size * minimap_scale,
                                self.cell_size * minimap_scale,
                            ),
                            Color::RED,
                        );
                    }

                    '#' => {
                        drawing.draw_rectangle_rec(
                            Rectangle::new(
                                x,
                                y,
                                self.cell_size * minimap_scale,
                                self.cell_size * minimap_scale,
                            ),
                            Color::GREEN,
                        );
                    }

                    '&' => {
                        // Final
                        drawing.draw_rectangle_rec(
                            Rectangle::new(
                                x,
                                y,
                                self.cell_size * minimap_scale,
                                self.cell_size * minimap_scale,
                            ),
                            Color::YELLOW,
                        );
                    }

                    _ => {}
                }
            }
        }

        let player_minimap_x = minimap_x + (player_position.x * minimap_scale);
        let player_minimap_y = minimap_y + (player_position.y * minimap_scale);

        drawing.draw_circle(player_minimap_x as i32, player_minimap_y as i32, 4.0, Color::WHITE);

        drawing.draw_text(
            "&",
            (player_minimap_x - 4.0) as i32,
            (player_minimap_y - 10.0) as i32,
            14,
            Color::WHITE,
        );
    }
}
