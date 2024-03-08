use symtop_spectrum::SymtopSpectrum;

fn main() {
    // gound state: A = 10.0, B = 1.0
    // excited state: A = 10.0, B = 1.0
    // j_max = 3
    let mut spec_1 = SymtopSpectrum::new(10, 10.0, 1.0, 10.0, 1.0, 1.0, 1.0, 1.0);
    spec_1.calc_spectrum();
    let result = spec_1.spectrum;

    for (x, y) in result {
        println!("{x}, {y}");
    }
}
