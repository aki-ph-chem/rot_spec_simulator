use line_shape::{convolute_line_shape_function_tup, LineShape};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use symtop_spectrum::micro_wave::SymtopSpectrum;

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
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    let mut spec_1 = SymtopSpectrum::new(30, 300.0, 10.0, 1.0, 0.0, 0.0, 1.0);
    spec_1.calc_spectrum();

    let line_profile = LineShape::new(0.04);
    let spectrum_1 =
        convolute_line_shape_function_tup(-120.0, 120.0, 0.01, &line_profile, &spec_1.spectrum);

    let file_name_result = argv[1].clone() + ".csv";
    out_csv(&spectrum_1.0, &spectrum_1.1, &file_name_result)?;

    let file_name_log = argv[1].clone() + ".log";
    spec_1.out_line_list(&file_name_log)?;

    Ok(())
}
