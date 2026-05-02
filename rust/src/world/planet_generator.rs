use crate::clutch::archive::biomes::*;
use crate::clutch::configs::base_config::*;
use crate::core::math_utils::NoiseSettings;
use crate::world::material_resolver::MaterialResolver;
use crate::world::planet_structure::PlanetStructure;
use crate::world::world_utils::*;
use godot::prelude::*;
// use rayon::prelude::*;

pub struct PlanetGenerator {
    pub noise_settings: NoiseSettings,
    pub material_resolver: MaterialResolver,
    pub planet_seed: u32,
    pub radius: f32,
    pub density_map: Vec<f32>,
    pub material_map: Vec<u8>,
    pub center: Vector2,
    pub planet_structure: PlanetStructure,
}

impl PlanetGenerator {
    pub fn new(radius: f32, atmos_height: f32) -> Self {
        Self {
            planet_seed: PLANET_SEED,
            radius,
            noise_settings: NoiseSettings::new(PLANET_SEED, 5.0, 5.0),
            material_resolver: MaterialResolver::new(radius, atmos_height),
            density_map: vec![0.0; MAP_SIZE * MAP_SIZE],
            material_map: vec![0; MAP_SIZE * MAP_SIZE],
            center: Vector2::new(MAP_SIZE as f32 / 2.0, MAP_SIZE as f32 / 2.0),
            planet_structure: PlanetStructure::generate(PLANET_SEED),
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

        for y in 0..map_size {
            let y_f = y as f32;
            let dy = y_f - center_y;
            let dy_sq = dy * dy;
            let row_offset = y * map_size;

            for x in 0..map_size {
                let x_f = x as f32;
                let dx = x_f - center_x;
                let idx = row_offset + x;
                let (angle, depth) =
                    get_polar_info(x_f, y_f, Vector2::new(center_x, center_y), radius);

                let dist = (dx * dx + dy_sq).sqrt();
                let mut d = radius - dist;

                // Добавляем шум (опционально)
                d += noise.get_value(x_f, y_f);
                self.density_map[idx] = d;
                let is_solid = d > 0.0;
                let mat = self
                    .material_resolver
                    .resolve(angle, depth, &self.planet_structure);
                self.material_map[idx] = mat;
            }
        }
    }

    pub fn get_chunk_mesh(&mut self, chunk_pos: Vector2i) -> Dictionary<StringName, Variant> {
        let mut mesh_data = MeshData::new();
        // let mut temp_vertices = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE * 2);

        let x_start = chunk_pos.x;
        let y_start = chunk_pos.y;

        for y in y_start..(y_start + CHUNK_SIZE as i32) {
            for x in x_start..(x_start + CHUNK_SIZE as i32) {
                if x >= MAP_SIZE as i32 - 1 || y >= MAP_SIZE as i32 - 1 {
                    continue;
                }

                let d0 = self.get_d(x as usize, y as usize);
                let d1 = self.get_d(x as usize + 1, y as usize);
                let d2 = self.get_d(x as usize + 1, y as usize + 1);
                let d3 = self.get_d(x as usize, y as usize + 1);
                let densities = [d0, d1, d2, d3];

                let mut case_index = 0;
                if d0 > 0.0 {
                    case_index |= 1;
                }
                if d1 > 0.0 {
                    case_index |= 2;
                }
                if d2 > 0.0 {
                    case_index |= 4;
                }
                if d3 > 0.0 {
                    case_index |= 8;
                }

                if case_index > 0 {
                    self.add_triangles(&mut mesh_data, x, y, densities, case_index);
                }
            }
        }
        let mut dict = Dictionary::new();
        dict.insert("vertices", &mesh_data.vertices);
        dict.insert("indices", &mesh_data.indices);
        dict.insert("colors", &mesh_data.colors);

        dict
        // PackedVector2Array::from_iter(temp_vertices)
    }

    fn get_d(&self, x: usize, y: usize) -> f32 {
        let d = &self.density_map[x + y * MAP_SIZE];
        return *d;
    }

    fn add_triangles(&self, mesh: &mut MeshData, x: i32, y: i32, d: [f32; 4], case: i32) {
        // Получаем ID материала из карты (берем по координате x, y ячейки)
        let mat_id = self.material_map[(x as usize + y as usize * MAP_SIZE)];

        // Кодируем ID в красный канал (0..1.0)
        let mat_color = Color::from_rgba(mat_id as f32 / 255.0, 0.0, 0.0, 1.0);
        let xf = x as f32;
        let yf = y as f32;

        // Углы
        let p0 = Vector2::new(xf, yf);
        let p1 = Vector2::new(xf + 1.0, yf);
        let p2 = Vector2::new(xf + 1.0, yf + 1.0);
        let p3 = Vector2::new(xf, yf + 1.0);

        // Точки на ребрах (лениво считаем через твою функцию интерполяции)
        let t = self.get_interpolation(p0, p1, d[0], d[1]);
        let r = self.get_interpolation(p1, p2, d[1], d[2]);
        let b = self.get_interpolation(p2, p3, d[2], d[3]);
        let l = self.get_interpolation(p3, p0, d[3], d[0]);

        match case {
            0 => {} // Пусто

            1 => mesh.add_triangle(p0, t, l, mat_color), // Лево-Верх (d0)

            2 => mesh.add_triangle(p1, r, t, mat_color), // Право-Верх (d1)

            3 => {
                // Верхняя грань (d0, d1)
                mesh.add_triangle(p0, p1, r, mat_color);
                mesh.add_triangle(p0, r, l, mat_color);
            }

            4 => mesh.add_triangle(p2, b, r, mat_color), // Право-Низ (d2)

            5 => {
                // Диагональ d0 и d2 (особый случай)
                mesh.add_triangle(p0, t, l, mat_color);
                mesh.add_triangle(p2, b, r, mat_color);
                mesh.add_triangle(t, r, l, mat_color); // Соединительный мостик (зависит от логики SDF)
                mesh.add_triangle(r, b, l, mat_color);
            }

            6 => {
                // Правая грань (d1, d2)
                mesh.add_triangle(p1, p2, b, mat_color);
                mesh.add_triangle(p1, b, t, mat_color);
            }

            7 => {
                // Кроме Лево-Низ
                mesh.add_triangle(p0, p1, p2, mat_color);
                mesh.add_triangle(p0, p2, b, mat_color);
                mesh.add_triangle(p0, b, l, mat_color);
            }

            8 => mesh.add_triangle(p3, l, b, mat_color), // Лево-Низ (d3)

            9 => {
                // Левая грань (d0, d3)
                mesh.add_triangle(p0, t, p3, mat_color);
                mesh.add_triangle(t, b, p3, mat_color);
            }

            10 => {
                // Диагональ d1 и d3 (особый случай)
                mesh.add_triangle(p1, r, t, mat_color);
                mesh.add_triangle(p3, l, b, mat_color);
                mesh.add_triangle(t, r, l, mat_color);
                mesh.add_triangle(r, b, l, mat_color);
            }

            11 => {
                // Кроме Право-Низ
                mesh.add_triangle(p0, p1, r, mat_color);
                mesh.add_triangle(p0, r, b, mat_color);
                mesh.add_triangle(p0, b, p3, mat_color);
            }

            12 => {
                // Нижняя грань (d2, d3)
                mesh.add_triangle(p3, p2, r, mat_color);
                mesh.add_triangle(p3, r, l, mat_color);
            }

            13 => {
                // Кроме Право-Верх
                mesh.add_triangle(p0, t, r, mat_color);
                mesh.add_triangle(p0, r, p2, mat_color);
                mesh.add_triangle(p0, p2, p3, mat_color);
            }

            14 => {
                // Кроме Лево-Верх
                mesh.add_triangle(t, p1, p2, mat_color);
                mesh.add_triangle(t, p2, p3, mat_color);
                mesh.add_triangle(t, p3, l, mat_color);
            }

            15 => {
                // Полный квадрат
                mesh.add_triangle(p0, p1, p2, mat_color);
                mesh.add_triangle(p0, p2, p3, mat_color);
            }

            _ => {}
        }
    }

    fn get_interpolation(&self, p1: Vector2, p2: Vector2, val1: f32, val2: f32) -> Vector2 {
        if (val2 - val1).abs() < 0.00001 {
            return p1;
        }
        let mu = (0.0 - val1) / (val2 - val1);
        p1 + (p2 - p1) * mu
    }

    pub fn modify_terrain(&mut self, pos: Vector2, radius: f32, amount: f32) {
        let radius_sq = radius * radius;

        // Определяем границы изменения в координатах сетки
        let x_min = (pos.x - radius).floor() as i32;
        let x_max = (pos.x + radius).ceil() as i32;
        let y_min = (pos.y - radius).floor() as i32;
        let y_max = (pos.y + radius).ceil() as i32;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                // Проверка границ всей карты
                if x < 0 || y < 0 || x >= MAP_SIZE as i32 || y >= MAP_SIZE as i32 {
                    continue;
                }

                let dx = x as f32 - pos.x;
                let dy = y as f32 - pos.y;
                let dist_sq = dx * dx + dy * dy;

                // Если точка внутри круга
                if dist_sq <= radius_sq {
                    let index = (x as usize) + (y as usize) * MAP_SIZE;

                    // Изменяем плотность. Clamp нужен, чтобы значения не улетали в бесконечность
                    let current = self.density_map[index];
                    self.density_map[index] = (current + amount).clamp(-1.0, 1.0);
                }
            }
        }
    }

    // pub fn generate_biomes(&mut self) {
    //     let center = (MAP_SIZE as f32) / 2.0;

    //     for y in 0..MAP_SIZE {
    //         for x in 0..MAP_SIZE {
    //             let idx = x + y * MAP_SIZE;

    //             let dx = x as f32 - center;
    //             let dy = y as f32 - center;
    //             let dist = (dx * dx + dy * dy).sqrt();
    //             let angle = dy.atan2(dx); // От -PI до PI

    //             // 1. Сначала база по глубине
    //             let mut mat = if dist < 50.0 {
    //                 MAT_CORE // Ядро
    //             } else if dist < 150.0 {
    //                 MAT_STONE // Глубинный камень
    //             } else {
    //                 MAT_DIRT // Поверхностный слой
    //             };

    //             // 2. Накладываем биомы по углу (только для поверхности)
    //             if dist > 140.0 {
    //                 if angle.abs() > 2.5 {
    //                     // Полюса (условно)
    //                     mat = MAT_SNOW;
    //                 } else if angle.abs() < 0.5 {
    //                     // Экватор
    //                     mat = MAT_SAND;
    //                 } else if dist > 180.0 {
    //                     mat = MAT_GRASS;
    //                 }
    //             }

    //             self.material_map[idx] = mat;
    //         }
    //     }
    // }
}
