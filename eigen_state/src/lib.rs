use std::collections::HashMap;

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
    pub energy_eigen_values: Vec<HashMap<i64, f64>>,
}

impl EnergyManifold {
    pub fn new(j_max: i64, a: f64, b: f64) -> Self {
        let mut energy_eigen_values = vec![];
        for j in 0..=j_max {
            let mut k_sub_struecure: HashMap<i64, f64> = HashMap::new();
            for k in -j..=j {
                k_sub_struecure.insert(k, 0.0);
            }
            energy_eigen_values.push(k_sub_struecure);
        }

        Self {
            eigen_state: SymmetricTop::new(a, b),
            energy_eigen_values,
        }
    }

    pub fn calc_energy(&mut self) {
        for (j, k_sub_structure) in self.energy_eigen_values.iter_mut().enumerate() {
            for (k, energy_eigen_value) in k_sub_structure.iter_mut() {
                *energy_eigen_value = self.eigen_state.energy(j as i64, *k);
            }
        }
    }
}
