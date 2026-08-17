use raylib::prelude::*;

use crate::config::{
    CELL_SIZE, FIELD_OF_VIEW, INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH, INPUT_FILE,
    WALL_THICKNESS,
};
use crate::enemigo::Enemigo;
use crate::jugador::Jugador;
use crate::laberinto::Laberinto;
use crate::raycaster::cast_field_of_view;
use crate::render::Framebuffer;

pub fn run() {
    let mut laberinto = match Laberinto::cargar(INPUT_FILE, CELL_SIZE) {
        Ok(laberinto) => laberinto,

        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    /*
     * player_spawn se guarda para colocar al enemigo cerca del inicio.
     */
    let player_spawn = match laberinto.buscar_y_quitar_jugador() {
        Some(position) => position,

        None => {
            eprintln!("No se encontró el símbolo '*' dentro del laberinto.");
            return;
        }
    };

    let mut jugador = Jugador::new(player_spawn);

    /*
     * false = vista superior 2D.
     * true  = vista de estacas.
     */
    let mut first_person_view = false;

    let maze_width = laberinto.ancho();
    let maze_height = laberinto.alto();

    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .title("Laberinto con Raycasting")
        .resizable()
        .build();

    raylib_handle.set_target_fps(60);

    let wall_texture = raylib_handle
        .load_texture(&raylib_thread, "src/assets/TexturaPared.png")
        .expect("No se pudo cargar src/assets/TexturaPared.png");

    let feddy_texture = raylib_handle
        .load_texture(&raylib_thread, "src/assets/Feddy.jpg")
        .expect("No se pudo cargar src/assets/Feddy.jpg");

    let enemigo = Enemigo::new(player_spawn.x + CELL_SIZE * 2.0, player_spawn.y);

    let framebuffer = Framebuffer::new(CELL_SIZE, WALL_THICKNESS);

    while !raylib_handle.window_should_close() {
        /*
         * E cambia entre vista superior 2D y vista de estacas.
         */
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_E) {
            first_person_view = !first_person_view;
        }

        /*
         * F11 cambia entre modo ventana y pantalla completa.
         */
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_F11) {
            raylib_handle.toggle_fullscreen();
        }

        let frame_time = raylib_handle.get_frame_time();

        jugador.actualizar(&raylib_handle, &laberinto, frame_time);

        /*
         * Los rayos se recalculan después de mover o girar al personaje.
         */
        let rays = cast_field_of_view(&laberinto, jugador.posicion, jugador.angulo);

        let screen_width = raylib_handle.get_screen_width();
        let screen_height = raylib_handle.get_screen_height();

        /*
         * Escalado utilizado en la vista 2D.
         *
         * Se deja espacio en la parte superior
         * para mostrar el HUD.
         */
        let top_margin: f32 = 115.0;

        let available_width = screen_width as f32;
        let available_height = (screen_height as f32 - top_margin).max(1.0);

        let scale_x = available_width / maze_width;
        let scale_y = available_height / maze_height;

        /*
         * La escala menor mantiene el laberinto
         * completamente dentro de la ventana.
         */
        let scale = scale_x.min(scale_y);

        let rendered_width = maze_width * scale;
        let rendered_height = maze_height * scale;

        let offset_x = (screen_width as f32 - rendered_width) / 2.0;
        let offset_y = top_margin + (available_height - rendered_height) / 2.0;

        let mut drawing = raylib_handle.begin_drawing(&raylib_thread);

        framebuffer.clear(&mut drawing);

        if first_person_view {
            framebuffer.draw_first_person_view(
                &mut drawing,
                &rays,
                jugador.angulo,
                screen_width,
                screen_height,
                FIELD_OF_VIEW,
                &wall_texture,
            );

            framebuffer.draw_enemies(
                &mut drawing,
                &rays,
                jugador.posicion,
                jugador.angulo,
                screen_width,
                screen_height,
                FIELD_OF_VIEW,
                &[enemigo.posicion],
                &feddy_texture,
            );

            framebuffer.draw_minimap(
                &mut drawing,
                laberinto.filas(),
                jugador.posicion,
                screen_width,
                screen_height,
            );
        } else {
            framebuffer.draw_maze_2d(&mut drawing, laberinto.filas(), scale, offset_x, offset_y);

            framebuffer.draw_rays_2d(&mut drawing, jugador.posicion, &rays, scale, offset_x, offset_y);

            framebuffer.draw_player_2d(
                &mut drawing,
                jugador.posicion,
                jugador.angulo,
                scale,
                offset_x,
                offset_y,
            );
        }

        framebuffer.draw_hud(&mut drawing, first_person_view, jugador.posicion, jugador.angulo);
    }
}
