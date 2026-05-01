use godot::prelude::*;
use std::collections::HashMap;
// Вспомогательная структура для хранения данных меша
pub struct MeshData {
    pub vertices: PackedVector2Array,
    pub indices: PackedInt32Array,
    // Хеш-карта для исключения дубликатов вершин: координаты -> индекс в массиве
    vertex_map: HashMap<(i32, i32), i32>,
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            vertices: PackedVector2Array::new(),
            indices: PackedInt32Array::new(),
            vertex_map: HashMap::new(),
        }
    }

    // Добавляет вершину, если её нет, и возвращает её индекс
    pub fn add_vertex(&mut self, v: Vector2) -> i32 {
        let key = ((v.x * 1000.0) as i32, (v.y * 1000.0) as i32);
        if let Some(&index) = self.vertex_map.get(&key) {
            index
        } else {
            let index = self.vertices.len() as i32;
            self.vertices.push(v);
            self.vertex_map.insert(key, index);
            index
        }
    }

    // Добавляет треугольник по трем точкам
    pub fn add_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2) {
        let i1 = self.add_vertex(v1);
        let i2 = self.add_vertex(v2);
        let i3 = self.add_vertex(v3);
        self.indices.extend([i1, i2, i3]);
    }
}

pub fn get_polar_info(x: f32, y: f32, center: Vector2, radius: f32) -> (f32, f32) {
    let dx = x - center.x;
    let dy = y - center.y;
    let dist = (dx * dx + dy * dy).sqrt();

    // Возвращаем кортеж (угол, относительная_глубина)
    (dy.atan2(dx), dist / radius)
}
