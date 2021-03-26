pub enum AtomicStructure {
    Monatomic,
    Diatomic
}

/// The ideal-gas law characterizes the state of a system.
/// It relates the state variables of the system through
/// the following equations:
///
/// - `pV = nRT`
///
/// - `pV = Nk_B T`
///
/// The following equation also holds for a sealed container where
/// the number of moles (and number of molecules) does not change:
///
/// - `p_f V_f / T_f = p_i V_i / T_i`
///
pub mod igl {
    use crate::consts::common::BOLTZMANN;

    /// Find the number density `N/V` in terms of pressure
    /// and temperature using the equation `N/V = p/kBT`
    /// where `kB` is Boltzmann's constant 1.38e-23.
    /// Pressure must be in Pascals (Pa).
    /// Temperature must be in Kelvins (K).
    pub fn number_density(pressure: f32, temperature: f32) -> f32 {
        pressure / (BOLTZMANN * temperature)
    }
}
