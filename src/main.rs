mod framebuffer;

use framebuffer::{
    Framebuffer,
    RayHit,
};

use raylib::prelude::*;

use std::f32::consts::PI;
use std::fs;

const INPUT_FILE: &str = "laberinto.txt";

const CELL_SIZE: f32 = 9.0;
const WALL_THICKNESS: f32 = 9.0;

const INITIAL_WINDOW_WIDTH: i32 = 1000;
const INITIAL_WINDOW_HEIGHT: i32 = 750;

/*
 * Campo visual de 60 grados.
 *
 * PI radianes equivalen a 180 grados.
 * PI / 3 equivale a 60 grados.
 */
const FIELD_OF_VIEW: f32 = PI / 3.0;

/*
 * Cada rayo genera una estaca vertical
 * en la vista en primera persona.
 */
const NUMBER_OF_RAYS: usize = 240;

/*
 * Distancia que avanza el rayo en cada
 * comprobación.
 */
const RAY_STEP_SIZE: f32 = 0.25;

/*
 * Configuración del jugador.
 */
const PLAYER_SPEED: f32 = 35.0;
const PLAYER_ROTATION_SPEED: f32 = 2.2;
const PLAYER_RADIUS: f32 = 2.5;

fn main() {
    let maze_text =
        match fs::read_to_string(INPUT_FILE) {
            Ok(content) => content,

            Err(error) => {
                eprintln!(
                    "No se pudo leer el archivo '{}': {}",
                    INPUT_FILE,
                    error,
                );

                return;
            }
        };

    let mut maze: Vec<Vec<char>> =
        maze_text
            .lines()
            .map(|line| line.chars().collect())
            .collect();

    if maze.is_empty() {
        eprintln!(
            "El archivo del laberinto está vacío."
        );

        return;
    }

    let rows =
        maze.len();

    let columns =
        maze
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);

    if columns == 0 {
        eprintln!(
            "El laberinto no contiene caracteres."
        );

        return;
    }

    /*
     * Se busca el símbolo '*' dentro del mapa.
     *
     * Después se reemplaza por un espacio porque
     * la posición del jugador será dinámica.
     */
    let mut player_position =
        match find_and_remove_player(
            &mut maze,
            CELL_SIZE,
        ) {
            Some(position) => position,

            None => {
                eprintln!(
                    "No se encontró el símbolo '*' dentro del laberinto."
                );

                return;
            }
        };

    /*
     * El jugador comienza mirando hacia
     * la derecha.
     */
    let mut player_angle: f32 =
        0.0;

    /*
     * false = vista superior 2D.
     * true  = vista de estacas.
     */
    let mut first_person_view =
        false;

    let maze_width =
        columns as f32 * CELL_SIZE;

    let maze_height =
        rows as f32 * CELL_SIZE;

    let (
        mut raylib_handle,
        raylib_thread,
    ) =
        raylib::init()
            .size(
                INITIAL_WINDOW_WIDTH,
                INITIAL_WINDOW_HEIGHT,
            )
            .title(
                "Laberinto con Raycasting",
            )
            .resizable()
            .build();

    raylib_handle.set_target_fps(60);

    let framebuffer =
        Framebuffer::new(
            CELL_SIZE,
            WALL_THICKNESS,
        );

    while !raylib_handle.window_should_close() {
        /*
         * E cambia entre:
         *
         * - Vista superior 2D.
         * - Vista de paredes mediante estacas.
         */
        if raylib_handle
            .is_key_pressed(
                KeyboardKey::KEY_E,
            )
        {
            first_person_view =
                !first_person_view;
        }

        /*
         * F11 cambia entre modo ventana
         * y pantalla completa.
         */
        if raylib_handle
            .is_key_pressed(
                KeyboardKey::KEY_F11,
            )
        {
            raylib_handle.toggle_fullscreen();
        }

        let frame_time: f32 =
            raylib_handle.get_frame_time();

        update_player(
            &raylib_handle,
            &maze,
            &mut player_position,
            &mut player_angle,
            frame_time,
        );

        /*
         * Los rayos se recalculan después
         * de mover o girar al personaje.
         */
        let rays =
            cast_field_of_view(
                &maze,
                player_position,
                player_angle,
            );

        let screen_width =
            raylib_handle.get_screen_width();

        let screen_height =
            raylib_handle.get_screen_height();

        /*
         * Escalado utilizado en la vista 2D.
         *
         * Se deja espacio en la parte superior
         * para mostrar el HUD.
         */
        let top_margin: f32 =
            115.0;

        let available_width =
            screen_width as f32;

        let available_height =
            (
                screen_height as f32
                    - top_margin
            )
                .max(1.0);

        let scale_x =
            available_width / maze_width;

        let scale_y =
            available_height / maze_height;

        /*
         * La escala menor mantiene el laberinto
         * completamente dentro de la ventana.
         */
        let scale =
            scale_x.min(scale_y);

        let rendered_width =
            maze_width * scale;

        let rendered_height =
            maze_height * scale;

        let offset_x =
            (
                screen_width as f32
                    - rendered_width
            )
                / 2.0;

        let offset_y =
            top_margin
                + (
                    available_height
                        - rendered_height
                )
                    / 2.0;

        let mut drawing =
            raylib_handle.begin_drawing(
                &raylib_thread,
            );

        framebuffer.clear(
            &mut drawing,
        );

        if first_person_view {
            framebuffer.draw_first_person_view(
                &mut drawing,
                &rays,
                player_angle,
                screen_width,
                screen_height,
                FIELD_OF_VIEW,
            );
        } else {
            framebuffer.draw_maze_2d(
                &mut drawing,
                &maze,
                scale,
                offset_x,
                offset_y,
            );

            framebuffer.draw_rays_2d(
                &mut drawing,
                player_position,
                &rays,
                scale,
                offset_x,
                offset_y,
            );

            framebuffer.draw_player_2d(
                &mut drawing,
                player_position,
                player_angle,
                scale,
                offset_x,
                offset_y,
            );
        }

        framebuffer.draw_hud(
            &mut drawing,
            first_person_view,
            player_position,
            player_angle,
        );
    }
}

