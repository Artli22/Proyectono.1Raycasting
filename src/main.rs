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

    if let Some(ruta_mapa) = menus::MenuInicio::mostrar(&mut rl, &thread) {
        app::ejecutar(&mut rl, &thread, &ruta_mapa);
    }
}
