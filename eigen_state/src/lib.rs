#[derive(Debug)]
pub struct SymmetricTop {
    a: f64,
    b: f64,
}

impl SymmetricTop {
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    pub fn energy(&self, j: i64, k: i64) -> f64 {
        let j_part = (j.pow(2) + j) as f64;
        let k_part = k.pow(2) as f64;

        self.b * j_part + (self.a - self.b) * k_part
    }
}

#[derive(Debug)]
pub struct EnergyManifold {
    eigen_state: SymmetricTop,
    energy_eigen_values: Vec<Vec<f64>>,
}

impl EnergyManifold {
    pub fn new(j_max: i64, a: f64, b: f64) -> Self {
        let mut energy_eigen_values = vec![];

        for k in 0..=(2 * j_max as usize + 1) {
            energy_eigen_values.push(vec![0.0; k]);
        }

        Self {
            eigen_state: SymmetricTop::new(a, b),
            energy_eigen_values,
        }
    }

    pub fn calc_energy(&mut self) {
        for (j, k_sub_structure) in &mut self.energy_eigen_values.iter_mut().enumerate() {
            for (k, energy_eigen_value) in k_sub_structure.iter_mut().enumerate() {
                *energy_eigen_value = self.eigen_state.energy(j as i64, k as i64);
            }
        }
    }
}
