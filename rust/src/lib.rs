
// use godot::prelude::*;

// struct MyExtension;

// #[gdextension]
// unsafe impl ExtensionLibrary for MyExtension {}

// #[derive(GodotClass)]
// #[class(base=Node2D)]
// struct PlanetEngine {
//     base: Base<Node2D>,
// }

// #[godot_api]
// impl INode2D for PlanetEngine {
//     fn init(base: Base<Node2D>) -> Self {
//         godot_print!("Rust Engine Initialized!");
//         Self { base }
//     }
// }

use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct PlanetEngine {
    time: f64,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for PlanetEngine {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Rust Engine Initialized!");
        Self { 
            time: 0.0,
            base 
        }
    }

    // Аналог _process(delta)
    fn process(&mut self, delta: f64) {
        self.time += delta;
        
        // Заставляем узел перерисоваться (вызывает _draw)
        self.base_mut().queue_redraw();
    }

    // Аналог _draw()
    fn draw(&mut self) {
        let center = Vector2::new(200.0 + (self.time.sin() as f32 * 100.0), 200.0);
        let radius = 50.0;
        let color = Color::from_rgb(0.1, 0.8, 0.4);

        // Рисуем круг средствами Godot, вызываемыми из Rust
        self.base_mut().draw_circle(center, radius, color);
    }
}
