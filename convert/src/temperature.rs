pub mod celsius {
    /// Convert from degrees Celsius (°C) to degrees Kelvin (K)
    pub fn kelvin(celsius: f32) -> f32 {
        celsius + crate::CELSIUS_KELVIN
    }

    /// Convert from degrees Celsius (°C) to degrees Celsius (°F)
    pub fn fahrenheit(celsius: f32) -> f32 {
        9.0 / 5.0 * celsius + 32.0
    }
}

pub mod kelvin {
    /// Convert from degrees Kelvin (K) to degrees Celsius (°C)
    pub fn celsius(kelvin: f32) -> f32 {
        kelvin - crate::CELSIUS_KELVIN
    }

    /// Convert from degrees Kelvin (K) to degrees Fahrenheit (°F)
    pub fn fahrenheit(kelvin: f32) -> f32 {
        crate::celsius::fahrenheit(celsius(kelvin))
    }
}

pub mod fahrenheit {
    /// Convert from degrees Fahrenheit (°F) to degrees Celsius (°C)
    pub fn celsius(fahrenheit: f32) -> f32 {
        (5.0 / 9.0) * (fahrenheit - 32.0)
    }

    /// Convert from degrees Fahrenheit (°F) to degrees Kelvin (K)
    pub fn kelvin(fahrenheit: f32) -> f32 {
        celsius(fahrenheit) + crate::CELSIUS_KELVIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kelvin_to_fahrenheit() {
        assert_eq!((kelvin::fahrenheit(0.0) * 100.0).round() / 100.0, -459.67);
    }

    #[test]
    fn fahrenheit_to_celsius() {
        assert_eq!((fahrenheit::celsius(0.0) * 10000.0).round() / 10000.0, -17.7778);
    }

    #[test]
    fn fahrenheit_to_kelvin() {
        assert_eq!((fahrenheit::kelvin(0.0) * 1000.0).round() / 1000.0, 255.372);
    }
}
