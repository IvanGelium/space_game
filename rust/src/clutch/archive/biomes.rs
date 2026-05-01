#[derive(Debug, Clone, Copy)]
pub struct Biome {
    pub id: u8,
    pub name: &'static str,
    pub dimensions: Dimensions,
    pub temperature: f32,
}

pub const BIOMES: &[Biome] = &[
    Biome {
        id: 1,
        name: "jungle",
        dimensions: Dimensions {
            height: 10.0,
            width: 20.0,
        },
        temperature: 0.6,
    },
    Biome {
        id: 2,
        name: "desert",
        dimensions: Dimensions {
            height: 10.0,
            width: 20.0,
        },
        temperature: 0.9,
    },
    Biome {
        id: 3,
        name: "Mountins",
        dimensions: Dimensions {
            height: 10.0,
            width: 20.0,
        },
        temperature: 0.3,
    },
];

// Удобные методы для работы
impl Biome {
    pub fn get_all() -> &'static [Biome] {
        BIOMES
    }

    pub fn by_id(id: u8) -> Option<&'static Biome> {
        BIOMES.iter().find(|b| b.id == id)
    }

    pub fn by_name(name: &'static str) -> Option<&'static Biome> {
        BIOMES.iter().find(|b| b.name == name)
    }

    pub fn length() -> usize {
        BIOMES.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub height: f32,
    pub width: f32,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
