mod app;
mod config;
mod enemigo;
mod jugador;
mod laberinto;
mod menus;
mod raycaster;
mod render;

use config::{INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .title("Fight Nights At Freddy's")
        .resizable()
        .build();

    rl.set_target_fps(60);

    loop {
        let seleccion = menus::MenuInicio::mostrar(&mut rl, &thread);

        let Some((ruta_mapa, ruta_textura_pared)) = seleccion else {
            break;
        };

        let jugar_de_nuevo = match app::ejecutar(&mut rl, &thread, &ruta_mapa, &ruta_textura_pared) {
            Some(app::EstadoFinal::GameOver) => menus::MenuGameOver::mostrar(&mut rl, &thread),
            Some(app::EstadoFinal::Ganaste)  => menus::MenuGanaste::mostrar(&mut rl, &thread),
            None => break,
        };

        if !jugar_de_nuevo {
            break;
        }
    }
}
