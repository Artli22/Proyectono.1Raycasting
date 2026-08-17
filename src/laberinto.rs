use raylib::prelude::Vector2;

use std::fs;

pub struct Laberinto {
    celdas: Vec<Vec<char>>,
    cell_size: f32,
}

impl Laberinto {
    pub fn cargar(ruta: &str, cell_size: f32) -> Result<Self, String> {
        let contenido = fs::read_to_string(ruta)
            .map_err(|error| format!("No se pudo leer el archivo '{ruta}': {error}"))?;

        let celdas: Vec<Vec<char>> = contenido
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        if celdas.is_empty() {
            return Err("El archivo del laberinto está vacío.".to_string());
        }

        let columnas = celdas.iter().map(|line| line.len()).max().unwrap_or(0);

        if columnas == 0 {
            return Err("El laberinto no contiene caracteres.".to_string());
        }

        Ok(Self { celdas, cell_size })
    }

    pub fn filas(&self) -> &[Vec<char>] {
        &self.celdas
    }

    pub fn ancho(&self) -> f32 {
        let columnas = self.celdas.iter().map(|line| line.len()).max().unwrap_or(0);

        columnas as f32 * self.cell_size
    }

    pub fn alto(&self) -> f32 {
        self.celdas.len() as f32 * self.cell_size
    }

    /*
     * Busca el símbolo '*' dentro del mapa y lo reemplaza
     * por un espacio porque la posición del jugador es dinámica.
     */
    pub fn buscar_y_quitar_jugador(&mut self) -> Option<Vector2> {
        for (row, line) in self.celdas.iter_mut().enumerate() {
            for (column, symbol) in line.iter_mut().enumerate() {
                if *symbol == '*' {
                    let position = Vector2::new(
                        column as f32 * self.cell_size + self.cell_size / 2.0,
                        row as f32 * self.cell_size + self.cell_size / 2.0,
                    );

                    *symbol = ' ';

                    return Some(position);
                }
            }
        }

        None
    }

    /*
     * Determina si un punto se encuentra dentro
     * de una celda de pared.
     */
    pub fn es_pared(&self, point: Vector2) -> bool {
        if point.x < 0.0 || point.y < 0.0 {
            return true;
        }

        let column = (point.x / self.cell_size).floor() as usize;
        let row = (point.y / self.cell_size).floor() as usize;

        if row >= self.celdas.len() {
            return true;
        }

        if column >= self.celdas[row].len() {
            return true;
        }

        matches!(self.celdas[row][column], '+' | '-' | '|')
    }
}
