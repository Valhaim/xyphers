// V4 (verification): prospective hydroformylation l:b prediction on
// bidentate diphosphines.
//
// The tolman50 training set (#273, #277) and the V1 held-out set (#278)
// were MONODENTATE phosphines. V4 tests whether S_τ(P) generalizes to a
// completely new class: BIDENTATE diphosphines with published l:b data
// from the Kranenburg/van Leeuwen (1995) and van der Veen/Kamer (1999)
// papers. None of these ligands are in any training set.
//
// For each bidentate, build Rh(CO)(H)(L) with BOTH P atoms of the
// diphosphine bonded to Rh. Compute S_τ at each P atom under the metal
// and average. Correlate against:
//   (a) Tolman-style natural bite angle β_n (sanity check)
//   (b) Published linear:branched selectivity in Rh-catalyzed
//       hydroformylation of 1-octene
//
// Success: ρ(S_τ(P)_avg vs ln l:b) ≥ 0.7 on n ≥ 8 held-out ligands.
// This would directly validate S_τ(P) as a catalysis predictor, not
// just a steric proxy.
//
// Failure modes:
//   - If S_τ(P)_avg tracks bite angle but NOT l:b, the descriptor is a
//     steric proxy that doesn't capture electronic effects.
//   - If it tracks neither, the bidentate class is out-of-domain.
//
// Data sources:
//   Kranenburg et al. Organometallics 1995, 14, 3081 (dppe/dppp/dppb/dppf)
//   van der Veen, Kamer, van Leeuwen, Angew. Chem. 1999, 38, 336 (Xantphos family)
//   Casey, Whiteker et al. JACS 1992, 114, 5535 (BISBI)

use thaim_core::molecular::{parse_smiles, MolGraph};
use thaim_core::molecular::{pearson_r, spearman_rho};

struct Diphosphine {
    name: &'static str,
    smiles: &'static str,
    bite_angle: f64, // natural bite angle β_n (degrees)
    lb_ratio: f64,   // published l:b in Rh-catalyzed 1-octene hydroformylation
}

fn diphosphines() -> Vec<Diphosphine> {
    vec![
        // ── Kranenburg/van Leeuwen 1995 ──
        // dppe: Ph₂P-CH₂-CH₂-PPh₂ (2-carbon backbone)
        Diphosphine {
            name: "dppe",
            smiles: "C1=CC=CC=C1P(C2=CC=CC=C2)CCP(C3=CC=CC=C3)C4=CC=CC=C4",
            bite_angle: 86.0,
            lb_ratio: 2.2,
        },
        // dppp: 3-carbon backbone
        Diphosphine {
            name: "dppp",
            smiles: "C1=CC=CC=C1P(C2=CC=CC=C2)CCCP(C3=CC=CC=C3)C4=CC=CC=C4",
            bite_angle: 91.0,
            lb_ratio: 4.2,
        },
        // dppb: 4-carbon backbone
        Diphosphine {
            name: "dppb",
            smiles: "C1=CC=CC=C1P(C2=CC=CC=C2)CCCCP(C3=CC=CC=C3)C4=CC=CC=C4",
            bite_angle: 98.0,
            lb_ratio: 6.7,
        },
        // dppf is excluded: 1,1'-bis(diphenylphosphino)ferrocene has an η⁵-Fe
        // backbone that SMILES cannot represent cleanly. Handled in bite_angle.rs
        // via a manual builder if needed. Not required for n=8 V4.
        // DPEphos: Ph₂P on diphenyl ether backbone
        // Ph₂P-C6H4-O-C6H4-PPh₂ (ortho on each aryl)
        Diphosphine {
            name: "DPEphos",
            smiles: "C1=CC=CC=C1P(C2=CC=CC=C2)C3=CC=CC=C3OC4=CC=CC=C4P(C5=CC=CC=C5)C6=CC=CC=C6",
            bite_angle: 102.0,
            lb_ratio: 14.0,
        },
        // ── van der Veen/Kamer 1999 + 2000 ──
        // Xantphos: 9,9-dimethylxanthene 4,5-bis(diphenylphosphine)
        // Xanthene is a tricyclic O-containing framework. Simplified Kekulé.
        Diphosphine {
            name: "Xantphos",
            smiles: "CC1(C)C2=CC=CC(P(C3=CC=CC=C3)C4=CC=CC=C4)=C2OC5=C1C=CC=C5P(C6=CC=CC=C6)C7=CC=CC=C7",
            bite_angle: 111.0,
            lb_ratio: 53.0,
        },
        // ── Casey/Whiteker 1992 ──
        // BISBI: 2,2'-bis(diphenylphosphinomethyl)-1,1'-biphenyl
        // Biphenyl with -CH₂-PPh₂ ortho on each ring
        Diphosphine {
            name: "BISBI",
            smiles: "C1=CC=CC(CP(C2=CC=CC=C2)C3=CC=CC=C3)=C1C4=CC=CC=C4CP(C5=CC=CC=C5)C6=CC=CC=C6",
            bite_angle: 122.0,
            lb_ratio: 66.0,
        },
        // ── van der Veen/Kamer 2000 (Nixantphos — N-H at the xanthene bridge) ──
        Diphosphine {
            name: "Nixantphos",
            smiles: "N1C2=CC=CC(P(C3=CC=CC=C3)C4=CC=CC=C4)=C2OC5=C1C=CC=C5P(C6=CC=CC=C6)C7=CC=CC=C7",
            bite_angle: 114.0,
            lb_ratio: 55.0,
        },
        // Sixantphos — Si(CH₃)₂ bridge instead of C(CH₃)₂ in Xantphos
        Diphosphine {
            name: "Sixantphos",
            smiles: "C[Si]1(C)C2=CC=CC(P(C3=CC=CC=C3)C4=CC=CC=C4)=C2OC5=C1C=CC=C5P(C6=CC=CC=C6)C7=CC=CC=C7",
            bite_angle: 108.0,
            lb_ratio: 35.0,
        },
    ]
}

