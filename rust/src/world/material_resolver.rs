use crate::clutch::archive::biomes::*;
use crate::world::planet_structure::PlanetStructure;
use crate::world::world_utils::get_structure_index;

pub const MAT_AIR: u8 = 0;
pub const MAT_STONE: u8 = 1;
pub const MAT_DIRT: u8 = 2;
pub const MAT_GRASS: u8 = 3;
pub const MAT_SAND: u8 = 4;
pub const MAT_SNOW: u8 = 5;
pub const MAT_CORE: u8 = 6;

pub struct MaterialResolver {
    pub planet_radius: f32,
    pub atmos_height: f32,
}

impl MaterialResolver {
    pub fn new(atmos_height: f32, planet_radius: f32) -> Self {
        Self {
            atmos_height,
            planet_radius,
        }
    }

    pub fn resolve(&self, angle: f32, depth: f32, structure: &PlanetStructure) -> u8 {
        // 1. Получаем адрес ячейки
        let (layer, belt, sector) = self.get_address(angle, depth);

        // 2. Узнаем, какой биом назначен этому сектору
        let biome_id = structure.get_biome(layer, belt, sector);

        // 3. Выбираем материал
        match layer {
            // Слои, где биом не важен (глубокие недра и космос)
            Layer::Vacuum => MAT_AIR,
            Layer::AtmosHigh | Layer::AtmosLow => MAT_AIR, // Позже тут будет газ
            Layer::Core => MAT_CORE,
            Layer::Lava => 11,   // MAT_LAVA (добавь в константы)
            Layer::Mantle => 10, // MAT_MANTLE

            // Слои, зависящие от биома
            Layer::Surface | Layer::Mountains | Layer::Underground | Layer::DeepUnderground => {
                self.resolve_by_biome(biome_id, layer, depth)
            }
        }
    }

    fn resolve_by_biome(&self, biome_id: u8, layer: Layer, depth: f32) -> u8 {
        match biome_id {
            1 => {
                // JUNGLE / FOREST
                if layer == Layer::Surface && depth > 0.99 {
                    return MAT_GRASS;
                }
                MAT_DIRT
            }
            2 => {
                // DESERT
                if depth > 0.97 {
                    return MAT_SAND;
                }
                MAT_STONE
            }
            3 => {
                // MOUNTAINS
                if depth > 0.99 {
                    return MAT_SNOW;
                }
                MAT_STONE
            }
            4 => {
                // SNOW / ARCTIC
                if depth > 0.95 {
                    return MAT_SNOW;
                }
                MAT_STONE
            }
            5 => {
                // CAVES (Default Underground)
                MAT_STONE
            }
            _ => MAT_STONE, // Дефолт
        }
    }

    pub fn get_address(&self, angle: f32, depth: f32) -> (Layer, Belt, u32) {
        // 1. ОПРЕДЕЛЯЕМ СЛОЙ (Layer)
        // depth: 0.0 (центр) .. 1.0 (поверхность) .. >1.0 (атмосфера)
        let layer = match depth {
            d if d > 1.2 => Layer::Vacuum,
            d if d > 1.1 => Layer::AtmosHigh,
            d if d > 1.0 => Layer::AtmosLow,
            d if d > 0.98 => Layer::Surface,
            d if d > 0.90 => Layer::Mountains, // Горы могут быть "вложены" в поверхность
            d if d > 0.70 => Layer::Underground,
            d if d > 0.50 => Layer::DeepUnderground,
            d if d > 0.30 => Layer::Mantle,
            d if d > 0.10 => Layer::Lava,
            _ => Layer::Core,
        };

        // 2. ОПРЕДЕЛЯЕМ КЛИМАТИЧЕСКИЙ ПОЯС (Belt)
        // Преобразуем угол в градусы для наглядности (от -180 до 180)
        let angle_deg = angle.to_degrees();

        // Используем абсолютное значение, так как северный и южный пояса симметричны
        let abs_deg = angle_deg.abs();

        let belt = match abs_deg {
            d if d > 75.0 => Belt::Arctic,    // 75..90 (и -75..-90)
            d if d > 55.0 => Belt::Tundra,    // 55..75
            d if d > 30.0 => Belt::Temperate, // 30..55
            d if d > 10.0 => Belt::Tropics,   // 10..30
            _ => Belt::Equator,               // 0..10
        };

        // 3. ОПРЕДЕЛЯЕМ ИНДЕКС СЕКТОРА (Sector Index)
        // Нормализуем -180..180 в 0..1, затем умножаем на количество секторов
        let normalized_angle = (angle_deg + 180.0) / 360.0;
        let sector_idx = (normalized_angle * SECTOR_COUNT as f32).floor() as u32;

        // Ограничиваем индекс, чтобы избежать ошибок округления (0..23)
        let sector_idx = sector_idx.clamp(0, SECTOR_COUNT - 1);

        (layer, belt, sector_idx)
    }
}
