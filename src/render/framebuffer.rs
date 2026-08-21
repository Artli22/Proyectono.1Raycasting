use raylib::prelude::*;

// Archivo padre que dicta a todo archivo de renderizacion en pantalla 
pub struct Framebuffer {
    pub(super) cell_size: f32,
    pub(super) wall_thickness: f32,

    pub(super) background_color: Color,
    pub(super) corner_color: Color,
    pub(super) wall_color: Color,
    pub(super) start_color: Color,
    pub(super) end_color: Color,
    pub(super) player_color: Color,
    pub(super) ray_color: Color,
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
}
