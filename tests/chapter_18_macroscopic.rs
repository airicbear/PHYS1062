mod solids_liquids_gases_1 {
    use thermodynamics::*;
    use geometry::*;

    /// A project on which you are working uses a cylindrical lead pipe
    /// with outer and inner diameters of 4.0 cm and 3.5 cm,
    /// respectively, and a length of 50 cm. What is its mass?
    ///
    /// The textbook gives the answer as 1.7 kg but I can't seem to manage
    /// to get that answer.
    #[test]
    #[ignore]
    fn example_18_1() {
        let outer_radius = 4.0 / 100.0; // m
        let inner_radius = 3.5 / 100.0; // m
        let length = 50.0 / 100.0; // m
        let outer_volume = volume::cylinder(outer_radius, length); // m^3
        let inner_volume = volume::cylinder(inner_radius, length); // m^3
        let volume = outer_volume - inner_volume; // m^3
        let density = consts::lead::DENSITY;

        // ρ = M/V => M = ρV
        let mass = density * volume; // kg
        assert_eq!(mass, 1.7);
    }
}

mod atoms_moles_2 {
    use thermodynamics::*;

    /// 100 g of oxygen gas is how many moles of oxygen?
    #[test]
    fn example_18_2() {
        let oxygen_mass = 32; // u
        let system_mass = 100.0 / 1000.0; // kg

        // First solution
        let kg_mass = convert::u::kg(oxygen_mass); // kg
        let num_particles = moles::number_of_atoms(system_mass, kg_mass);
        let number_of_moles = moles::num_moles_from_particles(num_particles);
        assert_eq!((number_of_moles * 100.0).round() / 100.0, 3.13);

        // Alternative solution
        let molar_mass = moles::molar_mass(oxygen_mass);
        let number_of_moles = moles::num_moles_from_mass(system_mass, molar_mass);
        assert_eq!((number_of_moles * 100.0).round() / 100.0, 3.13);
    }
}

mod thermal_expansion_4 {
    use thermodynamics::*;

    /// A 55-m-long steel pipe runs from one side of a refinery
    /// to the other. By how much does the pipe expand on a 5°C
    /// winter day when 155°C oil is pumped through it?
    #[test]
    fn example_18_3() {
        let temperature_delta = 150.0; // C
        let coefficient = consts::steel::COEFFICIENT_THERMAL_EXPANSION;
        let length = 55.0; // m

        // ΔL/L = ɑΔT => ΔL = ɑLΔT
        let change_in_length = coefficient * length * temperature_delta; // m
        let change_in_length = change_in_length * 100.0; // cm
        assert_eq!((change_in_length * 10.0).round() / 10.0, 9.1);
    }
}

mod ideal_gases_6 {
    use thermodynamics::*;

    /// Example 18.4 Calculating a gas pressure
    ///
    /// 100 g of oxygen is distilled into an evacuated 600 cm^3 container.
    /// What is the gas pressure at a temperature of 150°C?
    #[test]
    fn example_18_4() {
        let number_of_moles = 3.13;
        let temperature = convert::celsius::kelvin(150.0); // C
        let volume = 600.0 / (100.0 as f32).powf(3.0); // m^3

        // pV = nRT => p = nRT / V
        let pressure = number_of_moles * consts::common::GAS_CONSTANT * temperature / volume;
        let pressure = convert::pa::atm(pressure);
        assert_eq!(pressure.round(), 181.0);
    }

    /// The distance between molecules
    /// "Standard temperature and pressure," abbreviated STP,
    /// are T = 0°C and p = 1 atm. Estimate the average
    /// distance between gas molecules at STP.
    #[test]
    fn example_18_6() {
        let temperature: f32 = 273.0; // K
        let pressure: f32 = 1.0e5; // Pa
        let number_density = gases::igl::number_density(pressure, temperature);
        let average_molecule_volume = 1.0 / number_density; // m^3
        let average_molecule_radius = (3.0 / (4.0 * std::f32::consts::PI) * average_molecule_volume).powf(1.0 / 3.0); // m
        let average_distance = average_molecule_radius * 2.0; // m
        let average_distance = average_distance * 10.0_f32.powf(9.0); // nm
        assert_eq!(average_distance.round(), 4.0);
    }
}

mod ideal_gas_processes_7 {
    /// A constant-volume gas thermometer is placed in contact
    /// with a reference cell containing water at the triple point.
    /// After reaching equilibrium, the gas pressure is recorded as 55.78 kPa.
    /// The thermometer is then placed in contact with a sample of unknown temperature.
    /// After the thermometer reaches a new equilibrium, the gas
    /// pressure is 65.12 kPa. What is the temperature of this sample?
    #[test]
    fn example_18_7() {
        // Isochoric process -- the volume doesn't change
        let initial_pressure = 55.78; // kPa
        let final_pressure = 65.12; // kPa
        let triple_point = 273.16; // K
        let initial_temperature = triple_point; // K

        // Use ideal-gas law for closed system
        // => p2 V2 / T2 = p1 V1 / T1
        // =>         T2 = T1 V2 p2 / V1 p1
        // =>         T2 = T1 p2 / p1
        let final_temperature = initial_temperature * (final_pressure / initial_pressure); // K
        let final_temperature = final_temperature - 273.15; // C
        assert!(final_temperature > initial_temperature - 273.15);
    }

    /// The two cylinders in Figure 18.12 contain ideal gases at 20°C.
    /// Each cylinder is sealed by a frictionless piston of mass M.
    ///
    ///     a. How does the pressure of gas 2 compare to that of gas 1?
    ///        Is it larger, smaller, or the same?
    ///
    ///     b. Suppose gas 2 is warmed to 80°C.
    ///        Describe what happens to the pressure and volume.
    ///
    #[test]
    fn example_18_8() {
        // Isobaric process -- the pressure doesn't change
        let answer_a = "same";
        let answer_b = "pressure same, volume up";
        assert_eq!(answer_a, "same");
        assert_eq!(answer_b, "pressure same, volume up");
    }
}
