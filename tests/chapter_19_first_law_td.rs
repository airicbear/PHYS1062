// mod all_about_energy_1 {
//
// }

mod work_igl_2 {
    use geometry::area;

    /// Example 19.1 The work done on an expanding gas
    ///
    /// How much work is done in the ideal-gas process of Figure 19.6?
    #[test]
    fn example_19_1() {
        let a1 = area::rectangle(convert::cm3::m3(1000.0 - 500.0), convert::kpa::pa(100.0 - 0.0));
        let a2 = area::triangle(convert::cm3::m3(1000.0 - 500.0), convert::kpa::pa(300.0 - 100.0));
        let a3 = area::rectangle(convert::cm3::m3(1500.0 - 1000.0), convert::kpa::pa(300.0 - 0.0));
        let work = -(a1 + a2 + a3);
        assert_eq!(work, -250.0);
    }

    /// Example 19.2 The work of an isothermal compression
    ///
    /// A cylinder contains 7.0 g of nitrogen gas. How much
    /// work must be done to compress the gas at a constant
    /// temperature of 80°C until the volume is halved?
    #[test]
    fn example_19_2() {
        // Nitrogen gas is N₂ with molar mass 28.0 g/mol,
        // so 7.0 g is 0.25 mol of gas.
        let moles = 0.25; // mol
        let temperature = convert::celsius::kelvin(80.0); // K
        let work = -(moles * thermodynamics::consts::GAS_CONSTANT * temperature * (0.5 as f32).ln());
        assert_eq!(work.floor(), 508.0);
    }
}

mod heat_3 {
    use thermodynamics::heat;

    /// Example 19.3 Running a fever
    ///
    /// A 70 kg student catches the flu, and his body temperature
    /// increases from 37.0°C (98.6°F) to 39.0°C (102.2°F). How much
    /// energy is required to raise his body's temperature? The specific
    /// heat of a mammalian body is 3400 J/kg K, nearly that of water
    /// because mammals are mostly water.
    #[test]
    fn example_19_3() {
        let mass = 70.0; // kg
        let initial_temperature = convert::celsius::kelvin(37.0);
        let final_temperature = convert::celsius::kelvin(39.0);
        let temperature_change = final_temperature - initial_temperature;
        let specific_heat = 3400.0; // J/kg K
        let heat = heat::from_mass(mass, specific_heat, temperature_change); // J
        assert_eq!((heat / 10000.0).round() * 10000.0, 4.8e5);
    }
}

// mod first_law_td_4 {
//
// }

mod thermal_properties_of_matter_5 {
    /// Example 19.4 Melting wax
    ///
    /// An insulated jar containing 200 g of solid candle wax is placed on a hot
    /// plate that supplies heat energy to the wax at the rate of 220 J/s. The wax
    /// temperature is measured every 30 s, yielding the following data:
    ///
    /// | Time (s) | Temperature (°C) |
    /// |----------+------------------|
    /// |        0 |             20.0 |
    /// |       30 |             31.7 |
    /// |       60 |             42.2 |
    /// |       90 |             55.0 |
    /// |      120 |             64.7 |
    /// |      150 |             70.4 |
    /// |      180 |             70.5 |
    /// |      210 |             70.5 |
    /// |      240 |             70.6 |
    /// |      270 |             70.5 |
    /// |      300 |             70.4 |
    /// |      330 |             74.5 |
    ///
    /// What are the specific heat of the solid wax, the melting point, and the
    /// wax's heat of fusion?
    #[test]
    fn example_19_4() {
        let mass = convert::g::kg(200.0); // kg
        let rate = 220.0; // J/s
        let start_melting = (150.0 - 120.0) / 2.0 + 120.0; // s
        let finish_melting = (330.0 - 300.0) / 2.0 + 300.0; // s
        let melting = finish_melting - start_melting; // s
        let heat = melting * rate; // J
        let heat_of_fusion = heat / mass;
        assert_eq!((heat_of_fusion / 100000.0).round() * 100000.0, 2.0e5);
    }
}

mod calorimetry_6 {
    use thermodynamics::consts;

    /// Example 19.5 Calorimetery with a phase change
    ///
    /// Your 500 mL soda is at 20°C, room temperature, so you add 100 g of ice
    /// from the -20°C freezer. Does all the ice melt? If so, what is the final
    /// temperature? If not, what fraction of the ice melts? Assume that your have
    /// a well-insulated cup.
    #[test]
    #[ignore]
    fn example_19_5() {
        let temperature_change = 20.0; // K
        let mass_ice = convert::g::kg(100.0); // kg
        let specific_heat_ice = consts::ice::SPECIFIC_HEAT; // J/kg K
        let specific_heat_water = consts::water::SPECIFIC_HEAT; // J/kg K
        let heat_of_fusion = consts::water::HEAT_OF_FUSION; // J/kg
        let heat_melting = mass_ice * specific_heat_ice * temperature_change + mass_ice * heat_of_fusion; // J
        let volume = convert::ml::m3(500.0); // m^3
        let density = 997.0; // kg/m^3
        let mass_soda = density * volume; // kg
        //let heat_cool = mass_soda * specific_heat_water * -temperature_change; // J
        let final_temperature = (mass_soda * specific_heat_water * temperature_change - heat_melting) / (mass_ice * specific_heat_ice + mass_soda * specific_heat_water);
        assert_eq!(final_temperature, 1.7);
    }

