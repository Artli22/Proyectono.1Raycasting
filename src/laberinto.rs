use raylib::prelude::Vector2;

use std::fs;

pub struct Laberinto {
    celdas: Vec<Vec<char>>,
    cell_size: f32,
}

impl Laberinto {
    // Cargar el laberinto mediante los txt
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

    // Busca el simbolo '*' en el txt para saber la posicion inicial del jugador 
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

    // Busca las letras de los enemigos en el txt para saber la posicion inicial
    pub fn extraer_enemigos(&mut self, letras: &[char]) -> Vec<(char, Vector2)> {
        let mut resultado = Vec::new();
        for (row, line) in self.celdas.iter_mut().enumerate() {
            for (col, symbol) in line.iter_mut().enumerate() {
                if letras.contains(symbol) {
                    let letra = *symbol;
                    let pos = Vector2::new(
                        col as f32 * self.cell_size + self.cell_size / 2.0,
                        row as f32 * self.cell_size + self.cell_size / 2.0,
                    );
                    *symbol = ' ';
                    resultado.push((letra, pos));
                }
            }
        }
        resultado
    }
    
    // Dibujo de la pared del laberinto
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

        matches!(self.celdas[row][column], '+' | '-' | '|' | 's')
    }

    // Verifica si el jugador esta cerca de una pared para evitar traspasarla, colisiones para la pared
    pub fn posicion_valida_cerca(&self, pos: Vector2) -> Vector2 {
        if !self.es_pared(pos) {
            return pos;
        }
        for n in 1..=20i32 {
            let d = self.cell_size * n as f32;
            let candidatos = [
                Vector2::new(pos.x + d, pos.y),
                Vector2::new(pos.x - d, pos.y),
                Vector2::new(pos.x, pos.y + d),
                Vector2::new(pos.x, pos.y - d),
                Vector2::new(pos.x + d, pos.y + d),
                Vector2::new(pos.x - d, pos.y + d),
                Vector2::new(pos.x + d, pos.y - d),
                Vector2::new(pos.x - d, pos.y - d),
            ];
            for c in candidatos {
                if !self.es_pared(c) {
                    return c;
                }
            }
        }
        pos
    }

    // Verifica si el jugador esta cerca de la salida
    pub fn cerca_de_salida(&self, pos: Vector2, radio: f32) -> bool {
        let min_col = ((pos.x - radio) / self.cell_size).floor().max(0.0) as usize;
        let max_col = ((pos.x + radio) / self.cell_size).ceil() as usize;
        let min_row = ((pos.y - radio) / self.cell_size).floor().max(0.0) as usize;
        let max_row = ((pos.y + radio) / self.cell_size).ceil() as usize;

        for row in min_row..=max_row {
            if let Some(linea) = self.celdas.get(row) {
                for col in min_col..=max_col {
                    if linea.get(col) == Some(&'s') {
                        return true;
                    }
                }
            }
        }
        false
    }
}
