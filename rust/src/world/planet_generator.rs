use crate::clutch::config::*;
use crate::core::math_utils::NoiseSettings;
use godot::prelude::*;
use rayon::prelude::*;
pub struct PlanetGenerator {
    pub noise_settings: NoiseSettings,
    pub planet_seed: u32,
    pub radius: f32,
    pub density_map: Vec<f32>,
    pub center: Vector2,
}

impl PlanetGenerator {
    pub fn new(radius: f32) -> Self {
        Self {
            planet_seed: PLANET_SEED,
            radius,
            noise_settings: NoiseSettings::new(PLANET_SEED, 5.0, 5.0),
            density_map: vec![0.0; MAP_SIZE * MAP_SIZE],
            center: Vector2::new(MAP_SIZE as f32 / 2.0, MAP_SIZE as f32 / 2.0),
        }
    }

    // pub fn generate_density(&mut self) {
    //     let map_size = MAP_SIZE;
    //     let center = self.center;
    //     let radius = self.radius;
    //     for y in 0..map_size {
    //         for x in 0..map_size {
    //             let pos = Vector2::new(x as f32, y as f32);
    //             let offset = pos - center;
    //             let mut d = radius - offset.length();
    //             d += self.noise_settings.get_value(pos.x, pos.y);
    //             let index = x + y * map_size;
    //             self.density_map[index] = d;
    //         }
    //     }
    // }

    pub fn generate_density(&mut self) {
        let map_size = MAP_SIZE;
        let radius = self.radius;
        let center_x = self.center.x;
        let center_y = self.center.y;

        // Ссылка на настройки шума, чтобы передать её в замыкание потоков
        let noise = &self.noise_settings;

        // Разрезаем массив density_map на части по размеру строки (map_size)
        // Каждая итерация .for_each теперь обрабатывает целую строку в отдельном потоке
        self.density_map
            .par_chunks_mut(map_size)
            .enumerate()
            .for_each(|(y, row)| {
                let y_f = y as f32;
                let dy = y_f - center_y;
                let dy_sq = dy * dy; // Квадрат расстояния по Y — константа для всей строки

                for x in 0..map_size {
                    let x_f = x as f32;
                    let dx = x_f - center_x;

                    // Оптимизация: считаем гипотенузу без создания Vector2
                    let dist = (dx * dx + dy_sq).sqrt();
                    let mut d = radius - dist;

                    // Добавляем шум (метод get_value уже оптимизирован нами ранее)
                    d += noise.get_value(x_f, y_f);

                    // Записываем напрямую в ячейку строки
                    row[x] = d;
                }
            });
    }
}