/// Build Rh(CO)(H)(L) where L is a bidentate diphosphine. Both P atoms
/// of the ligand are bonded to Rh. Returns (mol, p1_index, p2_index).
fn build_complex(phosphine_smiles: &str) -> (MolGraph, usize, usize) {
    let mut mol = MolGraph::new();
    let rh = mol.add_heavy("Rh");
    let c_co = mol.add_heavy("C");
    let o_co = mol.add_heavy("O");
    mol.add_bond(rh, c_co, 1);
    mol.add_bond(c_co, o_co, 3);
    let h_lig = mol.add_heavy("H");
    mol.add_bond(rh, h_lig, 1);

    // Parse standalone diphosphine.
    let lig = parse_smiles(phosphine_smiles)
        .unwrap_or_else(|e| panic!("parse_smiles({}) failed: {}", phosphine_smiles, e));
    let offset = mol.n();
    for atom in &lig.atoms {
        mol.add_atom(&atom.symbol, atom.aromatic);
    }
    for bond in &lig.bonds {
        mol.add_bond(bond.a + offset, bond.b + offset, bond.order);
    }
    // Find both P atoms.
    let p_in_lig: Vec<usize> = lig
        .atoms
        .iter()
        .enumerate()
        .filter(|(_, a)| a.symbol == "P")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(p_in_lig.len(), 2, "diphosphine must have exactly 2 P atoms");
    let p1 = p_in_lig[0] + offset;
    let p2 = p_in_lig[1] + offset;
    mol.add_bond(rh, p1, 1);
    mol.add_bond(rh, p2, 1);
    (mol, p1, p2)
}

