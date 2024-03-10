use symtop_spectrum::fixed_j;

fn main() {
    let result = fixed_j::calc_spectrum();

    for (x, y) in result {
        println!("{x}, {y}");
    }
}
