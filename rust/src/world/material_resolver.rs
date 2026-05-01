pub struct MaterialResolver {
    pub planet_radius: f32,
    pub atmos_height: f32,
}

impl MaterialResolver {
    pub fn new(planet_radius: f32) -> Self {
        Self {
            planet_radius,
            atmos_height: 64.0, // Высота атмосферы в ячейках
        }
    }

    pub fn resolve(&self, dist: f32, is_solid: bool) -> u8 {
        if is_solid {
            // Логика грунта (внутренность планеты)
            let depth = self.planet_radius - dist; // Глубина от поверхности внутрь
            let relative_depth = depth / self.planet_radius;

            if relative_depth < 0.02 {
                return 6;
            } // Трава (верхний слой)
            if relative_depth < 0.10 {
                return 7;
            } // Земля
            if relative_depth < 0.40 {
                return 8;
            } // Камень
            if relative_depth < 0.70 {
                return 9;
            } // Твердый камень
            if relative_depth < 0.85 {
                return 10;
            } // Мантия
            if relative_depth < 0.95 {
                return 11;
            } // Лава
            return 12; // Ядро
        } else {
            // Логика атмосферы (снаружи планеты)
            let height = dist - self.planet_radius; // Высота над поверхностью

            if height > self.atmos_height {
                return 0;
            } // Вакуум

            // Распределяем 5 слоев атмосферы (ID 1-5) по высоте 64 ячейки
            let atmos_layer = (height / (self.atmos_height / 5.0)).floor() as u8;
            return 1 + atmos_layer; // Вернет от 1 до 5
        }
    }
}