/*
 * Busca la posición inicial marcada con '*'.
 */
fn find_and_remove_player(
    maze: &mut [Vec<char>],
    cell_size: f32,
) -> Option<Vector2> {
    for (row, line) in maze.iter_mut().enumerate() {
        for (column, symbol) in line.iter_mut().enumerate() {
            if *symbol == '*' {
                /*
                 * El jugador se coloca en el
                 * centro de la celda.
                 */
                let position =
                    Vector2::new(
                        column as f32
                            * cell_size
                            + cell_size / 2.0,

                        row as f32
                            * cell_size
                            + cell_size / 2.0,
                    );

                /*
                 * El símbolo se elimina del mapa
                 * porque ahora se dibuja aparte.
                 */
                *symbol = ' ';

                return Some(position);
            }
        }
    }

    None
}

/*
 * Movimiento direccional del jugador.
 *
 * W y S mueven según el ángulo actual.
 * A y D modifican el ángulo.
 */
fn update_player(
    raylib_handle: &RaylibHandle,
    maze: &[Vec<char>],
    player_position: &mut Vector2,
    player_angle: &mut f32,
    frame_time: f32,
) {
    /*
     * A gira hacia la izquierda.
     */
    if raylib_handle
        .is_key_down(
            KeyboardKey::KEY_A,
        )
    {
        *player_angle -=
            PLAYER_ROTATION_SPEED
                * frame_time;
    }

    /*
     * D gira hacia la derecha.
     */
    if raylib_handle
        .is_key_down(
            KeyboardKey::KEY_D,
        )
    {
        *player_angle +=
            PLAYER_ROTATION_SPEED
                * frame_time;
    }

    /*
     * Mantiene el ángulo aproximadamente entre
     * -PI y PI.
     */
    *player_angle =
        normalize_angle(
            *player_angle,
        );

    /*
     * Dirección hacia la que mira el jugador.
     *
     * Debido a que se utiliza coseno y seno,
     * puede avanzar en cualquier ángulo.
     */
    let forward =
        Vector2::new(
            player_angle.cos(),
            player_angle.sin(),
        );

    let mut movement_x: f32 =
        0.0;

    let mut movement_y: f32 =
        0.0;

    /*
     * W avanza hacia donde mira el jugador.
     */
    if raylib_handle
        .is_key_down(
            KeyboardKey::KEY_W,
        )
    {
        movement_x += forward.x;
        movement_y += forward.y;
    }

    /*
     * S retrocede sin cambiar la dirección
     * de la cámara.
     */
    if raylib_handle
        .is_key_down(
            KeyboardKey::KEY_S,
        )
    {
        movement_x -= forward.x;
        movement_y -= forward.y;
    }

    let movement_length: f32 =
        (
            movement_x * movement_x
                + movement_y * movement_y
        )
            .sqrt();

    /*
     * El jugador puede girar aunque no
     * esté avanzando.
     */
    if movement_length == 0.0 {
        return;
    }

    /*
     * Normaliza el vector para mantener
     * constante la velocidad.
     */
    movement_x /=
        movement_length;

    movement_y /=
        movement_length;

    let movement_distance: f32 =
        PLAYER_SPEED * frame_time;

    movement_x *=
        movement_distance;

    movement_y *=
        movement_distance;

    /*
     * Se comprueba el movimiento sobre X
     * independientemente.
     */
    let proposed_x =
        Vector2::new(
            player_position.x
                + movement_x,

            player_position.y,
        );

    if can_player_move_to(
        maze,
        proposed_x,
        PLAYER_RADIUS,
    ) {
        player_position.x =
            proposed_x.x;
    }

    /*
     * Después se comprueba el movimiento
     * sobre Y.
     *
     * Separar los ejes permite deslizarse
     * junto a una pared.
     */
    let proposed_y =
        Vector2::new(
            player_position.x,

            player_position.y
                + movement_y,
        );

    if can_player_move_to(
        maze,
        proposed_y,
        PLAYER_RADIUS,
    ) {
        player_position.y =
            proposed_y.y;
    }
}

