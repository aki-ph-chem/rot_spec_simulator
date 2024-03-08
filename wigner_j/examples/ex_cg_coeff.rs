use wigner_j::cg_coefficient::CGCoefficient as cg;
use wigner_j::utl::Binomial;

fn main() {
    let mut binomial_list = Binomial::new(20);

    let cg_1 = cg::new(2, 2, 3, 2, 5, 4);
    cg_1.show_jm_list();
    let res_cg_1 = cg_1.cg_coefficient(&mut binomial_list);
    println!("res_cg_1 = {}", res_cg_1);

    let cg_2 = cg::new(2, 1, 1, 1, 3, 2);
    cg_2.show_jm_list();
    let res_cg_2 = cg_2.cg_coefficient(&mut binomial_list);
    println!("res_cg_2 = {}", res_cg_2);

    let cg_3 = cg::new(12, 11, 11, 11, 13, 12);
    cg_3.show_jm_list();
    let res_cg_3 = cg_3.cg_coefficient(&mut binomial_list);
    println!("res_cg_3 = {}", res_cg_3);

    let cg_4 = cg::new(6, 3, 2, 1, 8, 4);
    cg_4.show_jm_list();
    let res_cg_4 = cg_4.cg_coefficient(&mut binomial_list);
    println!("res_cg_4 = {}", res_cg_4);

    let cg_5 = cg::new(6, 0, 4, 0, 2, 0);
    cg_5.show_jm_list();
    let res_cg_5 = cg_5.cg_coefficient(&mut binomial_list);
    println!("res_cg_5 = {}", res_cg_5);

    let cg_6 = cg::new(5, 0, 4, 0, 9, 0);
    cg_6.show_jm_list();
    let res_cg_6 = cg_6.cg_coefficient(&mut binomial_list);
    println!("res_cg_6 = {}", res_cg_6);
}
