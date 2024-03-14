pub struct Range {
    x_current: f64,
    x_fin: f64,
    step: f64,
}

impl Range {
    pub fn new(x_ini: f64, x_fin: f64, step: f64) -> Self {
        Self {
            x_current: x_ini,
            x_fin,
            step,
        }
    }
}

impl Iterator for Range {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_current <= self.x_fin {
            let value = self.x_current;
            self.x_current += self.step;
            Some(value)
        } else {
            None
        }
    }
}

pub struct LineShape {
    width: f64,
}

impl LineShape {
    pub fn new(width: f64) -> Self {
        Self { width }
    }

    pub fn lorentz(&self, x: f64, x_centor: f64) -> f64 {
        let pi = std::f64::consts::PI;
        (self.width / 2.0 * pi) / ((x - x_centor).powi(2) + (self.width / 2.0).powi(2))
    }

    pub fn gauss(&self, x: f64, x_centor: f64) -> f64 {
        let pi = std::f64::consts::PI;
        let coeff = ((2.0_f64).ln() / pi).sqrt() / (self.width / 2.0);

        coeff * (-(x - x_centor).powi(2) / (self.width / 2.0)).exp()
    }
}

pub fn convolute_line_shape_function(
    x_ini: f64,
    x_fin: f64,
    x_step: f64,
    line_profile: &LineShape,
    raw_signal: (&Vec<f64>, &Vec<f64>),
) -> (Vec<f64>, Vec<f64>) {
    let x_signal: Vec<f64> = Range::new(x_ini, x_fin, x_step).collect();
    let mut y_signal = vec![0.0; x_signal.len()];

    for i in 0..raw_signal.0.len() {
        for j in 0..x_signal.len() {
            y_signal[j] += raw_signal.1[i] * line_profile.lorentz(x_signal[j], raw_signal.0[i]);
        }
    }

    (x_signal, y_signal)
}

pub fn convolute_line_shape_function_tup(
    x_ini: f64,
    x_fin: f64,
    x_step: f64,
    line_profile: &LineShape,
    raw_signal: &Vec<(f64, f64)>,
) -> (Vec<f64>, Vec<f64>) {
    let x_signal: Vec<f64> = Range::new(x_ini, x_fin, x_step).collect();
    let mut y_signal = vec![0.0; x_signal.len()];

    for i in 0..raw_signal.len() {
        for j in 0..x_signal.len() {
            y_signal[j] += raw_signal[i].1 * line_profile.lorentz(x_signal[j], raw_signal[i].0);
        }
    }

    (x_signal, y_signal)
}
