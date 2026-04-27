use rand::{Rng, SeedableRng}; // Трейты для работы с рандомом
use rand_pcg::Pcg64; // Качественный и быстрый алгоритм

pub struct WorldSeeder {
    master_rng: Pcg64,
}

impl WorldSeeder {
    // Конструктор: создаем мастер-генератор из одного числа
    pub fn new(master_seed: u64) -> Self {
        Self {
            master_rng: Pcg64::seed_from_u64(master_seed),
        }
    }

    // Метод для получения сида для шума (u32, так как noise-rs обычно хочет u32)
    pub fn next_noise_seed(&mut self) -> u32 {
        self.master_rng.gen()
    }

    // Создает новый независимый генератор для конкретной подсистемы
    // Например, для генерации мобов или лута
    pub fn spawn_sub_rng(&mut self) -> Pcg64 {
        Pcg64::seed_from_u64(self.master_rng.gen())
    }
}
