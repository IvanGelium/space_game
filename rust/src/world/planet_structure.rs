use crate::clutch::archive::biomes::*;
use rand::prelude::*;
use rand::SeedableRng;

pub struct PlanetStructure {
    pub assignments: Vec<u8>, // Индекс биома для каждого адреса
}

impl PlanetStructure {
    pub fn generate(seed: u32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed as u64); // Нужен крейт rand с функцией SmallRng
        let mut assignments = vec![0; 10 * 5 * 24];

        for l in 0..10 {
            // Слои
            for b in 0..5 {
                // Пояса
                for s in 0..24 {
                    // Сектора
                    let layer = unsafe { std::mem::transmute::<u8, Layer>(l as u8) };
                    let belt = unsafe { std::mem::transmute::<u8, Belt>(b as u8) };

                    let biome_id = Self::pick_biome(layer, belt, &mut rng);
                    let idx = (l * 5 * 24) + (b * 24) + s;
                    assignments[idx] = biome_id;
                }
            }
        }
        Self { assignments }
    }

    fn pick_biome(layer: Layer, belt: Belt, rng: &mut SmallRng) -> u8 {
        let roll = rng.gen_range(0..100);

        match layer {
            Layer::Surface | Layer::Mountains => match belt {
                Belt::Arctic => {
                    if roll < 90 {
                        4
                    } else {
                        3
                    }
                } // Снег 90%, Горы 10%
                Belt::Equator => {
                    if roll < 60 {
                        2
                    } else {
                        1
                    }
                } // Пустыня 60%, Джунгли 40%
                _ => {
                    if roll < 70 {
                        1
                    } else {
                        3
                    }
                } // Лес/Джунгли 70%, Горы 30%
            },
            Layer::Underground => {
                if roll < 80 {
                    5
                } else {
                    1
                } // Обычное подземелье 80%, "Заросшее" 20%
            }
            Layer::DeepUnderground => 7, // Глубокое подземелье (дефолт)
            Layer::Core => 12,           // Ядро
            _ => 0,                      // Воздух/Вакуум
        }
    }

    pub fn get_biome(&self, layer: Layer, belt: Belt, sector: u32) -> u8 {
        let idx = (layer as usize * 5 * 24) + (belt as usize * 24) + sector as usize;
        // Используем get().cloned(), чтобы не упасть при ошибке индекса
        self.assignments.get(idx).cloned().unwrap_or(0)
    }
}
