use xypher_theory_of_aim_proof::evaluate;

fn main() {
    let report = evaluate();
    print!("{}", report.render());
    if !report.is_success() {
        std::process::exit(1);
    }
}
