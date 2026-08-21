use raylib::prelude::*;

use crate::audio::Sonidos;
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

// Enrutamiento del sprite del enemigo con su letra asignada en el txt
fn letra_a_textura(letra: char, ruta_mapa: &str) -> &'static str {
    if ruta_mapa.contains("FNAF1") {
        match letra {
            'F' => "src/assets/FNAF1assets/Feddy.jpg",
            'B' => "src/assets/FNAF1assets/bonni.png",
            'C' => "src/assets/FNAF1assets/cica.png",
            'X' => "src/assets/FNAF1assets/fexy.png",
            'G' => "src/assets/FNAF1assets/goldenfeddy.png",
            _ => "src/assets/FNAF1assets/Feddy.jpg",
        }
    } else if ruta_mapa.contains("FNAF2") {
        match letra {
            'T' => "src/assets/FNAF2assets/toyfeddy.png",
            'N' => "src/assets/FNAF2assets/toybonni.png",
            'Y' => "src/assets/FNAF2assets/toycica.png",
            'O' => "src/assets/FNAF2assets/oldfoxy.png",
            'R' => "src/assets/FNAF2assets/ruinfeddy.png",
            'W' => "src/assets/FNAF2assets/oldboni.png",
            'Q' => "src/assets/FNAF2assets/oldcica.png",
            _ => "src/assets/FNAF2assets/toyfeddy.png",
        }
    } else {
        match letra {
            'P' => "src/assets/FNAF3assets/sprintap.png",
            'H' => "src/assets/FNAF3assets/ghostfeddy.png",
            'J' => "src/assets/FNAF3assets/ghostfoxy.png",
            'D' => "src/assets/FNAF3assets/shadowbonni.png",
            _ => "src/assets/FNAF3assets/sprintap.png",
        }
    }
}

// Carga y enrutamiento de todos los assets del juego, sprites, dibujo de las paredes y sonidos; tambien contiene las mecanicas del juego
pub fn ejecutar(rl: &mut RaylibHandle, thread: &RaylibThread, ruta_mapa: &str, ruta_textura_pared: &str, sonidos: &Sonidos<'_>) -> Option<EstadoFinal> {
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

    let ruta_salida = if ruta_mapa.contains("FNAF1") {
        "src/assets/FNAF1assets/salidafnaf1.png"
    } else if ruta_mapa.contains("FNAF2") {
        "src/assets/FNAF2assets/salidafnaf2.png"
    } else {
        "src/assets/FNAF3assets/salidafnaf3.png"
    };
    let salida_texture = rl
        .load_texture(thread, ruta_salida)
        .expect("No se pudo cargar la textura de salida");

    let tex_linterna_apagada = rl
        .load_texture(thread, "src/assets/linterna_apagada.png")
        .expect("No se pudo cargar src/assets/linterna_apagada.png");

    let tex_linterna_encendida = rl
        .load_texture(thread, "src/assets/linterna_encendida.png")
        .expect("No se pudo cargar src/assets/linterna_encendida.png");

    let tex_mano_linterna = rl
        .load_texture(thread, "src/assets/mano_linterna.png")
        .expect("No se pudo cargar src/assets/mano_linterna.png");

    let letras: &[char] = if ruta_mapa.contains("FNAF1") {
        &['F', 'B', 'C', 'X', 'G']
    } else if ruta_mapa.contains("FNAF2") {
        &['T', 'N', 'Y', 'O', 'R', 'W', 'Q']
    } else {
        &['P', 'H', 'J', 'D']
    };
    let spawns = laberinto.extraer_enemigos(letras);
    let mut rutas_tex: Vec<&str> = Vec::new();
    let mut indices_tex: Vec<usize> = Vec::new();
    for (letra, _) in &spawns {
        let ruta = letra_a_textura(*letra, ruta_mapa);
        let idx = rutas_tex.iter().position(|&r| r == ruta).unwrap_or_else(|| {
            let i = rutas_tex.len();
            rutas_tex.push(ruta);
            i
        });
        indices_tex.push(idx);
    }
    let texturas_enemigos: Vec<Texture2D> = rutas_tex.iter()
        .map(|ruta| rl.load_texture(thread, ruta).expect("No se pudo cargar textura de enemigo"))
        .collect();
    let mut enemigos: Vec<(Enemigo, usize)> = spawns.iter().zip(indices_tex.iter())
        .map(|((_, pos), &idx)| (Enemigo::new(pos.x, pos.y), idx))
        .collect();

    let framebuffer = Framebuffer::new(CELL_SIZE, WALL_THICKNESS);
    let mut tiempo_restante: f32 = DURACION_JUEGO_SEGUNDOS;

    let mut ambiente_timer: f32 = 0.0;
    let mut ambiente_idx: usize = 0;
    if !sonidos.ambientes.is_empty() {
        sonidos.ambientes[0].play();
    }

    while !rl.window_should_close() {
        let gp = rl.is_gamepad_available(0);
        if rl.is_key_pressed(KeyboardKey::KEY_E)
            || (gp && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP))
        {
            first_person_view = !first_person_view;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            rl.toggle_fullscreen();
        }

        let frame_time = rl.get_frame_time();

        tiempo_restante = (tiempo_restante - frame_time).max(0.0);
        let salida_desbloqueada = tiempo_restante <= 0.0;
        let enemigos_activos = tiempo_restante <= TIEMPO_INICIO_ENEMIGOS;

        ambiente_timer += frame_time;
        if !sonidos.ambientes.is_empty()
            && ambiente_timer >= 60.0
            && !sonidos.ambientes[ambiente_idx].is_playing()
        {
            ambiente_idx = (ambiente_idx + 1) % sonidos.ambientes.len();
            sonidos.ambientes[ambiente_idx].play();
            ambiente_timer = 0.0;
        }

        jugador.actualizar(rl, &laberinto, frame_time, &sonidos.caminando, &sonidos.linterna);

        for (enemigo, _) in &enemigos {
            if enemigo.colisiona_con(jugador.posicion, PLAYER_RADIUS) {
                return Some(EstadoFinal::GameOver);
            }
        }

        // Mecanica de congelamiento de enemigos por linterna
        for (enemigo, _) in &mut enemigos {
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
        }

        if enemigos_activos {
            for (enemigo, _) in &mut enemigos {
                enemigo.mover_hacia(jugador.posicion, frame_time, &laberinto);
            }
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
                &laberinto,
                jugador.linterna_activa,
            );

            for (enemigo, tex_idx) in &enemigos {
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
                    &texturas_enemigos[*tex_idx],
                );
            }

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
