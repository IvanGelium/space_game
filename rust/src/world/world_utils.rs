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

// #[derive(godot::prelude::GodotConvert, godot::prelude::Var)]
// #[godot(via = Dictionary)]
// pub struct ChunkMeshData {
//     pub vertices: PackedVector2Array,
//     pub indices: PackedInt32Array,
// }
// impl ChunkMeshData {
//     pub fn new() -> ChunkMeshData {
//         Self {
//             vertices: PackedVector2Array::new(),
//             indices: PackedInt32Array::new(),
//         }
//     }

//     pub fn fill(&mut self, md: MeshData) {
//         self.vertices = md.vertices;
//         self.indices = md.indices;
//     }
// }
