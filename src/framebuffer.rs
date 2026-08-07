use raylib::prelude::*;

pub struct RayHit {
    pub point: Vector2,
    pub distance: f32,
    pub angle: f32,
}

pub struct Framebuffer {
    cell_size: f32,
    wall_thickness: f32,

    background_color: Color,
    corner_color: Color,
    wall_color: Color,
    start_color: Color,
    end_color: Color,
    player_color: Color,
    ray_color: Color,
}

impl Framebuffer {
    pub fn new(cell_size: f32, wall_thickness: f32) -> Self {
        Self {
            cell_size,
            wall_thickness,

            background_color: Color::BLACK,
            corner_color: Color::BLUE,
            wall_color: Color::RED,
            start_color: Color::GREEN,
            end_color: Color::YELLOW,
            player_color: Color::WHITE,
            ray_color: Color::new(255, 255, 0, 100),
        }
    }

    pub fn clear(&self, drawing: &mut RaylibDrawHandle) {
        drawing.clear_background(self.background_color);
    }

    pub fn draw_maze_2d(
        &self,
        drawing: &mut RaylibDrawHandle,
        maze: &[Vec<char>],
        scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        for (row, line) in maze.iter().enumerate() {
            for (column, symbol) in line.iter().enumerate() {
                self.draw_symbol(
                    drawing,
                    *symbol,
                    column,
                    row,
                    scale,
                    offset_x,
                    offset_y,
                );
            }
        }
    }

    fn draw_symbol(
        &self,
        drawing: &mut RaylibDrawHandle,
        symbol: char,
        column: usize,
        row: usize,
        scale: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        let x =
            offset_x + column as f32 * self.cell_size * scale;

        let y =
            offset_y + row as f32 * self.cell_size * scale;

        match symbol {
            '+' => {
                self.draw_corner(drawing, x, y, scale);
            }

            '-' => {
                self.draw_horizontal_wall(drawing, x, y, scale);
            }

            '|' => {
                self.draw_vertical_wall(drawing, x, y, scale);
            }

            '#' => {
                self.draw_start(drawing, x, y, scale);
            }

            '&' => {
                self.draw_end(drawing, x, y, scale);
            }

            _ => {}
        }
    }

