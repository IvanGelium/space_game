use crate::clutch::archive::biomes::*;
use godot::prelude::*;
use std::collections::HashMap;
// Вспомогательная структура для хранения данных меша
pub struct MeshData {
    pub vertices: PackedVector2Array,
    pub indices: PackedInt32Array,
    pub colors: PackedColorArray,
    // Хеш-карта для исключения дубликатов вершин: координаты -> индекс в массиве
    vertex_map: HashMap<(i32, i32), i32>,
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            vertices: PackedVector2Array::new(),
            indices: PackedInt32Array::new(),
            colors: PackedColorArray::new(),
            vertex_map: HashMap::new(),
        }
    }

    // Добавляет вершину, если её нет, и возвращает её индекс
    pub fn add_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        let i1 = self.add_vertex(v1, color);
        let i2 = self.add_vertex(v2, color);
        let i3 = self.add_vertex(v3, color);

        self.indices.extend([i1, i2, i3].into_iter());
    }

    pub fn add_vertex(&mut self, v: Vector2, color: Color) -> i32 {
        // Важно: теперь ключ должен учитывать и координаты, и цвет,
        // либо мы просто пушим вершины без HashMap (для начала так проще)
        let index = self.vertices.len() as i32;
        self.vertices.push(v);
        self.colors.push(color);
        index
    }
}

pub fn get_polar_info(x: f32, y: f32, center: Vector2, radius: f32) -> (f32, f32) {
    let dx = x - center.x;
    let dy = y - center.y;
    let dist = (dx * dx + dy * dy).sqrt();

    // Возвращаем кортеж (угол, относительная_глубина)
    (dy.atan2(dx), dist / radius)
}

pub fn get_structure_index(layer: Layer, belt: Belt, sector: u32) -> usize {
    let l = layer as usize;
    let b = belt as usize;
    let s = sector as usize;
    // Формула: (Слой * Кол-воПоясов * Кол-воСекторов) + (Пояс * Кол-воСекторов) + Сектор
    (l * 5 * 24) + (b * 24) + s
}