/*
 * Comprueba varios puntos alrededor del jugador
 * para aproximar una colisión circular.
 */
fn can_player_move_to(
    maze: &[Vec<char>],
    position: Vector2,
    radius: f32,
) -> bool {
    /*
     * Aproximación de radius / sqrt(2).
     */
    let diagonal_radius =
        radius * 0.7071;

    let test_points = [
        /*
         * Centro.
         */
        Vector2::new(
            position.x,
            position.y,
        ),

        /*
         * Puntos cardinales.
         */
        Vector2::new(
            position.x - radius,
            position.y,
        ),

        Vector2::new(
            position.x + radius,
            position.y,
        ),

        Vector2::new(
            position.x,
            position.y - radius,
        ),

        Vector2::new(
            position.x,
            position.y + radius,
        ),

        /*
         * Puntos diagonales.
         */
        Vector2::new(
            position.x - diagonal_radius,
            position.y - diagonal_radius,
        ),

        Vector2::new(
            position.x + diagonal_radius,
            position.y - diagonal_radius,
        ),

        Vector2::new(
            position.x - diagonal_radius,
            position.y + diagonal_radius,
        ),

        Vector2::new(
            position.x + diagonal_radius,
            position.y + diagonal_radius,
        ),
    ];

    test_points
        .iter()
        .all(|point| !is_wall(maze, *point))
}

/*
 * Determina si un punto se encuentra dentro
 * de una celda de pared.
 */
fn is_wall(
    maze: &[Vec<char>],
    point: Vector2,
) -> bool {
    /*
     * Salir del mapa se considera una colisión.
     */
    if point.x < 0.0
        || point.y < 0.0
    {
        return true;
    }

    let column =
        (
            point.x
                / CELL_SIZE
        )
            .floor()
            as usize;

    let row =
        (
            point.y
                / CELL_SIZE
        )
            .floor()
            as usize;

    if row >= maze.len() {
        return true;
    }

    if column >= maze[row].len() {
        return true;
    }

    matches!(
        maze[row][column],
        '+' | '-' | '|'
    )
}

/*
 * Genera todos los rayos correspondientes
 * al campo visual de 60 grados.
 */
fn cast_field_of_view(
    maze: &[Vec<char>],
    player_position: Vector2,
    player_angle: f32,
) -> Vec<RayHit> {
    let mut rays =
        Vec::with_capacity(
            NUMBER_OF_RAYS,
        );

    /*
     * El primer rayo se encuentra 30 grados
     * a la izquierda del centro.
     */
    let first_angle: f32 =
        player_angle
            - FIELD_OF_VIEW / 2.0;

    /*
     * Distribución uniforme de los rayos.
     */
    let angle_step: f32 =
        if NUMBER_OF_RAYS > 1 {
            FIELD_OF_VIEW
                / (
                    NUMBER_OF_RAYS - 1
                ) as f32
        } else {
            0.0
        };

    for ray_index in 0..NUMBER_OF_RAYS {
        let ray_angle =
            first_angle
                + ray_index as f32
                    * angle_step;

        let ray =
            cast_ray(
                maze,
                player_position,
                ray_angle,
            );

        rays.push(ray);
    }

    rays
}

/*
 * Lanza un único rayo hasta encontrar
 * una pared.
 */
fn cast_ray(
    maze: &[Vec<char>],
    start: Vector2,
    angle: f32,
) -> RayHit {
    /*
     * Vector de dirección del rayo.
     */
    let direction_x: f32 =
        angle.cos();

    let direction_y: f32 =
        angle.sin();

    let mut current_point =
        start;

    let mut distance: f32 =
        0.0;

    let maximum_columns =
        maze
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(1);

    /*
     * Distancia máxima de seguridad.
     */
    let maximum_distance: f32 =
        (
            maze.len() as f32
                + maximum_columns as f32
        )
            * CELL_SIZE;

    while distance < maximum_distance {
        current_point.x +=
            direction_x * RAY_STEP_SIZE;

        current_point.y +=
            direction_y * RAY_STEP_SIZE;

        distance +=
            RAY_STEP_SIZE;

        if is_wall(
            maze,
            current_point,
        ) {
            return RayHit {
                point: current_point,
                distance,
                angle,
            };
        }
    }

    RayHit {
        point: current_point,
        distance,
        angle,
    }
}

/*
 * Mantiene el ángulo dentro de una vuelta.
 */
fn normalize_angle(
    mut angle: f32,
) -> f32 {
    let full_rotation: f32 =
        PI * 2.0;

    while angle > PI {
        angle -= full_rotation;
    }

    while angle < -PI {
        angle += full_rotation;
    }

    angle
}