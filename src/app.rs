use raylib::prelude::*;

use crate::config::{CELL_SIZE, DURACION_JUEGO_SEGUNDOS, FIELD_OF_VIEW, FLASHLIGHT_CONE_HALF, PLAYER_RADIUS, TIEMPO_INICIO_ENEMIGOS, WALL_THICKNESS};
use crate::enemigo::Enemigo;
use crate::jugador::Jugador;
use crate::laberinto::Laberinto;
use crate::raycaster::{cast_field_of_view, cast_ray, normalize_angle};
use crate::render::Framebuffer;

pub enum EstadoFinal {
    GameOver,
    Ganaste,
}

pub fn ejecutar(rl: &mut RaylibHandle, thread: &RaylibThread, ruta_mapa: &str, ruta_textura_pared: &str, ruta_textura_enemigo: &str) -> Option<EstadoFinal> {
    let mut laberinto = match Laberinto::cargar(ruta_mapa, CELL_SIZE) {
        Ok(laberinto) => laberinto,

        Err(error) => {
            eprintln!("{error}");
            return None;
        }
    };

    let player_spawn = match laberinto.buscar_y_quitar_jugador() {
        Some(position) => position,

        None => {
            eprintln!("No se encontró el símbolo '*' dentro del laberinto.");
            return None;
        }
    };

    let mut jugador = Jugador::new(player_spawn);

    let mut first_person_view = false;

    let maze_width = laberinto.ancho();
    let maze_height = laberinto.alto();

    let wall_texture = rl
        .load_texture(thread, ruta_textura_pared)
        .expect("No se pudo cargar la textura de pared");

    let feddy_texture = rl
        .load_texture(thread, ruta_textura_enemigo)
        .expect("No se pudo cargar la textura del enemigo");

    let salida_texture = rl
        .load_texture(thread, "src/assets/salida.png")
        .expect("No se pudo cargar src/assets/salida.png");

    let tex_linterna_apagada = rl
        .load_texture(thread, "src/assets/linterna_apagada.png")
        .expect("No se pudo cargar src/assets/linterna_apagada.png");

    let tex_linterna_encendida = rl
        .load_texture(thread, "src/assets/linterna_encendida.png")
        .expect("No se pudo cargar src/assets/linterna_encendida.png");

    let tex_mano_linterna = rl
        .load_texture(thread, "src/assets/mano_linterna.png")
        .expect("No se pudo cargar src/assets/mano_linterna.png");

    let spawn_enemigo = laberinto.posicion_valida_cerca(
        Vector2::new(player_spawn.x + CELL_SIZE * 2.0, player_spawn.y),
    );
    let mut enemigo = Enemigo::new(spawn_enemigo.x, spawn_enemigo.y);

    let framebuffer = Framebuffer::new(CELL_SIZE, WALL_THICKNESS);
    let mut tiempo_restante: f32 = DURACION_JUEGO_SEGUNDOS;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_E) {
            first_person_view = !first_person_view;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            rl.toggle_fullscreen();
        }

        let frame_time = rl.get_frame_time();

        tiempo_restante = (tiempo_restante - frame_time).max(0.0);
        let salida_desbloqueada = tiempo_restante <= 0.0;
        let enemigos_activos = tiempo_restante <= TIEMPO_INICIO_ENEMIGOS;

        jugador.actualizar(rl, &laberinto, frame_time);

        if enemigo.colisiona_con(jugador.posicion, PLAYER_RADIUS) {
            return Some(EstadoFinal::GameOver);
        }

        // Comprobar si el enemigo está dentro del cono de la linterna con línea de visión libre
        enemigo.congelado = false;
        if jugador.linterna_activa {
            let dx = enemigo.posicion.x - jugador.posicion.x;
            let dy = enemigo.posicion.y - jugador.posicion.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq > 0.1 {
                let angle_to = dy.atan2(dx);
                let diff = normalize_angle(angle_to - jugador.angulo);
                if diff.abs() < FLASHLIGHT_CONE_HALF {
                    let distancia = dist_sq.sqrt();
                    let los = cast_ray(&laberinto, jugador.posicion, angle_to);
                    if los.distance + CELL_SIZE * 0.5 >= distancia {
                        enemigo.congelado = true;
                    }
                }
            }
        }

        if enemigos_activos {
            enemigo.mover_hacia(jugador.posicion, frame_time, &laberinto);
        }

        if salida_desbloqueada && laberinto.cerca_de_salida(jugador.posicion, CELL_SIZE * 1.5) {
            return Some(EstadoFinal::Ganaste);
        }

        let rays = cast_field_of_view(&laberinto, jugador.posicion, jugador.angulo);

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        let top_margin: f32 = 115.0;

        let available_width = screen_width as f32;
        let available_height = (screen_height as f32 - top_margin).max(1.0);

        let scale_x = available_width / maze_width;
        let scale_y = available_height / maze_height;

        let scale = scale_x.min(scale_y);

        let rendered_width = maze_width * scale;
        let rendered_height = maze_height * scale;

        let offset_x = (screen_width as f32 - rendered_width) / 2.0;
        let offset_y = top_margin + (available_height - rendered_height) / 2.0;

        let mut drawing = rl.begin_drawing(thread);

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
                &salida_texture,
                jugador.posicion,
                &laberinto,
                jugador.linterna_activa,
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
                enemigo.congelado,
                &feddy_texture,
            );

            framebuffer.draw_minimap(
                &mut drawing,
                laberinto.filas(),
                jugador.posicion,
                screen_width,
                screen_height,
            );

            framebuffer.draw_linterna(
                &mut drawing,
                jugador.linterna_activa,
                jugador.tiempo_total,
                screen_width,
                screen_height,
                &tex_linterna_apagada,
                &tex_linterna_encendida,
                &tex_mano_linterna,
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
        framebuffer.draw_timer(&mut drawing, tiempo_restante, screen_width, screen_height);
    }

    None
}
