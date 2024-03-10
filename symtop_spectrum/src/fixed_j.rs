use eigen_state::EnergyManifold;
use transition::TransitionSymTop;
use wigner_j::utl::Binomial;

pub fn calc_spectrum() -> Vec<(f64, f64)> {
    let mut result = vec![];
    // j_max
    let j_max = 3;
    // rot const A,B in ground and exited state
    let (a_ground, b_ground) = (10.0, 1.0);
    let (a_excited, b_excited) = (10.0, 1.0);
    // electronic dipole moment along x,y,z axis
    let (mu_x, mu_y, mu_z) = (1.0, 1.0, 1.0);

    let mut energy_ground = EnergyManifold::new(j_max, a_ground, b_ground);
    let mut energy_excited = EnergyManifold::new(j_max, a_excited, b_excited);
    energy_ground.calc_energy();
    energy_excited.calc_energy();

    let transition = TransitionSymTop::new(mu_x, mu_y, mu_z);

    eprintln!("energy_ground: {:#?}", energy_ground);
    eprintln!("energy_excited: {:#?}", energy_excited);

    result
}
