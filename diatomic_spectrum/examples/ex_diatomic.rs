use diatomic_spectrum::{DiatomicMolecule, Spectrum};
use line_shape::{convolute_line_shape_function, LineShape};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn out_csv(x_data: &Vec<f64>, y_data: &Vec<f64>, path_to_file: &str) -> Result<(), Box<dyn Error>> {
    if x_data.len() != y_data.len() {
        panic!("Error: length of two vectors in not equal");
    }

    let mut file = File::create(path_to_file)?;

    for (x, y) in x_data.iter().zip(y_data.iter()) {
        writeln!(file, "{x},{y}")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        eprint!("Error: invalid args");
        std::process::exit(1);
    }

    let mol_1 = DiatomicMolecule::new(2.0);
    let spec_1 = Spectrum::new(300.0, 30, mol_1).calc_spectrum();

    let line_profile = LineShape::new(0.04);
    let spectrum_1 =
        convolute_line_shape_function(0.0, 120.0, 0.01, &line_profile, (&spec_1.0, &spec_1.1));

    out_csv(&spectrum_1.0, &spectrum_1.1, &argv[1])?;

    Ok(())
}