fn main() {
    println!("═══ V4 — Bidentate diphosphine hydroformylation l:b prediction ═══\n");
    println!("Test set: 8 diphosphines with published l:b data from Kranenburg/");
    println!("van Leeuwen 1995, van der Veen/Kamer 1999-2000, Casey/Whiteker 1992.");
    println!("None of these are in the tolman50 monodentate training set.\n");

    let ligs = diphosphines();
    let tau: u32 = 5;

    println!("─────────────────────────────────────────────────────────────────────────────");
    println!(
        " {:<12} {:>5} {:>5} {:>4} {:>9} {:>9} {:>9}",
        "Ligand", "β_n", "l:b", "n", "S_τ(P1)", "S_τ(P2)", "S_τ(P)_avg"
    );
    println!("─────────────────────────────────────────────────────────────────────────────");

    let mut rows: Vec<(String, f64, f64, f64, f64, f64)> = Vec::new();
    for lig in &ligs {
        let (mol, p1, p2) = build_complex(lig.smiles);
        let n = mol.n();
        let s_p1 = mol.s_tau_node(tau, p1, true);
        let s_p2 = mol.s_tau_node(tau, p2, true);
        let s_avg = 0.5 * (s_p1 + s_p2);
        println!(
            " {:<12} {:>5.0} {:>5.1} {:>4} {:>9.4} {:>9.4} {:>9.4}",
            lig.name, lig.bite_angle, lig.lb_ratio, n, s_p1, s_p2, s_avg
        );
        rows.push((
            lig.name.to_string(),
            lig.bite_angle,
            lig.lb_ratio,
            s_p1,
            s_p2,
            s_avg,
        ));
    }
    println!("─────────────────────────────────────────────────────────────────────────────");

    let beta: Vec<f64> = rows.iter().map(|r| r.1).collect();
    let lb: Vec<f64> = rows.iter().map(|r| r.2).collect();
    let lnlb: Vec<f64> = rows.iter().map(|r| r.2.ln()).collect();
    let sp1: Vec<f64> = rows.iter().map(|r| r.3).collect();
    let sp2: Vec<f64> = rows.iter().map(|r| r.4).collect();
    let savg: Vec<f64> = rows.iter().map(|r| r.5).collect();

    println!("\n── Sanity check: S_τ(P)_avg vs natural bite angle β_n ──");
    println!(
        "   r = {:+.3},  ρ = {:+.3}",
        pearson_r(&savg, &beta),
        spearman_rho(&savg, &beta)
    );

    println!("\n── Primary test: S_τ(P)_avg vs published l:b (linear) ──");
    println!(
        "   r = {:+.3},  ρ = {:+.3}",
        pearson_r(&savg, &lb),
        spearman_rho(&savg, &lb)
    );

    println!("\n── Primary test: S_τ(P)_avg vs ln(l:b) ──");
    println!(
        "   r = {:+.3},  ρ = {:+.3}",
        pearson_r(&savg, &lnlb),
        spearman_rho(&savg, &lnlb)
    );

    println!("\n── Per-P descriptors vs ln(l:b) ──");
    println!(
        "   S_τ(P1) : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&sp1, &lnlb),
        spearman_rho(&sp1, &lnlb)
    );
    println!(
        "   S_τ(P2) : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&sp2, &lnlb),
        spearman_rho(&sp2, &lnlb)
    );

    println!("\n── Bite angle vs ln(l:b) (the known correlation for comparison) ──");
    println!(
        "   r = {:+.3},  ρ = {:+.3}",
        pearson_r(&beta, &lnlb),
        spearman_rho(&beta, &lnlb)
    );

    println!();
    println!(" ─────────────────────────────────────────────────────────────");
    println!(
        " SUCCESS criterion: ρ(S_τ(P)_avg vs ln l:b) ≥ 0.70 on n = {}",
        rows.len()
    );
    let rho = spearman_rho(&savg, &lnlb);
    if rho.abs() >= 0.90 {
        println!("   VERDICT: ★★ BREAKTHROUGH (ρ = {:+.3})", rho);
    } else if rho.abs() >= 0.70 {
        println!("   VERDICT: ★ PASSES (ρ = {:+.3})", rho);
    } else if rho.abs() >= 0.50 {
        println!("   VERDICT: ~ weak signal (ρ = {:+.3})", rho);
    } else {
        println!("   VERDICT: ✗ FAILS (ρ = {:+.3})", rho);
    }
    println!(" ─────────────────────────────────────────────────────────────");
}
