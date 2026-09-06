// V1 (verification): Held-out Tolman θ prediction.
//
// A genuine held-out test for the S_τ/atom universal Tolman proxy claim
// from insights #273 and #277. The tolman50.rs binary established
// ρ = -0.752 on n=50 (all classes) and ρ = -0.842 on n=43 (no phosphites).
// The concern: those 50 phosphines were curated during development, so
// any subset correlation could be overfit to the ligand selection.
//
// This binary runs a STRICTLY PRE-COMMITTED held-out set of 10 phosphines
// that were NOT used to develop or tune the descriptor. The predictions
// are made in the same Rh(CO)(H)(L) framework with no re-fitting or
// hyperparameter adjustment — whatever S_τ/atom is in the codebase now
// is what gets used.
//
// Pre-commitment (before running):
//   1. PBr₃ and PI₃ should sit between PCl₃ and PPh₃ in S_τ/atom.
//   2. The 5 para-substituted PPh₃ variants should cluster tightly
//      (std dev < 0.005 in S_τ/atom) since they all have θ = 145°.
//   3. P(sec-Bu)₃ at θ=160° should be near P(iPr)₃ and PCy₃.
//   4. Overall Spearman on n=10: ρ ≈ -0.80 if descriptor generalizes,
//      ρ ≈ -0.60 if there's latent overfit.
//
// All SMILES are Kekulé form for aromatics to match the tolman50.rs
// convention and the existing manual builders.

use thaim_core::molecular::{parse_smiles, MolGraph};
use thaim_core::molecular::{pearson_r, spearman_rho};

struct HeldoutPhosphine {
    name: &'static str,
    theta: f64,
    smiles: &'static str,
    class: &'static str,
}

fn heldout_set() -> Vec<HeldoutPhosphine> {
    vec![
        // ── Halides (Tolman 1977) ──
        HeldoutPhosphine {
            name: "PBr₃",
            theta: 131.0,
            smiles: "BrP(Br)Br",
            class: "halide",
        },
        HeldoutPhosphine {
            name: "PI₃",
            theta: 144.0,
            smiles: "IP(I)I",
            class: "halide",
        },
        // ── Para-substituted PPh₃ variants (invariance rule: para subs don't change θ=145°) ──
        HeldoutPhosphine {
            name: "P(p-F-Ph)₃",
            theta: 145.0,
            smiles: "FC1=CC=C(P(C2=CC=C(F)C=C2)C3=CC=C(F)C=C3)C=C1",
            class: "para-aryl",
        },
        HeldoutPhosphine {
            name: "P(p-Cl-Ph)₃",
            theta: 145.0,
            smiles: "ClC1=CC=C(P(C2=CC=C(Cl)C=C2)C3=CC=C(Cl)C=C3)C=C1",
            class: "para-aryl",
        },
        HeldoutPhosphine {
            name: "P(p-CF₃-Ph)₃",
            theta: 145.0,
            smiles: "FC(F)(F)C1=CC=C(P(C2=CC=C(C(F)(F)F)C=C2)C3=CC=C(C(F)(F)F)C=C3)C=C1",
            class: "para-aryl",
        },
        HeldoutPhosphine {
            name: "P(p-NMe₂-Ph)₃",
            theta: 145.0,
            smiles: "CN(C)C1=CC=C(P(C2=CC=C(N(C)C)C=C2)C3=CC=C(N(C)C)C=C3)C=C1",
            class: "para-aryl",
        },
        HeldoutPhosphine {
            name: "P(p-Br-Ph)₃",
            theta: 145.0,
            smiles: "BrC1=CC=C(P(C2=CC=C(Br)C=C2)C3=CC=C(Br)C=C3)C=C1",
            class: "para-aryl",
        },
        // ── New alkyls (Tolman 1977) ──
        HeldoutPhosphine {
            name: "P(sec-Bu)₃",
            theta: 160.0,
            smiles: "CCC(C)P(C(C)CC)C(C)CC",
            class: "alkyl",
        },
        HeldoutPhosphine {
            name: "P(n-Pent)₃",
            theta: 132.0,
            smiles: "CCCCCP(CCCCC)CCCCC",
            class: "alkyl",
        },
        // ── Meta-aryl (Tolman 1977) ──
        HeldoutPhosphine {
            name: "P(m-tol)₃",
            theta: 165.0,
            smiles: "CC1=CC=CC(P(C2=CC=CC(C)=C2)C3=CC=CC(C)=C3)=C1",
            class: "meta-aryl",
        },
    ]
}

/// Build Rh(CO)(H)(L) — identical to tolman50.rs, deliberately not
/// refactored into a shared module so that this binary is a
/// completely self-contained held-out test. Any drift between this
/// and tolman50.rs would be a red flag for the verification to catch.
fn build_complex(phosphine_smiles: &str) -> (MolGraph, usize) {
    let mut mol = MolGraph::new();
    let rh = mol.add_heavy("Rh");
    let c_co = mol.add_heavy("C");
    let o_co = mol.add_heavy("O");
    mol.add_bond(rh, c_co, 1);
    mol.add_bond(c_co, o_co, 3);
    let h_lig = mol.add_heavy("H");
    mol.add_bond(rh, h_lig, 1);

    let lig = parse_smiles(phosphine_smiles)
        .unwrap_or_else(|e| panic!("parse_smiles({}) failed: {}", phosphine_smiles, e));
    let offset = mol.n();
    for atom in &lig.atoms {
        mol.add_atom(&atom.symbol, atom.aromatic);
    }
    for bond in &lig.bonds {
        mol.add_bond(bond.a + offset, bond.b + offset, bond.order);
    }
    let p_in_lig = lig
        .atoms
        .iter()
        .position(|a| a.symbol == "P")
        .expect("phosphine SMILES has no P atom");
    let p_in_mol = p_in_lig + offset;
    mol.add_bond(rh, p_in_mol, 1);
    (mol, p_in_mol)
}

