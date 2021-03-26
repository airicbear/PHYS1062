pub mod u {
    /// Convert from atomic mass units (u) to kilograms (kg).
    pub fn kg(u: u32) -> f32 {
        u as f32 * crate::KG_TO_U
    }
}

pub mod g {
    /// Convert from grams (g) to kilograms (kg)
    pub fn kg(g: f32) -> f32 {
        g / 1000.0
    }
}

pub mod kg {
    /// Convert from kilograms (kg) to grams (g)
    pub fn g(kg: f32) -> f32 {
        kg * 1000.0
    }
    /// Convert from kilograms (kg) to atomic mass units (u).
    pub fn u(kg: f32) -> u32 {
        (kg / crate::KG_TO_U).round() as u32
    }
}
