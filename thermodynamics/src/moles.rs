use crate::consts::*;

/// Find the number density `N/V` of a system.
pub fn number_density(num_atoms: u32, volume: f32) -> u32 {
    num_atoms / volume.round() as u32
}

/// The atomic mass number of an atom is given by
/// the sum of the number of protons and neutrons.
///
/// An isotope is such an atom where the number of
/// neutrons is not equal to the number of protons.
///
/// For our purposes, we will use the integer atomic
/// mass numbers as the values of the atomic mass.
/// E.g. m(12C) = 12 u, m(1H) = 1 u, m(4He) = 4 u.
pub fn atomic_mass_number(num_protons: u32, num_neutrons: u32) -> u32 {
    num_protons + num_neutrons
}

/// The number of moles in a substance containing `N`
/// basic particles is given by the `N/NA` where `NA`
/// is Avogadro's number.
pub fn num_moles_from_particles(num_particles: f32) -> f32 {
    num_particles / AVOGADRO
}

/// Find the number of atoms in a system of mass `M`
/// with atom mass `m` given by `N = M/m` where both
/// masses are given in kg.
pub fn number_of_atoms(mass: f32, atom_mass: f32) -> f32 {
    mass / atom_mass
}

/// The molar mass of a substance whose atomic or molecular
/// mass is given relative to 12C is the atomic or molecular
/// mass divided by 1000.
///
/// For example,
/// the molar mass of He, with m = 4 u, is M_mol (He) = 0.004 kg/mol
/// and the molar mass of diatomic O2 is M_mol (O_2) = 0.032 kg/mol.
pub fn molar_mass(molecular_mass: u32) -> f32 {
    molecular_mass as f32 / 1000.0
}

/// Similar to how we find `number_of_atoms` using atomic mass,
/// we can find `number_of_moles` using molar mass by
/// `n = M/M_mol` where `M` is the system mass and `M_mol` is the molar mass.
pub fn num_moles_from_mass(mass: f32, molar_mass: f32) -> f32 {
    mass / molar_mass
}
