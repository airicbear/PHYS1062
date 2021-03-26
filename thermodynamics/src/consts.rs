pub use common::*;
pub use material::*;

pub mod common {
    pub const AVOGADRO: f32 = 6.02e23; // particles/mol
    pub const ATM: f32 = 101300.0; // Pa
    pub const BOLTZMANN: f32 = 1.38e-23; // J/K
    pub const GAS_CONSTANT: f32 = 8.3144598; // J/mol K
}

/// Density is in (kg/m^3).
/// Coefficient of thermal expansion is in (°C^-1).
/// Specific heat is in (J/kg K).
/// Heat of fusion is in (J/kg).
pub mod material {
    pub mod air {
        pub const DENSITY: f32 = 1.29;
    }

    pub mod ice {
        pub const DENSITY: f32 = 920.0;
        pub const SPECIFIC_HEAT: f32 = 2090.0;
    }

    pub mod water {
        pub const DENSITY: f32 = 1000.0;
        pub const SPECIFIC_HEAT: f32 = 4190.0;
        pub const HEAT_OF_FUSION: f32 = 3.33e5;
    }

    pub mod ethyl_alcohol {
        pub const DENSITY: f32 = 790.0;
        pub const COEFFICIENT_THERMAL_EXPANSION: f32 = 1.1e-4;
        pub const SPECIFIC_HEAT: f32 = 2400.0;
        pub const HEAT_OF_FUSION: f32 = 1.09e5;
    }

    pub mod seawater {
        pub const DENSITY: f32 = 1030.0;
    }

    pub mod oil {
        pub const DENSITY: f32 = 900.0;
    }

    pub mod gasoline {
        pub const DENSITY: f32 = 680.0;
    }

    pub mod glycerin {
        pub const DENSITY: f32 = 1260.0;
    }

    pub mod mercury {
        pub const COEFFICIENT_THERMAL_EXPANSION: f32 = 1.8e-4;
        pub const DENSITY: f32 = 13600.0;
        pub const SPECIFIC_HEAT: f32 = 140.0;
        pub const HEAT_OF_FUSION: f32 = 0.11e5;
    }

    pub mod aluminum {
        pub const COEFFICIENT_THERMAL_EXPANSION: f32 = 2.3e-5;
        pub const DENSITY: f32 = 2700.0;
        pub const SPECIFIC_HEAT: f32 = 900.0;
    }

    pub mod copper {
        pub const DENSITY: f32 = 8920.0;
        pub const SPECIFIC_HEAT: f32 = 385.0;
    }

    pub mod gold {
        pub const DENSITY: f32 = 19300.0;
        pub const SPECIFIC_HEAT: f32 = 129.0;
    }

    pub mod iron {
        pub const DENSITY: f32 = 7870.0;
        pub const SPECIFIC_HEAT: f32 = 449.0;
    }

    pub mod lead {
        pub const DENSITY: f32 = 11300.0;
        pub const SPECIFIC_HEAT: f32 = 128.0;
        pub const HEAT_OF_FUSION: f32 = 0.25e5;
    }

    pub mod steel {
        pub const COEFFICIENT_THERMAL_EXPANSION: f32 = 1.1e-5;
    }

    pub mod silicon {
        pub const DENSITY: f32 = 2330.0;
        pub const SPECIFIC_HEAT: f32 = 703.0;
    }
}

/// Z means "atomic number"
pub mod atom {
    pub mod hydrogen {
        pub const Z: u32 = 1;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 28.7;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 20.4;
    }

    pub mod helium {
        pub const Z: u32 = 2;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 20.8;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 12.5;
    }

    pub mod carbon {
        pub const Z: u32 = 6;
    }

    pub mod nitrogen {
        pub const Z: u32 = 7;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 29.1;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 20.8;
    }

    pub mod oxygen {
        pub const Z: u32 = 8;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 29.2;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 20.9;
    }

    pub mod neon {
        pub const Z: u32 = 10;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 20.8;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 12.5;
    }

    pub mod aluminum {
        pub const Z: u32 = 13;
    }

    pub mod argon {
        pub const Z: u32 = 18;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE: f32 = 20.8;
        pub const MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME: f32 = 12.5;
    }

    pub mod lead {
        pub const Z: u32 = 82;
    }

    pub mod uranium {
        pub const Z: u32 = 92;
    }
}
