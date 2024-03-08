use eigen_state::EnergyManifold;
use transition::TransitionSymTop;
use wigner_j::utl::Binomial;

#[derive(Debug)]
pub struct SymtopSpectrum {
    j_max: i64,
    pub spectrum: Vec<(f64, f64)>,
    energy_manifold_ground: EnergyManifold,
    energy_manifold_excited: EnergyManifold,
    transition: TransitionSymTop,
}

impl SymtopSpectrum {
    pub fn new(
        j_max: i64,
        a_ground: f64,
        b_ground: f64,
        a_excited: f64,
        b_excited: f64,
        mu_x: f64,
        mu_y: f64,
        mu_z: f64,
    ) -> Self {
        Self {
            j_max,
            spectrum: vec![],
            energy_manifold_ground: EnergyManifold::new(j_max, a_ground, b_ground),
            energy_manifold_excited: EnergyManifold::new(j_max, a_excited, b_excited),
            transition: TransitionSymTop::new(mu_x, mu_y, mu_z),
        }
    }

    pub fn calc_spectrum(&mut self) {
        self.energy_manifold_ground.calc_energy();
        self.energy_manifold_excited.calc_energy();
        let mut binomial_list = Binomial::new(3 * self.j_max);

        for j_gound in 0..=self.j_max {
            for (k_ground, energy_k_ground) in
                self.energy_manifold_ground.energy_eigen_values[j_gound as usize].iter()
            {
                for delta_j in [0, 1, -1] {
                    for delta_k in [0, 1, -1] {
                        if let Some(k_sub_structure_excited) = self
                            .energy_manifold_excited
                            .energy_eigen_values
                            .get((j_gound + delta_j) as usize)
                        {
                            for (k_excited, energy_k_excited) in k_sub_structure_excited {
                                if *k_excited == k_ground + delta_k {
                                    let delta_e = energy_k_excited - energy_k_ground;
                                    let int = self.transition.intensity(
                                        j_gound,
                                        j_gound + delta_j,
                                        *k_ground,
                                        *k_excited,
                                        &mut binomial_list,
                                    );

                                    self.spectrum.push((delta_e, int));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
