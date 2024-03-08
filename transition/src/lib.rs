use wigner_j::cg_coefficient::Wigner3j;
use wigner_j::utl::Binomial;

#[derive(Debug)]
struct TreeJList {
    pub three_j_1: Wigner3j,
    pub three_j_2: Wigner3j,
    pub three_j_3: Wigner3j,
}

impl TreeJList {
    pub fn new() -> Self {
        Self {
            three_j_1: Wigner3j::new(0, 0, 0, 0, 0, 0),
            three_j_2: Wigner3j::new(0, 0, 0, 0, 0, 0),
            three_j_3: Wigner3j::new(0, 0, 0, 0, 0, 0),
        }
    }
}

#[derive(Debug)]
pub struct TransitionSymTop {
    pub mu_x: f64,
    pub mu_y: f64,
    pub mu_z: f64,
    tree_j_list: TreeJList,
}

impl TransitionSymTop {
    pub fn new(mu_x: f64, mu_y: f64, mu_z: f64) -> Self {
        Self {
            mu_x,
            mu_y,
            mu_z,
            tree_j_list: TreeJList::new(),
        }
    }

    pub fn can_transition_j(&self, j_ground: i64, j_excited: i64) -> bool {
        let abs_delta_j = (j_excited - j_ground).abs();

        if abs_delta_j == 0 && abs_delta_j == 1 {
            return true;
        }

        false
    }

    pub fn can_transition_k(&self, k_ground: i64, k_excited: i64) -> bool {
        let abs_delta_k = (k_excited - k_ground).abs();

        if (self.mu_x != 0.0 && self.mu_y != 0.0 && abs_delta_k == 1)
            || (self.mu_z != 0.0 && abs_delta_k == 0)
        {
            return true;
        }

        false
    }

    pub fn intensity(
        &mut self,
        j_ground: i64,
        j_excited: i64,
        k_ground: i64,
        k_excited: i64,
        binomial_list: &mut Binomial,
    ) -> f64 {
        if !self.can_transition_j(j_ground, j_excited)
            || !self.can_transition_k(k_ground, k_excited)
        {
            return 0.0;
        }

        let coefficient = ((2 * j_excited + 1) * (2 * j_ground + 1)) as f64 / 3.0;

        self.tree_j_list
            .three_j_1
            .update_jm(1, -1, j_ground, -k_ground, j_excited, k_excited);
        self.tree_j_list
            .three_j_2
            .update_jm(1, 1, j_ground, -k_ground, j_excited, k_excited);
        self.tree_j_list
            .three_j_3
            .update_jm(1, 0, j_ground, -k_ground, j_excited, k_excited);

        let three_j_1 = self.tree_j_list.three_j_1.wigner_3j(binomial_list);
        let three_j_2 = self.tree_j_list.three_j_2.wigner_3j(binomial_list);
        let three_j_3 = self.tree_j_list.three_j_3.wigner_3j(binomial_list);

        let int_value = coefficient
            * (self.mu_x * (three_j_1 - three_j_2)
                + self.mu_y * (three_j_1 + three_j_2)
                + self.mu_z * three_j_3)
                .powi(2);

        int_value
    }
}
