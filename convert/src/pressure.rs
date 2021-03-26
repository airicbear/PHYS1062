pub mod kpa {
    pub fn pa(kpa: f32) -> f32 {
        kpa * 1000.0
    }
}

pub mod pa {
    /// Convert from Pascals (Pa) to atmospheres (atm).
    pub fn atm(pa: f32) -> f32 {
        pa / crate::ATM_TO_PA
    }
}

pub mod atm {
    /// Convert from atmospheres (atm) to Pascals (Pa).
    pub fn pa(atm: f32) -> f32 {
        atm * crate::ATM_TO_PA
    }
}
