use std::f32::consts::PI;

pub const INPUT_FILE: &str = "laberinto.txt";

pub const CELL_SIZE: f32 = 9.0;
pub const WALL_THICKNESS: f32 = 9.0;

pub const INITIAL_WINDOW_WIDTH: i32 = 1000;
pub const INITIAL_WINDOW_HEIGHT: i32 = 750;

pub const FIELD_OF_VIEW: f32 = PI / 3.0;

pub const NUMBER_OF_RAYS: usize = 960;

pub const RAY_STEP_SIZE: f32 = 0.25;


pub const PLAYER_SPEED: f32 = 35.0;
pub const PLAYER_ROTATION_SPEED: f32 = 2.2;
pub const PLAYER_RADIUS: f32 = 2.5;
pub const ENEMY_RADIUS: f32 = 4.0;

pub const DURACION_JUEGO_SEGUNDOS: f32 = 360.0; 
pub const TIEMPO_INICIO_ENEMIGOS: f32 = 300.0;   
pub const VELOCIDAD_ENEMIGO: f32 = 20.0;
pub const FLASHLIGHT_CONE_HALF: f32 = PI / 8.0; 
