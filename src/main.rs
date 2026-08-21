mod app;
mod audio;
mod config;
mod enemigo;
mod jugador;
mod laberinto;
mod menus;
mod raycaster;
mod render;

use config::{INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH};
use raylib::audio::RaylibAudio;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .title("Fight Nights At Freddy's")
        .resizable()
        .build();

    rl.set_target_fps(60);

    let audio = RaylibAudio::init_audio_device().expect("No se pudo inicializar el dispositivo de audio");
    let sonidos = audio::Sonidos::cargar(&audio);

    loop {
        let seleccion = menus::MenuInicio::mostrar(&mut rl, &thread, &sonidos.linterna);

        let Some((ruta_mapa, ruta_textura_pared)) = seleccion else {
            break;
        };

        let jugar_de_nuevo = match app::ejecutar(&mut rl, &thread, &ruta_mapa, &ruta_textura_pared, &sonidos) {
            Some(app::EstadoFinal::GameOver) => menus::MenuGameOver::mostrar(&mut rl, &thread, &sonidos.linterna),
            Some(app::EstadoFinal::Ganaste)  => menus::MenuGanaste::mostrar(&mut rl, &thread, &sonidos.linterna),
            None => break,
        };

        if !jugar_de_nuevo {
            break;
        }
    }
}
