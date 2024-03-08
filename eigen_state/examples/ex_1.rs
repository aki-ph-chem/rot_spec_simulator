use eigen_state::{EnergyManifold, SymmetricTop};

fn main() {
    let sym_top_1 = SymmetricTop::new(1.2, 1.0);
    println!("sym_top_1.energy(1,-1): {}", sym_top_1.energy(1, -1));

    let mut sym_top_energy = EnergyManifold::new(3, 1.2, 1.0);
    sym_top_energy.calc_energy();
    println!("sym_top_energy: {:#?}", sym_top_energy);
}
