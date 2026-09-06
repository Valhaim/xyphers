use std::error::Error;
use thaim_core::molecular::parse_smiles;

fn main() -> Result<(), Box<dyn Error>> {
    let horizon: u32 = std::env::args()
        .nth(1)
        .ok_or("usage: cargo run --example phosphines -- <horizon>")?
        .parse()?;
    println!("Ligand-only, unweighted endpoint entropy from phosphorus; horizon={horizon}");
    println!("name\tformula\tatoms\tS_tau_bits\tS_tau_nats");
    for row in include_str!("phosphines.tsv").lines().skip(1) {
        let (name, smiles) = row.split_once('\t').ok_or("invalid molecule row")?;
        let molecule = parse_smiles(smiles)?;
        let phosphorus = molecule
            .atoms
            .iter()
            .position(|atom| atom.symbol == "P")
            .ok_or("phosphorus is absent")?;
        let bits = molecule.s_tau_node(horizon, phosphorus, false);
        println!(
            "{name}\t{}\t{}\t{bits:.9}\t{:.9}",
            molecule.formula(),
            molecule.n(),
            bits * std::f64::consts::LN_2,
        );
    }
    Ok(())
}
