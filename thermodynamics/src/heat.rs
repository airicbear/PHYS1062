pub fn from_mass(mass: f32, specific_heat: f32, temperature_change: f32) -> f32 {
    mass * specific_heat * temperature_change
}

pub fn from_moles(moles: f32, molar_specific_heat: f32, temperature_change: f32) -> f32 {
    moles * molar_specific_heat * temperature_change
}

pub fn of_transformation(mass: f32, heat_of_transformation: f32) -> f32 {
    mass * heat_of_transformation
}
