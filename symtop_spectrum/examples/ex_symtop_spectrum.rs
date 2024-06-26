use std::error::Error;
use std::fs::File;
use std::io::Write;
use symtop_spectrum::symtop::SymtopSpectrum;

fn out_csv(xy_data: &Vec<(f64, f64)>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_name)?;

    for (x, y) in xy_data {
        writeln!(file, "{x},{y}")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    let mut spec_1 = SymtopSpectrum::new(20, 300.0, 1.5, 1.1, 1.45, 1.06, 0.0, 0.0, 1.0);
    spec_1.calc_spectrum();
    out_csv(&spec_1.spectrum, &argv[1])?;

    Ok(())
}
