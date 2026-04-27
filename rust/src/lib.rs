use godot::prelude::*;

mod bridge;
mod clutch;
mod core;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
