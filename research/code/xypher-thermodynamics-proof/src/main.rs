use xypher_thermodynamics_proof::evaluate;

fn main() {
    let report = evaluate();
    print!("{}", report.render());
    if !report.is_success() {
        std::process::exit(1);
    }
}
