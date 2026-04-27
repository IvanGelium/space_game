use godot::classes::Engine;
use godot::meta::*;
use godot::prelude::*;

// pub struct GlobalConfig {
//     node: Gd<Object>,
// }

// impl GlobalConfig {
//     pub fn new() -> Option<Self> {
//         let engine = Engine::singleton();
//         let node = engine.get_singleton("config")?;
//         Some(Self { node })
//     }

//     pub fn get(&self, name: &str) -> Variant {
//         self.node.get(name)
//     }
// }

// use godot::meta::FromGodot;

// pub trait GodotConfigExt {
//     fn get_as<T: FromGodot>(&self, name: &str) -> T;
// }

// impl<O: Inherits<Object>> GodotConfigExt for Gd<O> {
//     fn get_as<T: FromGodot>(&self, name: &str) -> T {
//         // Используем StringName для ключа
//         let sn = StringName::from(name);
//         // Метод .to::<T>() внутри вызывает FromGodot
//         self.get(sn).to::<T>()
//     }
// }

// pub struct GlobalConfig;

// impl GlobalConfig {
//     pub fn get<T: FromGodot>(name: &str) -> T {
//         Engine::singleton()
//             .get_singleton(StringName::from("Config"))
//             .expect("Autoload 'Config' not found!")
//             .get(StringName::from(name))
//             .to::<T>()
//     }
// }
