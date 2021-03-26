//! This module allows you to convert between units.

pub mod consts {
    /// Knowing that 6.02e23 (1 mol) 12C atoms have a mass of
    /// 0.012 kg, the mass of one 12C atom is given by
    /// `m(12C) = 0.012kg / 6.02e23 = 1.993e-26 kg`
    ///
    /// We defined m(12C) = 12 u, thus the conversion
    /// factor between atomic mass units and kilograms is
    /// `1 u = m(12C) / 12 = 1.66e-27 kg`
    pub const KG_TO_U: f32 = 1.66e-27;
    pub const ATM_TO_PA: f32 = 101325.0;
    pub const CELSIUS_KELVIN: f32 = 273.15;
}

pub mod area;
pub mod volume;
pub mod mass;
pub mod pressure;
pub mod temperature;
pub mod units;

pub use area::*;
pub use consts::*;
pub use volume::*;
pub use mass::*;
pub use pressure::*;
pub use temperature::*;
pub use units::*;
