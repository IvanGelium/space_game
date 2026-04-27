use crate::bridge::api_utils::safe_run;
use crate::world::planet_generator::PlanetGenerator;
use godot::classes::{INode, Image};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldAPI {
    // Внутреннее состояние, скрытое от Godot
    generator: Option<PlanetGenerator>,
    base: Base<Node>,
}

#[godot_api]
impl INode for WorldAPI {
    fn init(base: Base<Node>) -> Self {
        Self {
            generator: None,
            base,
        }
    }
}

#[godot_api]
impl WorldAPI {
    #[func]
    pub fn setup_generator(&mut self, radius: f32) {
        // Оборачиваем даже инициализацию, чтобы видеть ошибки, если что-то пойдет не так
        safe_run("setup_generator", || {
            godot_print!("Bridge: Setting up generator with radius {}", radius);

            // Предположим, что PlanetGenerator::new теперь принимает только радиус
            // и берет MAP_SIZE из констант, как мы решили ранее
            self.generator = Some(PlanetGenerator::new(radius));

            Ok(()) // Возвращаем успех
        });
    }

    #[func]
    pub fn generate_density(&mut self) -> bool {
        // Важно: &mut self, так как генератор будет менять свой density_map
        let res = safe_run("generate_density", || {
            // Достаем мутабельную ссылку на генератор
            let gen = self
                .generator
                .as_mut()
                .ok_or("Генератор не инициализирован! Сначала вызови setup_generator.")?;

            // Вызываем саму логику
            gen.generate_density();

            Ok(true)
        });

        // Если safe_run вернул None (ошибка), отдаем false
        res.unwrap_or(false)
    }
}
