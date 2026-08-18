use raylib::prelude::*;

const ASCII_ART: &[&str] = &[
    r"  __   __           __        ___       ",
    r"  \ \ / /__  _   _  \ \      / (_)_ __  ",
    r"   \ V / _ \| | | |  \ \ /\ / /| | '_ \ ",
    r"    | | (_) | |_| |   \ V  V / | | | | |",
    r"    |_|\___/ \__,_|    \_/\_/  |_|_| |_|",
    r"                                        ",
];

// Devuelve true si el jugador elige "Jugar de nuevo".
pub fn mostrar(rl: &mut RaylibHandle, thread: &RaylibThread) -> bool {
    let opciones = ["Jugar de nuevo", "Salir"];
    let mut seleccion: usize = 0;

    let imagen = rl
        .load_texture(thread, "src/assets/FNAF2assets/toyboni.png")
        .expect("No se pudo cargar toyboni.png");

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            seleccion = (seleccion + 1) % opciones.len();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            seleccion = if seleccion == 0 { opciones.len() - 1 } else { seleccion - 1 };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            return seleccion == 0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return false;
        }

        let frame_time = rl.get_frame_time();
        let _ = frame_time;

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK);

        let art_font_size = 12;
        let art_line_height = 15;
        let char_width = 7;

        let art_total_height = ASCII_ART.len() as i32 * art_line_height;
        let img_h = 150i32;
        let total_height = art_total_height + 20 + img_h + 20 + opciones.len() as i32 * 50;
        let art_start_y = ((screen_height - total_height) / 2).max(10);

        let max_chars = ASCII_ART.iter().map(|l| l.chars().count()).max().unwrap_or(0) as i32;
        let art_x = ((screen_width - max_chars * char_width) / 2).max(0);

        dibujar_ascii(&mut d, art_x, art_start_y, art_font_size, char_width, art_line_height);

        let img_y = art_start_y + art_total_height + 20;
        let img_w = (imagen.width() as f32 * img_h as f32 / imagen.height().max(1) as f32) as i32;
        let img_x = (screen_width - img_w) / 2;
        d.draw_texture_pro(
            &imagen,
            Rectangle::new(0.0, 0.0, imagen.width() as f32, imagen.height() as f32),
            Rectangle::new(img_x as f32, img_y as f32, img_w as f32, img_h as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        let menu_start_y = img_y + img_h + 20;

        for (i, opcion) in opciones.iter().enumerate() {
            let seleccionado = i == seleccion;
            let color = if seleccionado { Color::YELLOW } else { Color::WHITE };
            let prefijo = if seleccionado { "> " } else { "  " };
            let texto = format!("{prefijo}{opcion}");
            let tw = d.measure_text(&texto, 30);
            d.draw_text(&texto, (screen_width - tw) / 2, menu_start_y + i as i32 * 50, 30, color);
        }

        dibujar_hint(&mut d, screen_width, screen_height, "Enter: seleccionar  |  Flechas: navegar  |  Esc: salir");
    }

    false
}

fn dibujar_ascii(
    d: &mut RaylibDrawHandle,
    start_x: i32,
    start_y: i32,
    font_size: i32,
    char_width: i32,
    line_height: i32,
) {
    for (fila, linea) in ASCII_ART.iter().enumerate() {
        let y = start_y + fila as i32 * line_height;
        for (columna, caracter) in linea.chars().enumerate() {
            if caracter == ' ' {
                continue;
            }
            let x = start_x + columna as i32 * char_width;
            d.draw_text(&caracter.to_string(), x, y, font_size, Color::RED);
        }
    }
}

fn dibujar_hint(d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32, texto: &str) {
    let tw = d.measure_text(texto, 18);
    d.draw_text(texto, (screen_width - tw) / 2, screen_height - 40, 18, Color::GRAY);
}