fn main() {
    println!("═══ V1 — Held-out Tolman θ prediction ═══\n");
    println!("Test set: 10 phosphines NOT used during the n=50 tolman50 development.");
    println!("No re-fitting, no hyperparameter tuning. The descriptors below are");
    println!("the exact same S_τ/atom and S_τ(P) formulas used in #273, #277.\n");

    let heldout = heldout_set();
    let tau: u32 = 5;

    println!("─────────────────────────────────────────────────────────────");
    println!(
        " {:<20} {:>5} {:>4} {:>10} {:>10} {:>12}",
        "Ligand", "θ", "n", "S_τ/atom", "S_τ(P)", "class"
    );
    println!("─────────────────────────────────────────────────────────────");

    let mut rows: Vec<(String, f64, f64, f64, &str)> = Vec::new();
    for p in &heldout {
        let (mol, p_idx) = build_complex(p.smiles);
        let n = mol.n();
        let s_tau_w = mol.s_tau(tau, true);
        let s_tau_pa = s_tau_w / n as f64;
        let s_tau_p = mol.s_tau_node(tau, p_idx, true);
        println!(
            " {:<20} {:>5.0} {:>4} {:>10.4} {:>10.4} {:>12}",
            p.name, p.theta, n, s_tau_pa, s_tau_p, p.class
        );
        rows.push((p.name.to_string(), p.theta, s_tau_pa, s_tau_p, p.class));
    }
    println!("─────────────────────────────────────────────────────────────");

    let theta_vals: Vec<f64> = rows.iter().map(|r| r.1).collect();
    let spa_vals: Vec<f64> = rows.iter().map(|r| r.2).collect();
    let sp_vals: Vec<f64> = rows.iter().map(|r| r.3).collect();

    println!(
        "\n── Held-out correlations (n = {}, τ = {}) ──",
        rows.len(),
        tau
    );
    println!(
        "   S_τ/atom vs θ : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&spa_vals, &theta_vals),
        spearman_rho(&spa_vals, &theta_vals)
    );
    println!(
        "   S_τ(P)   vs θ : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&sp_vals, &theta_vals),
        spearman_rho(&sp_vals, &theta_vals)
    );

    // Pre-commitment check 2: para-aryl cluster variance.
    let para_spa: Vec<f64> = rows
        .iter()
        .filter(|r| r.4 == "para-aryl")
        .map(|r| r.2)
        .collect();
    if para_spa.len() >= 2 {
        let mean = para_spa.iter().sum::<f64>() / para_spa.len() as f64;
        let var = para_spa.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / para_spa.len() as f64;
        let std = var.sqrt();
        println!("\n── Para-substituent invariance test ──");
        println!(
            "   {} para-subs at θ=145°, S_τ/atom mean = {:.4}, std = {:.4}",
            para_spa.len(),
            mean,
            std
        );
        println!("   Pre-commitment: std < 0.005 means descriptor is invariant");
        println!("                   to para chemistry (the physical expectation).");
        if std < 0.005 {
            println!("   VERDICT: ✓ descriptor is para-invariant as expected.");
        } else if std < 0.010 {
            println!("   VERDICT: ~ mild para-sensitivity (std = {:.4}).", std);
        } else {
            println!(
                "   VERDICT: ✗ descriptor is over-sensitive to para chemistry (std = {:.4}).",
                std
            );
        }
    }

    // Pre-commitment check 1: PBr3 and PI3 between PCl3 and PPh3.
    // PCl3 S_τ/atom at τ=5 from tolman50: 0.1749 (from tolman50 output)
    // PPh3 S_τ/atom at τ=5 from tolman50: 0.0656
    // So PBr3 and PI3 should be in [0.0656, 0.1749].
    let pbr3 = rows.iter().find(|r| r.0 == "PBr₃").map(|r| r.2);
    let pi3 = rows.iter().find(|r| r.0 == "PI₃").map(|r| r.2);
    println!("\n── Halide interpolation test ──");
    println!("   Reference (from tolman50 training set):");
    println!("     PCl₃ S_τ/atom = 0.1749 (θ=124°)");
    println!("     PPh₃ S_τ/atom = 0.0656 (θ=145°)");
    if let (Some(br), Some(i)) = (pbr3, pi3) {
        println!(
            "   PBr₃ = {:.4}  {}",
            br,
            if br >= 0.0656 && br <= 0.1749 {
                "✓ in range"
            } else {
                "✗ out of range"
            }
        );
        println!(
            "   PI₃  = {:.4}  {}",
            i,
            if i >= 0.0656 && i <= 0.1749 {
                "✓ in range"
            } else {
                "✗ out of range"
            }
        );
    }

    // Pre-commitment check 4: overall Spearman threshold.
    println!("\n── Pre-commitment threshold check ──");
    let rho = spearman_rho(&spa_vals, &theta_vals);
    println!("   Predicted: ρ ≈ -0.80 if generalizes, ρ ≈ -0.60 if overfit");
    println!("   Actual:    ρ = {:+.3}", rho);
    if rho.abs() >= 0.80 {
        println!("   VERDICT: ✓✓ PASSES strong generalization threshold");
    } else if rho.abs() >= 0.65 {
        println!("   VERDICT: ~ partial generalization (between predictions)");
    } else if rho.abs() >= 0.50 {
        println!("   VERDICT: ⚠ weak — closer to overfit prediction");
    } else {
        println!("   VERDICT: ✗ FAILS both pre-committed thresholds");
    }
}
