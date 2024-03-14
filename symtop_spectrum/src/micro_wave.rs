use eigen_state::EnergyManifold;
use transition::TransitionSymTop;
use wigner_j::utl::Binomial;

#[derive(Debug)]
pub struct SymtopSpectrum {
    j_max: i64,
    pub spectrum: Vec<(f64, f64)>,
    energy_manifold: EnergyManifold,
    transition: TransitionSymTop,
}

impl SymtopSpectrum {
    pub fn new(j_max: i64, a: f64, b: f64, mu_x: f64, mu_y: f64, mu_z: f64) -> Self {
        Self {
            j_max,
            spectrum: vec![],
            energy_manifold: EnergyManifold::new(j_max, a, b),
            transition: TransitionSymTop::new(mu_x, mu_y, mu_z),
        }
    }

    pub fn calc_spectrum(&mut self) {
        self.energy_manifold.calc_energy();
        let mut binomial_list = Binomial::new(3 * self.j_max);

        for j_gound in 0..=self.j_max {
            for (k_ground, energy_k_ground) in
                self.energy_manifold.energy_eigen_values[j_gound as usize].iter()
            {
                for delta_j in [0, 1, -1] {
                    if let Some(k_sub_structure_excited) = self
                        .energy_manifold
                        .energy_eigen_values
                        .get((j_gound + delta_j) as usize)
                    {
                        for (k_excited, energy_k_excited) in k_sub_structure_excited {
                            if *k_excited == *k_ground {
                                let delta_e = energy_k_excited - energy_k_ground;

                                let (k_b, temp) = (1.380649e-23, 300.0);
                                // ch x 10^2
                                let convert_const = 299792458.0 * 6.62607015e-34 * 1.0e2;
                                let (p_ground, p_excited) = (
                                    ((-energy_k_ground * convert_const) / (k_b * temp)).exp(),
                                    ((-energy_k_excited * convert_const) / (k_b * temp)).exp(),
                                );
                                let boltzman_factor = (p_ground - p_excited).abs();

                                let intensity = self.transition.intensity(
                                    j_gound,
                                    j_gound + delta_j,
                                    *k_ground,
                                    *k_excited,
                                    &mut binomial_list,
                                ) * boltzman_factor;

                                self.spectrum.push((delta_e, intensity));
                            }
                        }
                    }
                }
            }
        }
    }
}
