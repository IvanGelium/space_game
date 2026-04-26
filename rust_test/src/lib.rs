use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct PlanetEngine {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for PlanetEngine {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Rust Engine Initialized!");
        Self { base }
    }
}
