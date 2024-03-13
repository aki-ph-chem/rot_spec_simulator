#[derive(Debug)]
pub struct DiatomicMolecule {
    rot_const: f64,
}

impl DiatomicMolecule {
    pub fn new(rot_const: f64) -> Self {
        Self { rot_const }
    }

    pub fn energy(&self, j: i64) -> f64 {
        2.0 * self.rot_const * ((j as f64).powi(2) + j as f64)
    }
}

#[derive(Debug)]
pub struct Spectrum {
    temperature: f64,
    j_max: i64,
    molecule: DiatomicMolecule,
}

impl Spectrum {
    pub fn new(temperature: f64, j_max: i64, molecule: DiatomicMolecule) -> Self {
        Self {
            temperature,
            j_max,
            molecule,
        }
    }

    pub fn calc_spectrum(&self) -> (Vec<f64>, Vec<f64>) {
        let range_j = 0..=self.j_max;

        let mut raw_signal_x = vec![0.0; self.j_max as usize + 1];
        let mut partition_func = 0.0;
        for j in range_j.clone() {
            raw_signal_x[j as usize] = 2.0 * self.molecule.rot_const * (j as f64 + 1.0);
            partition_func +=
                (2.0 * j as f64 + 1.0) * (-self.molecule.energy(j) / self.temperature).exp();
        }

        let raw_signal_y = range_j
            .map(|j| {
                ((j as f64 + 1.0) * (-self.molecule.energy(j) / self.temperature).exp()
                    - (j as f64 + 2.0) * (-self.molecule.energy(j + 1) / self.temperature).exp())
                    / partition_func
            })
            .collect();

        (raw_signal_x, raw_signal_y)
    }
}
