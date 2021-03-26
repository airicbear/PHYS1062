pub mod cm3 {
    pub fn m3(cm3: f32) -> f32 {
        cm3 / (100.0 as f32).powf(3.0)
    }
}

pub mod ml {
    pub fn m3(ml: f32) -> f32 {
        ml / 1.0e6
    }
}
