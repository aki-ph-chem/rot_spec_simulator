use symtop_spectrum::SymtopSpectrum;

fn main() {
    let mut spec_1 = SymtopSpectrum::new(3, 1.2, 1.1, 1.5, 1.3, 1.0, 1.0, 1.0);
    spec_1.calc_spectrum();

    let result = spec_1.spectrum;
    println!("result: {:#?}", result);
}
