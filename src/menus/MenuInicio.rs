use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum PantallaMenu {
    Principal,
    Niveles,
}

const ASCII_ART: &[&str] = &[
    r"  _____ _       _     _     _   _ _       _     _            _   _     _____             _     _       _      ",
    r" |  ___(_) __ _| |__ | |_  | \ | (_) __ _| |__ | |_ ___     / \ | |_  |  ___| __ ___  __| | __| |_   _( )___  ",
    r" | |_  | |/ _` | '_ \| __| |  \| | |/ _` | '_ \| __/ __|   / _ \| __| | |_ | '__/ _ \/ _` |/ _` | | | |// __| ",
    r" |  _| | | (_| | | | | |_  | |\  | | (_| | | | | |_\__ \  / ___ \ |_  |  _|| | |  __/ (_| | (_| | |_| | \__ \ ",
    r" |_|   |_|\__, |_| |_|\__| |_| \_|_|\__, |_| |_|\__|___/ /_/   \_\__| |_|  |_|  \___|\__,_|\__,_|\__, | |___/ ",
    r"  _____ __|___/____ _____ _____ ____|___/__ _____ _____ _____ _____ _____ _____ _____ _____ _____|___/_ _____ ",
    r" |_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|",
    r"                         __      ____ _| | | _| |_| |__  _ __ ___  _   _  __ _| |__                           ",
    r"                         \ \ /\ / / _` | | |/ / __| '_ \| '__/ _ \| | | |/ _` | '_ \                          ",
    r"                          \ V  V / (_| | |   <| |_| | | | | | (_) | |_| | (_| | | | |                         ",
    r"                           \_/\_/ \__,_|_|_|\_\___|_| |_|_|  \___/ \__,_|\__, |_| |_|                         ",
    r"                                                                         |___/                                ",
    r"                                                                                                              ",
];

pub fn mostrar(rl: &mut RaylibHandle, thread: &RaylibThread) -> Option<(String, String, String)> {
    let mut pantalla = PantallaMenu::Principal;

    let opciones_principal = ["Jugar", "Salir"];
    let opciones_niveles = ["FNAF 1", "FNAF 2", "FNAF 3"];

    let mut seleccion: usize = 0;

    while !rl.window_should_close() {
        let total = match pantalla {
            PantallaMenu::Principal => opciones_principal.len(),
            PantallaMenu::Niveles => opciones_niveles.len(),
        };

        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            seleccion = (seleccion + 1) % total;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            seleccion = if seleccion == 0 {
                total - 1
            } else {
                seleccion - 1
            };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            match pantalla {
                PantallaMenu::Principal => return None,

                PantallaMenu::Niveles => {
                    pantalla = PantallaMenu::Principal;
                    seleccion = 0;
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            match pantalla {
                PantallaMenu::Principal => match seleccion {
                    0 => {
                        pantalla = PantallaMenu::Niveles;
                        seleccion = 0;
                    }

                    _ => return None,
                },

                PantallaMenu::Niveles => {
                    match seleccion {
                        0 => return Some((
                            "src/niveles/FNAF1.txt".to_string(),
                            "src/assets/FNAF1assets/fnaf1wallpaper.png".to_string(),
                            "src/assets/FNAF1assets/Feddy.jpg".to_string(),
                        )),
                        1 => return Some((
                            "src/niveles/FNAF2.txt".to_string(),
                            "src/assets/FNAF2assets/TexturaPared.png".to_string(),
                            "src/assets/FNAF2assets/toyfeddy.png".to_string(),
                        )),
                        _ => return Some((
                            "src/niveles/FNAF3.txt".to_string(),
                            "src/assets/FNAF3assets/fnaf3wall.jpg".to_string(),
                            "src/assets/FNAF3assets/sprintap.png".to_string(),
                        )),
                    }
                }
            }
        }

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK);

        let art_font_size = 12;
        let art_line_height = 15;
        let char_width = 7;

        let art_total_height = ASCII_ART.len() as i32 * art_line_height;

        let num_opciones = match pantalla {
            PantallaMenu::Principal => opciones_principal.len() as i32,
            PantallaMenu::Niveles => opciones_niveles.len() as i32,
        };
        let titulo_extra = if pantalla == PantallaMenu::Niveles { 56 } else { 0 };
        let total_height = art_total_height + 50 + titulo_extra + num_opciones * 50;
        let art_start_y = ((screen_height - total_height) / 2).max(10);

        let max_chars = ASCII_ART
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as i32;

        let art_width = max_chars * char_width;

        let art_x = ((screen_width - art_width) / 2).max(0);

        dibujar_ascii(
            &mut d,
            art_x,
            art_start_y,
            art_font_size,
            char_width,
            art_line_height,
        );

        let menu_start_y = art_start_y + art_total_height + 50;

        match pantalla {
            PantallaMenu::Principal => {
                for (i, opcion) in opciones_principal.iter().enumerate() {
                    let seleccionado = i == seleccion;

                    let color = if seleccionado {
                        Color::YELLOW
                    } else {
                        Color::WHITE
                    };

                    let prefijo = if seleccionado {
                        "> "
                    } else {
                        "  "
                    };

                    let texto = format!("{prefijo}{opcion}");

                    let tw = d.measure_text(&texto, 30);

                    d.draw_text(
                        &texto,
                        (screen_width - tw) / 2,
                        menu_start_y + i as i32 * 50,
                        30,
                        color,
                    );
                }

                dibujar_hint(
                    &mut d,
                    screen_width,
                    screen_height,
                    "Flechas: navegar  |  Enter: seleccionar  |  Esc: salir",
                );
            }

            PantallaMenu::Niveles => {
                let titulo = "Niveles";

                let tw = d.measure_text(titulo, 34);

                d.draw_text(
                    titulo,
                    (screen_width - tw) / 2,
                    menu_start_y - 46,
                    34,
                    Color::ORANGE,
                );

                for (i, opcion) in opciones_niveles.iter().enumerate() {
                    let seleccionado = i == seleccion;

                    let color = if seleccionado { Color::YELLOW } else { Color::WHITE };
                    let prefijo = if seleccionado { "> " } else { "  " };
                    let texto = format!("{prefijo}{opcion}");

                    let tw = d.measure_text(&texto, 30);

                    d.draw_text(
                        &texto,
                        (screen_width - tw) / 2,
                        menu_start_y + i as i32 * 50,
                        30,
                        color,
                    );
                }

                dibujar_hint(
                    &mut d,
                    screen_width,
                    screen_height,
                    "Flechas: navegar  |  Enter: seleccionar  |  Esc: volver",
                );
            }
        }
    }

    None
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

        let color = match fila {
            0..=4 => Color::WHITE, // Fight Nights At Freddy's
            5..=6 => Color::RED,   // Separador
            7..=11 => Color::BLUE, // Walkthrough
            _ => Color::WHITE,
        };

        for (columna, caracter) in linea.chars().enumerate() {
            if caracter == ' ' {
                continue;
            }

            let x = start_x + columna as i32 * char_width;

            let texto = caracter.to_string();

            d.draw_text(
                &texto,
                x,
                y,
                font_size,
                color,
            );
        }
    }
}

fn dibujar_hint(
    d: &mut RaylibDrawHandle,
    screen_width: i32,
    screen_height: i32,
    texto: &str,
) {
    let tw = d.measure_text(texto, 18);

    d.draw_text(
        texto,
        (screen_width - tw) / 2,
        screen_height - 40,
        18,
        Color::GRAY,
    );
}