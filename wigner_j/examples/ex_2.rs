use wigner_j::cg_coefficient::Wigner3j as w_3j;
use wigner_j::utl::Binomial;

fn main() {
    let mut binomial_list = Binomial::new(30);

    let three_j_1 = w_3j::new(2, 2, 3, 2, 5, -4);
    three_j_1.show_jm_list();
    let value_1 = three_j_1.wigner_3j(&mut binomial_list);
    println!("tree_1: {}", value_1);

    let three_j_2 = w_3j::new(6, 0, 4, 0, 2, 0);
    three_j_2.show_jm_list();
    let value_2 = three_j_2.wigner_3j(&mut binomial_list);
    println!("tree_1: {}", value_2);

    let three_j_3 = w_3j::new(2, 1, 1, 1, 3, -2);
    three_j_3.show_jm_list();
    let value_3 = three_j_3.wigner_3j(&mut binomial_list);
    println!("tree_1: {}", value_3);
}
