use transition::TransitionSymTop;
use wigner_j::utl::Binomial;

fn main() {
    let mut binomial_list = Binomial::new(5 * 3);
    let mut sym_top_rot_spectrum = TransitionSymTop::new(1.0, 1.2, 1.5);

    let mut int_transiton = |j_ground, j_excited, k_ground, k_excite| {
        sym_top_rot_spectrum.intensity(j_ground, j_excited, k_ground, k_excite, &mut binomial_list)
    };

    // j 0 -> 1, k 0 -> 1
    let intensity = int_transiton(0, 1, 0, 1);
    println!("j: 0 -> 1, k: 0 -> 1, intensity = {}", intensity);

    // j 2 -> 3, k 0 -> 1
    let intensity = int_transiton(2, 3, 0, 1);
    println!("j: 2 -> 3, k: 0 -> 1, intensity = {}", intensity);

    // j 0 -> 4, k 0 -> 1
    let intensity = int_transiton(2, 5, 0, 1);
    println!("j: 2 -> 5, k: 0 -> 1, intensity = {}", intensity);
}
