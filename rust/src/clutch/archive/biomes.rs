#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)] // Это говорит Rust хранить енам как число u8
pub enum Layer {
    Vacuum = 0,
    AtmosHigh = 1,
    AtmosLow = 2,
    Mountains = 3,
    Surface = 4,
    Underground = 5,
    DeepUnderground = 6,
    Mantle = 7,
    Lava = 8,
    Core = 9,
}

impl Layer {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Layer::Vacuum,
            1 => Layer::AtmosHigh,
            2 => Layer::AtmosLow,
            3 => Layer::Mountains,
            4 => Layer::Surface,
            5 => Layer::Underground,
            6 => Layer::DeepUnderground,
            7 => Layer::Mantle,
            8 => Layer::Lava,
            9 => Layer::Core,
            _ => Layer::Vacuum, // Дефолт в случае ошибки
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Belt {
    Arctic = 0,
    Tundra = 1,
    Temperate = 2,
    Tropics = 3,
    Equator = 4,
}

impl Belt {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Belt::Arctic,
            1 => Belt::Tundra,
            2 => Belt::Temperate,
            3 => Belt::Tropics,
            4 => Belt::Equator,
            _ => Belt::Arctic, // Дефолт в случае ошибки
        }
    }
}
// Константы для настройки нарезки
pub const SECTOR_COUNT: u32 = 24; // 12 секторов на видимую сторону
