use std::io::{self, Write};

fn main() -> io::Result<()> {
    let report = xypher_endogenous_thermodynamics_proof::evaluate();
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    report.write_to(&mut output)?;
    output.flush()
}
