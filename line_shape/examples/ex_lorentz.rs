use line_shape::convolute_line_shape_function;
use line_shape::LineShape;
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
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    // raw signal: (with no line width)
    let x_value = vec![0.5, 1.2, 2.3, 2.4, 3.5];
    let y_value = vec![1.0, 1.6, 3.0, 1.6, 1.2];

    let lorentz_profile = LineShape::new(0.04);
    let (x_signal, y_signal) =
        convolute_line_shape_function(0.1, 4.0, 0.01, &lorentz_profile, (&x_value, &y_value));
    out_csv(&x_signal, &y_signal, &argv[1])?;

    Ok(())
}
