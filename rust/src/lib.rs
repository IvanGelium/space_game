// #[derive(GodotClass)]
// #[class(base=Node2D)]
// struct PlanetEngine {
//     time: f64,
//     base: Base<Node2D>,
// }

// #[godot_api]
// impl INode2D for PlanetEngine {
//     fn init(base: Base<Node2D>) -> Self {
//         godot_print!("Rust Engine Initialized!");
//         Self { 
//             time: 0.0,
//             base 
//         }
//     }

//     // Аналог _process(delta)
//     fn process(&mut self, delta: f64) {
//         self.time += delta;
        
//         // Заставляем узел перерисоваться (вызывает _draw)
//         self.base_mut().queue_redraw();
//     }

//     // Аналог _draw()
//     fn draw(&mut self) {
//         let center = Vector2::new(200.0 + (self.time.sin() as f32 * 100.0), 200.0);
//         let radius = 50.0;
//         let color = Color::from_rgb(0.1, 0.8, 0.4);

//         // Рисуем круг средствами Godot, вызываемыми из Rust
//         self.base_mut().draw_circle(center, radius, color);
//     }
// }

use godot::prelude::*;

mod planet_generator; 

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


// use godot::classes::{Node2D, FastNoiseLite};

// #[derive(GodotClass)]
// #[class(base=Node2D)]
// pub struct PlanetEngine {
//     #[base]
//     base: Base<Node2D>,
//     density_map: Vec<f32>,
//     noise: Gd<FastNoiseLite>,
//     map_size: i32,
// }

// #[godot_api]
// impl INode2D for PlanetEngine {
//     fn init(base: Base<Node2D>) -> Self {
//         let mut noise = FastNoiseLite::new_gd();
//         noise.set_seed(1337);
//         noise.set_frequency(0.04);

//         Self {
//             base,
//             density_map: Vec::new(),
//             noise,
//             map_size: 0,
//         }
//     }
// }

// #[godot_api]
// impl PlanetEngine {
//     #[func]
//     pub fn setup(&mut self, size: i32) {
//         self.map_size = size;
//         self.density_map = vec![0.0; (size * size) as usize];
//     }

//     #[func]
//     pub fn generate_density(&mut self, radius: f32) {
//         let center = self.map_size as f32 / 2.0;
//         for y in 0..self.map_size {
//             for x in 0..self.map_size {
//                 let dx = x as f32 - center;
//                 let dy = y as f32 - center;
//                 let dist = (dx*dx + dy*dy).sqrt();
//                 let n = self.noise.get_noise_2d(x as f32, y as f32) * 5.0;
//                 self.density_map[(x + y * self.map_size) as usize] = radius - dist + n;
//             }
//         }
//     }

//     #[func]
//     pub fn modify_terrain(&mut self, pos: Vector2, radius: f32, amount: f32) {
//         let r_int = radius.ceil() as i32;
//         let cx = pos.x as i32;
//         let cy = pos.y as i32;

//         for y in (cy - r_int)..(cy + r_int) {
//             for x in (cx - r_int)..(cx + r_int) {
//                 if x < 0 || x >= self.map_size || y < 0 || y >= self.map_size { continue; }
                
//                 let dist = pos.distance_to(Vector2::new(x as f32, y as f32));
//                 if dist < radius {
//                     let idx = (x + y * self.map_size) as usize;
//                     // Плавное затухание кисти
//                     let falloff = 1.0 - (dist / radius);
//                     self.density_map[idx] += amount * falloff;
//                     self.density_map[idx] = self.density_map[idx].clamp(-50.0, 50.0);
//                 }
//             }
//         }
//     }

//     #[func]
//     pub fn get_chunk_geometry(&self, chunk_pos: Vector2i, chunk_size: i32) -> PackedVector2Array {
//         let mut segments = Vec::new();

//         // 1. Сбор отрезков Marching Squares
//         for y in chunk_pos.y..(chunk_pos.y + chunk_size) {
//             for x in chunk_pos.x..(chunk_pos.x + chunk_size) {
//                 if x >= self.map_size - 1 || y >= self.map_size - 1 { continue; }

//                 let d = [
//                     self.get_d(x, y), self.get_d(x+1, y),
//                     self.get_d(x+1, y+1), self.get_d(x, y+1)
//                 ];

//                 let mut case_index = 0;
//                 if d[0] > 0.0 { case_index |= 1; }
//                 if d[1] > 0.0 { case_index |= 2; }
//                 if d[2] > 0.0 { case_index |= 4; }
//                 if d[3] > 0.0 { case_index |= 8; }

//                 if case_index > 0 && case_index < 15 {
//                     self.append_edges(&mut segments, x, y, d, case_index);
//                 }
//             }
//         }

//         // 2. Сшивка отрезков в контур (простейший вариант для Polygon2D)
//         self.stitch_to_packed_array(segments)
//     }

//     fn get_d(&self, x: i32, y: i32) -> f32 {
//         self.density_map[(x + y * self.map_size) as usize]
//     }

//     fn append_edges(&self, segments: &mut Vec<Vector2>, x: i32, y: i32, d: [f32; 4], case: i32) {
//         let p = [
//             Vector2::new(x as f32, y as f32), Vector2::new(x as f32 + 1.0, y as f32),
//             Vector2::new(x as f32 + 1.0, y as f32 + 1.0), Vector2::new(x as f32, y as f32 + 1.0)
//         ];

//         // Интерполяция на ребрах
//         let t = self.lerp(p[0], p[1], d[0], d[1]);
//         let r = self.lerp(p[1], p[2], d[1], d[2]);
//         let b = self.lerp(p[2], p[3], d[2], d[3]);
//         let l = self.lerp(p[3], p[0], d[3], d[0]);

//         match case {
//             1 => segments.extend_from_slice(&[l, t]),
//             2 => segments.extend_from_slice(&[t, r]),
//             3 => segments.extend_from_slice(&[l, r]),
//             4 => segments.extend_from_slice(&[r, b]),
//             5 => segments.extend_from_slice(&[l, t, r, b]),
//             6 => segments.extend_from_slice(&[t, b]),
//             7 => segments.extend_from_slice(&[l, b]),
//             8 => segments.extend_from_slice(&[b, l]),
//             9 => segments.extend_from_slice(&[b, t]),
//             10 => segments.extend_from_slice(&[t, l, b, r]),
//             11 => segments.extend_from_slice(&[t, r]),
//             12 => segments.extend_from_slice(&[r, l]),
//             13 => segments.extend_from_slice(&[t, r]),
//             14 => segments.extend_from_slice(&[l, t]),
//             _ => {}
//         }
//     }

//     fn lerp(&self, p1: Vector2, p2: Vector2, v1: f32, v2: f32) -> Vector2 {
//         let mu = (0.0 - v1) / (v2 - v1);
//         Vector2::new(p1.x + mu * (p2.x - p1.x), p1.y + mu * (p2.y - p1.y))
//     }

//     fn stitch_to_packed_array(&self, segments: Vec<Vector2>) -> PackedVector2Array {
//         if segments.is_empty() { return PackedVector2Array::new(); }
//         // В рамках MVP просто возвращаем точки. 
//         // В полноценной версии тут должен быть поиск следующей точки по хеш-карте.
//         PackedVector2Array::from_iter(segments.into_iter())
//     }
// }
