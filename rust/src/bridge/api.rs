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
    pub fn setup_generator(&mut self, radius: f32, atmos_height: f32) {
        // Оборачиваем даже инициализацию, чтобы видеть ошибки, если что-то пойдет не так
        safe_run("setup_generator", || {
            godot_print!("Bridge: Setting up generator with radius {}", radius);

            // Предположим, что PlanetGenerator::new теперь принимает только радиус
            // и берет MAP_SIZE из констант, как мы решили ранее
            self.generator = Some(PlanetGenerator::new(radius, atmos_height));

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
                .ok_or("Не удалось сгенерировать карту плотности!")?;

            // Вызываем саму логику
            gen.generate_density();

            Ok(true)
        });

        // Если safe_run вернул None (ошибка), отдаем false
        res.unwrap_or(false)
    }

    #[func]
    pub fn get_chunk_mesh(&mut self, chunk_pos: Vector2i) -> Dictionary<StringName, Variant> {
        let res = safe_run("get_chunk_mesh", || {
            let gen = self.generator.as_mut().ok_or("Не удалось получить чанк!")?;
            let mesh = gen.get_chunk_mesh(chunk_pos);

            Ok(mesh)
        });
        res.unwrap_or_else(Dictionary::<StringName, Variant>::new)
    }

    #[func]
    pub fn modify_terrain(&mut self, world_pos: Vector2, radius: f32, amount: f32) {
        safe_run("modify_terrain", || {
            let gen = self.generator.as_mut().ok_or("Generator not initialized")?;

            // world_pos в Godot обычно соответствует индексам сетки,
            // если GRID_STEP = 1.0. Если нет — раздели на GRID_STEP.
            gen.modify_terrain(world_pos, radius, amount);

            Ok(())
        });
    }
}
