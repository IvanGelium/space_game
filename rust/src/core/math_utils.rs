pub struct NoiseSettings {
    pub noise: noise::OpenSimplex,
    pub scale: f64,
    pub strength: f32,
}

impl NoiseSettings {
    pub fn new(seed: u32, scale: f64, strength: f32) -> Self {
        Self {
            noise: noise::OpenSimplex::new(seed),
            scale,
            strength,
        }
    }

    pub fn get_value(&self, x: f32, y: f32) -> f32 {
        use noise::NoiseFn;
        let val = self
            .noise
            .get([x as f64 * self.scale, y as f64 * self.scale]);
        val as f32 * self.strength
    }
}
