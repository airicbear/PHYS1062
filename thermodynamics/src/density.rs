pub fn density(mass: f32, volume: f32) -> f32 {
    mass / volume
}

pub fn volume(mass: f32, density: f32) -> f32 {
    mass / density
}

pub fn mass(density: f32, volume: f32) -> f32 {
    density * volume
}