    fn draw_corner(
        &self,
        drawing: &mut RaylibDrawHandle,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        let size = self.cell_size * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(x, y, size, size),
            self.corner_color,
        );
    }

    fn draw_horizontal_wall(
        &self,
        drawing: &mut RaylibDrawHandle,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        let width = self.cell_size * scale;
        let height = self.wall_thickness * scale;

        let wall_y = y
            + ((self.cell_size - self.wall_thickness) / 2.0)
                * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(
                x,
                wall_y,
                width,
                height,
            ),
            self.wall_color,
        );
    }

    fn draw_vertical_wall(
        &self,
        drawing: &mut RaylibDrawHandle,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        let width = self.wall_thickness * scale;
        let height = self.cell_size * scale;

        let wall_x = x
            + ((self.cell_size - self.wall_thickness) / 2.0)
                * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(
                wall_x,
                y,
                width,
                height,
            ),
            self.wall_color,
        );
    }

    fn draw_start(
        &self,
        drawing: &mut RaylibDrawHandle,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        let margin = scale;
        let size = (self.cell_size - 2.0) * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(
                x + margin,
                y + margin,
                size,
                size,
            ),
            self.start_color,
        );
    }

    fn draw_end(
        &self,
        drawing: &mut RaylibDrawHandle,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        let margin = scale;
        let size = (self.cell_size - 2.0) * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(
                x + margin,
                y + margin,
                size,
                size,
            ),
            self.end_color,
        );
    }

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

        let radius =
            self.cell_size * 0.32 * scale;

        drawing.draw_circle_v(
            center,
            radius,
            self.player_color,
        );

        /*
         * Esta línea muestra hacia dónde
         * está mirando el jugador.
         */
        let direction_length =
            self.cell_size * 1.2 * scale;

        let direction_end = Vector2::new(
            center.x
                + player_angle.cos() * direction_length,
            center.y
                + player_angle.sin() * direction_length,
        );

        drawing.draw_line_ex(
            center,
            direction_end,
            (2.0 * scale).max(1.0),
            Color::ORANGE,
        );
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

        /*
         * Se dibuja uno de cada cuatro rayos
         * para no saturar demasiado la vista 2D.
         */
        for ray in rays.iter().step_by(4) {
            let end = Vector2::new(
                offset_x + ray.point.x * scale,
                offset_y + ray.point.y * scale,
            );

            drawing.draw_line_ex(
                start,
                end,
                scale.max(1.0),
                self.ray_color,
            );
        }

        /*
         * Primer límite del campo visual.
         */
        if let Some(first_ray) = rays.first() {
            let end = Vector2::new(
                offset_x + first_ray.point.x * scale,
                offset_y + first_ray.point.y * scale,
            );

            drawing.draw_line_ex(
                start,
                end,
                (2.0 * scale).max(1.0),
                Color::YELLOW,
            );
        }

        /*
         * Segundo límite del campo visual.
         */
        if let Some(last_ray) = rays.last() {
            let end = Vector2::new(
                offset_x + last_ray.point.x * scale,
                offset_y + last_ray.point.y * scale,
            );

            drawing.draw_line_ex(
                start,
                end,
                (2.0 * scale).max(1.0),
                Color::YELLOW,
            );
        }
    }

    pub fn draw_first_person_view(
        &self,
        drawing: &mut RaylibDrawHandle,
        rays: &[RayHit],
        player_angle: f32,
        screen_width: i32,
        screen_height: i32,
        field_of_view: f32,
    ) {
        let width = screen_width as f32;
        let height = screen_height as f32;

        /*
         * Cielo.
         */
        drawing.draw_rectangle(
            0,
            0,
            screen_width,
            screen_height / 2,
            Color::new(
                30,
                45,
                80,
                255,
            ),
        );

        /*
         * Suelo.
         */
        drawing.draw_rectangle(
            0,
            screen_height / 2,
            screen_width,
            screen_height / 2,
            Color::new(
                40,
                40,
                40,
                255,
            ),
        );

        if rays.is_empty() {
            return;
        }

        /*
         * Plano virtual sobre el que se proyectan
         * las paredes.
         */
        let projection_plane_distance =
            (width / 2.0)
                / (field_of_view / 2.0).tan();

        /*
         * Cada rayo ocupa una franja vertical
         * de la pantalla.
         */
        let stake_width =
            width / rays.len() as f32;

        for (index, ray) in rays.iter().enumerate() {
            /*
             * Corrección del efecto de ojo de pez.
             *
             * Sin esta corrección, las paredes
             * rectas parecerían curvas.
             */
            let angle_difference =
                normalize_angle(
                    ray.angle - player_angle,
                );

            let corrected_distance =
                (
                    ray.distance
                        * angle_difference.cos()
                )
                    .max(0.01);

            /*
             * El tamaño de la pared cambia
             * dinámicamente según su distancia.
             *
             * Cerca  -> estaca alta.
             * Lejos  -> estaca baja.
             */
            let stake_height =
                (
                    self.cell_size
                        * projection_plane_distance
                )
                    / corrected_distance;

            /*
             * Se limita la altura máxima para
             * evitar rectángulos excesivos.
             */
            let visible_height =
                stake_height.min(height * 3.0);

            let stake_x =
                index as f32 * stake_width;

            let stake_y =
                height / 2.0
                    - visible_height / 2.0;

            /*
             * Las paredes lejanas son más oscuras.
             */
            let brightness =
                (
                    255.0
                        / (
                            1.0
                                + corrected_distance
                                    * 0.018
                        )
                )
                    .clamp(
                        45.0,
                        255.0,
                    ) as u8;

            let wall_color = Color::new(
                brightness,
                brightness / 5,
                brightness / 5,
                255,
            );

            drawing.draw_rectangle_rec(
                Rectangle::new(
                    stake_x,
                    stake_y,
                    stake_width + 1.0,
                    visible_height,
                ),
                wall_color,
            );
        }
    }

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

        drawing.draw_rectangle(
            0,
            0,
            660,
            107,
            Color::new(
                0,
                0,
                0,
                180,
            ),
        );

        drawing.draw_text(
            &format!("Modo: {mode}"),
            10,
            8,
            18,
            Color::WHITE,
        );

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

fn normalize_angle(mut angle: f32) -> f32 {
    let full_rotation =
        std::f32::consts::PI * 2.0;

    while angle > std::f32::consts::PI {
        angle -= full_rotation;
    }

    while angle < -std::f32::consts::PI {
        angle += full_rotation;
    }

    angle
}