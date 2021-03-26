pub fn cylinder(radius: f32, length: f32) -> f32 {
    std::f32::consts::PI * radius.powf(2.0) * length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(format!("{:.1}", cylinder(2.0, 3.0)), "37.7");
    }
}
