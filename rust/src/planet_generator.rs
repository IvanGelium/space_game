use godot::classes::Node2D;
use godot::prelude::*; // Импортируем базовый класс Godot

#[derive(GodotClass)] // Этот макрос делает структуру видимой для Godot
#[class(base=Node2D)] // Указываем, от кого наследуемся
pub struct PlanetGeneratorNew {
    #[base]
    base: Base<Node2D>, // Ссылка на "родительский" объект внутри Godot

    // Твои данные (теперь на Rust)
    pub density_map: Vec<f32>,
}

#[godot_api] // Макрос для регистрации методов (функций) в Godot
impl INode2D for PlanetGeneratorNew {
    // Это конструктор. Вызывается, когда Godot создает объект.
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            density_map: Vec::new(),
        }
    }
}

#[godot_api]
impl PlanetGeneratorNew {
    // Обычный метод, который можно будет вызвать из GDScript: generator.hello()
    #[func]
    pub fn hello(&self) {
        godot_print!("Привет из Rust!");
    }
}
