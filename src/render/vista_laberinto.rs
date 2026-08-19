use raylib::prelude::*;

use super::Framebuffer;

impl Framebuffer {
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
                self.draw_symbol(drawing, *symbol, column, row, scale, offset_x, offset_y);
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
        let x = offset_x + column as f32 * self.cell_size * scale;
        let y = offset_y + row as f32 * self.cell_size * scale;

        match symbol {
            '+' => self.draw_corner(drawing, x, y, scale),
            '-' => self.draw_horizontal_wall(drawing, x, y, scale),
            '|' => self.draw_vertical_wall(drawing, x, y, scale),
            '#' => self.draw_start(drawing, x, y, scale),
            's' => {
                let size = self.cell_size * scale;
                drawing.draw_rectangle_rec(
                    Rectangle::new(x, y, size, size),
                    Color::new(50, 200, 50, 200),
                );
            }
            _ => {}
        }
    }

    fn draw_corner(&self, drawing: &mut RaylibDrawHandle, x: f32, y: f32, scale: f32) {
        let size = self.cell_size * scale;

        drawing.draw_rectangle_rec(Rectangle::new(x, y, size, size), self.corner_color);
    }

    fn draw_horizontal_wall(&self, drawing: &mut RaylibDrawHandle, x: f32, y: f32, scale: f32) {
        let width = self.cell_size * scale;
        let height = self.wall_thickness * scale;

        let wall_y = y + ((self.cell_size - self.wall_thickness) / 2.0) * scale;

        drawing.draw_rectangle_rec(Rectangle::new(x, wall_y, width, height), self.wall_color);
    }

    fn draw_vertical_wall(&self, drawing: &mut RaylibDrawHandle, x: f32, y: f32, scale: f32) {
        let width = self.wall_thickness * scale;
        let height = self.cell_size * scale;

        let wall_x = x + ((self.cell_size - self.wall_thickness) / 2.0) * scale;

        drawing.draw_rectangle_rec(Rectangle::new(wall_x, y, width, height), self.wall_color);
    }

    fn draw_start(&self, drawing: &mut RaylibDrawHandle, x: f32, y: f32, scale: f32) {
        let margin = scale;
        let size = (self.cell_size - 2.0) * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(x + margin, y + margin, size, size),
            self.start_color,
        );
    }

    fn draw_end(&self, drawing: &mut RaylibDrawHandle, x: f32, y: f32, scale: f32) {
        let margin = scale;
        let size = (self.cell_size - 2.0) * scale;

        drawing.draw_rectangle_rec(
            Rectangle::new(x + margin, y + margin, size, size),
            self.end_color,
        );
    }
}
