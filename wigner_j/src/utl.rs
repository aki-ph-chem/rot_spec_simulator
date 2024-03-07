pub struct Binomial {
    binomial_list: Vec<Vec<f64>>,
}

impl Binomial {
    pub fn new(j_max: i64) -> Self {
        Self {
            binomial_list: vec![vec![-1_f64; j_max as usize + 1]; j_max as usize + 1],
        }
    }

    /// function for calculate binomial
    fn calc_binomial_dp(&mut self, n: i64, k: i64) {
        for i in 0..=n {
            for j in 0..i.min(k) + 1 {
                if i == j || j == 0 {
                    self.binomial_list[i as usize][j as usize] = 1.0;
                } else {
                    self.binomial_list[i as usize][j as usize] = self.binomial_list[i as usize - 1]
                        [j as usize - 1]
                        + self.binomial_list[i as usize - 1][j as usize];
                }
            }
        }
    }

    /// get binomial
    pub fn binomial(&mut self, n: i64, k: i64) -> f64 {
        if self.binomial_list[n as usize][k as usize] < 0.0 {
            self.calc_binomial_dp(n, k);
        }

        self.binomial_list[n as usize][k as usize]
    }
}

/// function for calculate sign
/// n is even => 1.0, n is ood => -1.0
#[inline]
pub fn sign(n: i32) -> f64 {
    if n & 1 != 0 {
        -1.0
    } else {
        1.0
    }
}

/// function for check condtion of j
fn is_triangle_j(j_1: i64, j_2: i64, j_3: i64) -> bool {
    let is_j_positive = j_1 >= 0 && j_2 >= 0 && j_3 >= 0;
    let (j_max, j_min) = (j_1 + j_2, (j_2 - j_1).abs());
    let is_triangle = j_3 <= j_max && j_min <= j_3;

    is_j_positive && is_triangle
}

/// function for check conditon of j,m
fn is_triangle_jm(j_1: i64, j_2: i64, j_3: i64, m_1: i64, m_2: i64, m_3: i64) -> bool {
    let condition_jm = j_1 >= m_1.abs() && j_2 >= m_2.abs() && j_3 >= m_3.abs();
    let is_triangle = m_3 == m_1 + m_2;

    condition_jm && is_triangle
}

/// calculate Clebsch-Gordan coefficient by binomial
pub fn calc_cg_binomial_raw(
    j_1: i64,
    j_2: i64,
    j_3: i64,
    m_1: i64,
    m_2: i64,
    m_3: i64,
    binomials: &mut Binomial,
) -> f64 {
    if !is_triangle_j(j_1, j_2, j_3) || !is_triangle_jm(j_1, j_2, j_3, m_1, m_2, m_3) {
        return 0.0;
    }

    let s = (2.0 * (j_3 as f64) + 1.0)
        / ((2.0 * (j_1 as f64) + 1.0) * (2.0 * (j_2 as f64) + 1.0)).powf(0.5);

    let numerator = binomials.binomial(j_1 + j_2 + j_3 + 1, j_1 + j_2 - j_3)
        * binomials.binomial(2 * j_3, j_3 + m_3);

    let denominator = binomials.binomial(j_1 + j_2 + j_3 + 1, j_1 - j_2 + j_3)
        * binomials.binomial(j_1 + j_2 + j_3 + 1, j_2 - j_1 + j_3)
        * binomials.binomial(2 * j_1, j_1 + m_1)
        * binomials.binomial(2 * j_2, j_2 + m_2);

    let k_max = (j_1 + j_2 - j_3).min(j_1 - m_1).min(j_2 + m_2);
    let k_min = (-j_3 + j_2 - m_1).max(-j_3 + j_1 + m_2).max(0);

    let mut res_cg = 0.0;
    for k in k_min..k_max + 1 {
        res_cg += sign(k as i32)
            * binomials.binomial(j_1 + j_2 - j_3, k)
            * binomials.binomial(j_3 + m_3, j_2 + m_2 - k)
            * binomials.binomial(j_3 - m_3, j_1 - m_1 - k);
    }

    s * (numerator / denominator).powf(0.5) * res_cg
}
