use crate::utl;
use utl::Binomial;

#[derive(Debug)]
pub struct CGCoefficient {
    j_1: i64,
    m_1: i64,
    j_2: i64,
    m_2: i64,
    j_3: i64,
    m_3: i64,
}

impl CGCoefficient {
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3: i64) -> Self {
        Self {
            j_1,
            m_1,
            j_2,
            m_2,
            j_3,
            m_3,
        }
    }

    pub fn show_jm_list(&self) {
        println!(
            "<{},{},{},{}|{},{} >",
            self.j_1, self.m_1, self.j_2, self.m_2, self.j_3, self.m_3
        );
    }

    pub fn update_jm(&mut self, j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3: i64) {
        self.j_1 = j_1;
        self.m_1 = m_1;
        self.j_2 = j_2;
        self.m_2 = m_2;
        self.j_3 = j_3;
        self.m_3 = m_3;
    }

    pub fn cg_coefficient(&self, binomials: &mut Binomial) -> f64 {
        utl::calc_cg_binomial_raw(
            self.j_1, self.j_2, self.j_3, self.m_1, self.m_2, self.m_3, binomials,
        )
    }
}

#[derive(Debug)]
pub struct Wigner3j {
    j_1: i64,
    m_1: i64,
    j_2: i64,
    m_2: i64,
    j_3: i64,
    m_3: i64,
}

impl Wigner3j {
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3: i64) -> Self {
        Self {
            j_1,
            m_1,
            j_2,
            m_2,
            j_3,
            m_3,
        }
    }

    pub fn show_jm_list(&self) {
        println!(
            "( {} ,{}, {} )\n( {}, {}, {} ) ",
            self.j_1, self.j_2, self.j_3, self.m_1, self.m_2, self.m_3
        );
    }

    pub fn update_jm(&mut self, j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3: i64) {
        self.j_1 = j_1;
        self.m_1 = m_1;
        self.j_2 = j_2;
        self.m_2 = m_2;
        self.j_3 = j_3;
        self.m_3 = m_3;
    }

    pub fn wigner_3j(&self, binomials: &mut Binomial) -> f64 {
        (utl::sign((self.j_1 - self.j_2 - self.m_3) as i32)
            / ((2 * self.j_3) as f64 + 1.0).powf(0.5))
            * utl::calc_cg_binomial_raw(
                self.j_1, self.j_2, self.j_3, self.m_1, self.m_2, -self.m_3, binomials,
            )
    }
}
