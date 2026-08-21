use raylib::audio::{RaylibAudio, Sound};

pub struct Sonidos<'aud> {
    pub caminando: Sound<'aud>,
    pub linterna: Sound<'aud>,
    pub ambientes: Vec<Sound<'aud>>,
}

impl<'aud> Sonidos<'aud> {
    pub fn cargar(audio: &'aud RaylibAudio) -> Self {
        let caminando = audio
            .new_sound("src/assets/audio/caminando.mp3")
            .expect("No se pudo cargar caminando.mp3");
        caminando.set_volume(0.65);

        let linterna = audio
            .new_sound("src/assets/audio/linterna.wav")
            .expect("No se pudo cargar linterna.wav");
        linterna.set_volume(0.5);

        let rutas_ambiente = [
            "src/assets/audio/ambiente1.wav",
            "src/assets/audio/ambiente2.wav",
            "src/assets/audio/ambiente3.mp3",
        ];
        let mut ambientes = Vec::new();
        for ruta in &rutas_ambiente {
            let s = audio.new_sound(ruta).expect("No se pudo cargar sonido ambiente");
            s.set_volume(0.4);
            ambientes.push(s);
        }

        Self { caminando, linterna, ambientes }
    }
}