    /// Example 19.6 Three interacting systems
    ///
    /// A 200 g piece of iron at 120°C and a 150 g piece of copper at -50°C are
    /// dropped into an insulated beaker containing 300 g of ethyl alcohol at
    /// 20°C. What is the final temperature?
    #[test]
    fn example_19_6() {
        let mass_iron = 200.0; // g
        let mass_copper = 150.0; // g
        let mass_ethyl_alcohol = 300.0; // g
        let temperature_iron = 120.0; // C
        let temperature_copper = -50.0; // C
        let temperature_ethyl_alcohol = 20.0; // C

        // Qi + Qc + Qe = Mi ci (Tf - 120) + Mc cc (Tf - (-50)) + Me ce (Tf - 20) = 0
        let temperature_final =
            (mass_iron * consts::iron::SPECIFIC_HEAT * temperature_iron
             + mass_copper * consts::copper::SPECIFIC_HEAT * temperature_copper
             + mass_ethyl_alcohol * consts::ethyl_alcohol::SPECIFIC_HEAT * temperature_ethyl_alcohol)
            / (mass_iron * consts::iron::SPECIFIC_HEAT
               + mass_copper * consts::copper::SPECIFIC_HEAT
               + mass_ethyl_alcohol * consts::ethyl_alcohol::SPECIFIC_HEAT);
        assert_eq!(temperature_final.round(), 26.0);
    }
}

mod specific_heat_of_gases_7 {
    use thermodynamics::consts;

    /// Example 19.7 Heating and cooling a gas
    ///
    /// Three moles of O₂ gas are at 20.0°C. 600 J of heat energy are transferred
    /// to the gas at constant pressure, then 600 J are removed at constant volume.
    /// What is the final temperature? Show the process on a pV diagram.
    #[test]
    fn example_19_7() {
        let n = 3.0; // mol
        let t1 = 20.0; // C
        let cp = consts::atom::oxygen::MOLAR_SPECIFIC_HEAT_CONSTANT_PRESSURE; // J/mol K
        let cv = consts::atom::oxygen::MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME; // J/mol K
        let q = 600.0; // J

        // Q = nCΔT
        // ΔT = Q/nC
        //
        // Isobaric process
        let delta_t1 = q / (n * cp);
        //
        // Isochoric process
        let delta_t2 = -q / (n * cv);

        let t3 = t1 + delta_t1 + delta_t2;
        assert_eq!((t3 * 10.0).round() / 10.0, 17.3);
        assert!(t3 < t1);
    }

    /// Example 19.8 Calorimetry with a gas and a solid
    ///
    /// The interior volume of a 200 g hollow aluminum box is 800 cm³. The box
    /// contains nitrogen gas at STP. A 20 cm³ block of copper at a temperature
    /// of 300°C is placed inside the box, then the box is sealed. What is the
    /// final temperature?
    #[test]
    fn example_19_8() {
        let m_al = convert::g::kg(200.0); // g
        let t_al = convert::celsius::kelvin(0.0); // C
        let t_n2 = convert::celsius::kelvin(0.0); // C
        let t_cu = convert::celsius::kelvin(300.0); // C
        //let v_al = convert::cm3::m3(800.0); // m^3
        let v_n2 = convert::cm3::m3(800.0 - 20.0); // m^3
        let v_cu = convert::cm3::m3(20.0); // m^3
        let c_al = consts::aluminum::SPECIFIC_HEAT;
        let c_v = consts::atom::nitrogen::MOLAR_SPECIFIC_HEAT_CONSTANT_VOLUME;
        let c_cu = consts::copper::SPECIFIC_HEAT;

        // Compute unknown values
        let m_cu = consts::copper::DENSITY * v_cu; // kg
        assert_eq!((m_cu * 1000.0).round() / 1000.0, 0.178);

        let n_n2 = (convert::atm::pa(1.0) * v_n2) / (consts::GAS_CONSTANT * t_n2); // mol
        assert_eq!((n_n2 * 10000.0).round() / 10000.0, 0.0348);

        // Q(net) = Q(Al) + Q(N2) + Q(Cu) = 0
        // Q(net) = m(Al)*c(Al)*(T(f) - T(Al)) + n(N2)*C(V)*(T(f) - T(N2)) + m(Cu)*c(Cu)*(T(f) - T(Cu)) = 0
        // T(f) = (m(Al)*c(Al)*T(Al) + n(N2)*C(V)*T(N2) + m(Cu)*c(Cu)*T(Cu)) / (m(Al)*c(Al) + n(N2)*C(V) + m(Cu)*c(Cu))
        let t_f =
            (m_al * c_al * t_al
             + n_n2 * c_v * t_n2
             + m_cu * c_cu * t_cu)
            / (m_al * c_al
               + n_n2 * c_v
               + m_cu * c_cu);
        let t_f = convert::kelvin::celsius(t_f);
        assert_eq!(t_f.round(), 83.0);
    }
}

mod heat_transfer_mechanisms_8 {
    /// A 1.8-m-wide by 1.0-m-tall by 0.65-m-deep home freeer
    /// is insulated with 5.0-cm-thick Styrofoam insulation.
    /// At what rate must the compressor remove heat from the
    /// freezer to keep the inside at -20°C in a room where
    /// the air temperature is 25°C?
    #[test]
    fn example_19_10() {
        let width = 1.8;
        let height = 1.0;
        let depth = 0.65;
        let volume = width * height * depth;
        let insulation_width = convert::cm::m(5.0);
        println!("{}", insulation_width);
        panic!();
    }
}
