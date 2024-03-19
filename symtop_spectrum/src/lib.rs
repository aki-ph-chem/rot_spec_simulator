pub mod micro_wave;
pub mod symtop;

#[derive(Debug)]
pub struct LineList {
    j_ground: i64,
    k_ground: i64,
    j_excited: i64,
    k_excited: i64,
    intensity: f64,
}

impl LineList {
    pub fn new(
        j_ground: i64,
        k_ground: i64,
        j_excited: i64,
        k_excited: i64,
        intensity: f64,
    ) -> Self {
        Self {
            j_ground,
            k_ground,
            j_excited,
            k_excited,
            intensity,
        }
    }
}
