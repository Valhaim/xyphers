// Tolman-50 (C3): expand the n=20 hydroformylation result (insight #273) to
// n=50 phosphines using SMILES-based ligand construction.
//
// The n=20 hydroformylation_n20.rs binary used hand-written graph builders for
// 20 phosphines. Insight #273 found:
//   - S_τ/atom universal Tolman proxy: ρ = -0.875 (n=20)
//   - S_τ(P)_lig alkyl-class precision: ρ = 0.967 (n=9 alkyl subset)
//
// C3 goal: extend to n≥50 to lock in the universal claim at publishable n.
// Method: encode every phosphine as a SMILES string (Kekulé form for aromatics
// to match the existing n=20 convention exactly — verified via parity check).
// Build the same Rh(CO)(H)(L) complex per phosphine, compute the same
// descriptors, run the same correlation analysis, including subset breakdowns.
//
// Sources for cone angles:
//   - Tolman, C.A. Chem. Rev. 1977, 77, 313 (the canonical Table I)
//   - Compilations in van Leeuwen "Homogeneous Catalysis" (2004)
//   - Crabtree "Organometallic Chemistry of the Transition Metals" (2019 ed.)
//
// All values are widely cited. Where multiple measurements disagree, I use
// the median Tolman 1977 value or the most-commonly-cited follow-up.

use thaim_core::molecular::{parse_smiles, MolGraph};
use thaim_core::molecular::{pearson_r, spearman_rho};

#[derive(Copy, Clone, PartialEq)]
enum Class {
    Alkyl,
    Aryl,
    MixedAlkylAryl,
    OrthoAryl,
    Phosphite,
    Halide,
    AminoPhos,  // P-N bonds (P(NR2)3 family)
    HeteroAryl, // furyl, thienyl
    Fluoroaryl, // perfluorophenyl
}

struct Phosphine {
    name: &'static str,
    theta: f64, // Tolman cone angle (degrees)
    smiles: &'static str,
    class: Class,
}

fn ligand_table() -> Vec<Phosphine> {
    use Class::*;
    vec![
        // ── Original n=20 set (insight #273) ──
        Phosphine {
            name: "PH₃",
            theta: 87.0,
            smiles: "P",
            class: Alkyl,
        },
        Phosphine {
            name: "PF₃",
            theta: 104.0,
            smiles: "FP(F)F",
            class: Halide,
        },
        Phosphine {
            name: "P(OMe)₃",
            theta: 107.0,
            smiles: "COP(OC)OC",
            class: Phosphite,
        },
        Phosphine {
            name: "P(OEt)₃",
            theta: 109.0,
            smiles: "CCOP(OCC)OCC",
            class: Phosphite,
        },
        Phosphine {
            name: "PMe₃",
            theta: 118.0,
            smiles: "CP(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PMe₂Ph",
            theta: 122.0,
            smiles: "CP(C)C1=CC=CC=C1",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "PCl₃",
            theta: 124.0,
            smiles: "ClP(Cl)Cl",
            class: Halide,
        },
        Phosphine {
            name: "P(OPh)₃",
            theta: 128.0,
            smiles: "C1=CC=C(OP(OC2=CC=CC=C2)OC3=CC=CC=C3)C=C1",
            class: Phosphite,
        },
        Phosphine {
            name: "PEt₃",
            theta: 132.0,
            smiles: "CCP(CC)CC",
            class: Alkyl,
        },
        Phosphine {
            name: "PBu₃",
            theta: 132.0,
            smiles: "CCCCP(CCCC)CCCC",
            class: Alkyl,
        },
        Phosphine {
            name: "PMePh₂",
            theta: 136.0,
            smiles: "CP(C1=CC=CC=C1)C2=CC=CC=C2",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "P(iBu)₃",
            theta: 143.0,
            smiles: "CC(C)CP(CC(C)C)CC(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PPh₃",
            theta: 145.0,
            smiles: "C1=CC=CC=C1P(C2=CC=CC=C2)C3=CC=CC=C3",
            class: Aryl,
        },
        Phosphine {
            name: "P(iPr)₃",
            theta: 160.0,
            smiles: "CC(C)P(C(C)C)C(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "P(tBu)₂Me",
            theta: 161.0,
            smiles: "CP(C(C)(C)C)C(C)(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PBn₃",
            theta: 165.0,
            smiles: "C1=CC=C(CP(CC2=CC=CC=C2)CC3=CC=CC=C3)C=C1",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "PCy₃",
            theta: 170.0,
            smiles: "C1CCC(P(C2CCCCC2)C3CCCCC3)CC1",
            class: Alkyl,
        },
        Phosphine {
            name: "P(tBu)₃",
            theta: 182.0,
            smiles: "CC(C)(C)P(C(C)(C)C)C(C)(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "P(o-tol)₃",
            theta: 194.0,
            smiles: "CC1=CC=CC=C1P(C2=CC=CC=C2C)C3=CC=CC=C3C",
            class: OrthoAryl,
        },
        Phosphine {
            name: "P(mesityl)₃",
            theta: 212.0,
            smiles: "CC1=CC(C)=CC(C)=C1P(C2=C(C)C=C(C)C=C2C)C3=C(C)C=C(C)C=C3C",
            class: OrthoAryl,
        },
        // ── New additions (Tolman 1977 + canonical follow-ups) ──
        // Alkyl extensions
        Phosphine {
            name: "PMe₂Et",
            theta: 123.0,
            smiles: "CCP(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PMeEt₂",
            theta: 127.0,
            smiles: "CCP(C)CC",
            class: Alkyl,
        },
        Phosphine {
            name: "PPr₃",
            theta: 132.0,
            smiles: "CCCP(CCC)CCC",
            class: Alkyl,
        },
        Phosphine {
            name: "PMe₂(iPr)",
            theta: 132.0,
            smiles: "CC(C)P(C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PMe(iPr)₂",
            theta: 142.0,
            smiles: "CC(C)P(C(C)C)C",
            class: Alkyl,
        },
        Phosphine {
            name: "PCyMe₂",
            theta: 137.0,
            smiles: "CP(C)C1CCCCC1",
            class: Alkyl,
        },
        Phosphine {
            name: "PCy₂Me",
            theta: 159.0,
            smiles: "CP(C1CCCCC1)C2CCCCC2",
            class: Alkyl,
        },
        Phosphine {
            name: "P(c-Pent)₃",
            theta: 170.0,
            smiles: "C1CCC(P(C2CCCC2)C3CCCC3)C1",
            class: Alkyl,
        },
        Phosphine {
            name: "P(neoPent)₃",
            theta: 180.0,
            smiles: "CC(C)(C)CP(CC(C)(C)C)CC(C)(C)C",
            class: Alkyl,
        },
        // Mixed alkyl/aryl extensions
        Phosphine {
            name: "PEt₂Ph",
            theta: 136.0,
            smiles: "CCP(CC)C1=CC=CC=C1",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "PEtPh₂",
            theta: 140.0,
            smiles: "CCP(C1=CC=CC=C1)C2=CC=CC=C2",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "PCyPh₂",
            theta: 148.0,
            smiles: "C1CCC(P(C2=CC=CC=C2)C3=CC=CC=C3)CC1",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "PCy₂Ph",
            theta: 153.0,
            smiles: "C1CCC(P(C2CCCCC2)C3=CC=CC=C3)CC1",
            class: MixedAlkylAryl,
        },
        // Aryl with para substituents (don't change cone angle, expected ~145°)
        Phosphine {
            name: "P(p-tol)₃",
            theta: 145.0,
            smiles: "CC1=CC=C(P(C2=CC=C(C)C=C2)C3=CC=C(C)C=C3)C=C1",
            class: Aryl,
        },
        Phosphine {
            name: "P(p-MeO-Ph)₃",
            theta: 145.0,
            smiles: "COC1=CC=C(P(C2=CC=C(OC)C=C2)C3=CC=C(OC)C=C3)C=C1",
            class: Aryl,
        },
        // Fluoroaryl
        Phosphine {
            name: "P(C₆F₅)₃",
            theta: 184.0,
            smiles: "FC1=C(F)C(F)=C(P(C2=C(F)C(F)=C(F)C(F)=C2F)C3=C(F)C(F)=C(F)C(F)=C3F)C(F)=C1F",
            class: Fluoroaryl,
        },
        // Heteroaryls
        Phosphine {
            name: "P(2-furyl)₃",
            theta: 133.0,
            smiles: "C1=CC(=CO1)P(C2=CC=CO2)C3=CC=CO3",
            class: HeteroAryl,
        },
        // Aminophosphines (P-N bonds)
        Phosphine {
            name: "P(NMe₂)₃",
            theta: 157.0,
            smiles: "CN(C)P(N(C)C)N(C)C",
            class: AminoPhos,
        },
        // Phosphite extensions
        Phosphine {
            name: "P(OPr)₃",
            theta: 130.0,
            smiles: "CCCOP(OCCC)OCCC",
            class: Phosphite,
        },
        Phosphine {
            name: "P(O-iPr)₃",
            theta: 130.0,
            smiles: "CC(C)OP(OC(C)C)OC(C)C",
            class: Phosphite,
        },
        Phosphine {
            name: "P(O-nBu)₃",
            theta: 131.0,
            smiles: "CCCCOP(OCCCC)OCCCC",
            class: Phosphite,
        },
        Phosphine {
            name: "P(O-tBu)₃",
            theta: 175.0,
            smiles: "CC(C)(C)OP(OC(C)(C)C)OC(C)(C)C",
            class: Phosphite,
        },
        // More mixed steric
        Phosphine {
            name: "P(Ph)(tBu)₂",
            theta: 170.0,
            smiles: "CC(C)(C)P(C(C)(C)C)C1=CC=CC=C1",
            class: MixedAlkylAryl,
        },
        Phosphine {
            name: "P(iPr)(tBu)₂",
            theta: 175.0,
            smiles: "CC(C)P(C(C)(C)C)C(C)(C)C",
            class: Alkyl,
        },
        // Mixed ortho-aryl
        Phosphine {
            name: "P(o-anisyl)₃",
            theta: 195.0,
            smiles: "COC1=CC=CC=C1P(C2=CC=CC=C2OC)C3=CC=CC=C3OC",
            class: OrthoAryl,
        },
        // Even bigger
        Phosphine {
            name: "P(1-Naphthyl)₃",
            theta: 210.0,
            smiles: "C1=CC=C2C(=C1)C=CC=C2P(C3=CC=CC4=CC=CC=C34)C5=CC=CC6=CC=CC=C56",
            class: OrthoAryl,
        },
        // Aminophos extensions
        Phosphine {
            name: "PMe(NMe₂)₂",
            theta: 132.0,
            smiles: "CN(C)P(C)N(C)C",
            class: AminoPhos,
        },
        Phosphine {
            name: "P(NMe₂)Me₂",
            theta: 122.0,
            smiles: "CP(C)N(C)C",
            class: AminoPhos,
        },
        Phosphine {
            name: "PEt(NMe₂)₂",
            theta: 138.0,
            smiles: "CCP(N(C)C)N(C)C",
            class: AminoPhos,
        },
        // 50th — heteroaryl thiophene (Allman & Goel 1982)
        Phosphine {
            name: "P(2-thienyl)₃",
            theta: 133.0,
            smiles: "C1=CC(=CS1)P(C2=CC=CS2)C3=CC=CS3",
            class: HeteroAryl,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// Complex construction (parse SMILES, attach to Rh fragment)
// ═══════════════════════════════════════════════════════════════════════════

/// Build Rh(CO)(H)(L). Returns (mol, p_index_in_complex, n_lig_atoms).
fn build_complex(phosphine_smiles: &str) -> (MolGraph, usize, usize) {
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
    let lig_n = lig.n();
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
    (mol, p_in_mol, lig_n)
}

/// Extract the ligand-only subgraph: BFS from P avoiding the metal at index 0.
/// Returns (lig_graph, p_index_in_lig).
fn extract_ligand(mol: &MolGraph, p: usize, metal: usize) -> (MolGraph, usize) {
    let n = mol.n();
    let mut in_set = vec![false; n];
    let mut queue = vec![p];
    in_set[p] = true;
    let mut i = 0;
    while i < queue.len() {
        let cur = queue[i];
        i += 1;
        for bond in &mol.bonds {
            let neighbour = if bond.a == cur {
                bond.b
            } else if bond.b == cur {
                bond.a
            } else {
                continue;
            };
            if neighbour == metal || in_set[neighbour] {
                continue;
            }
            in_set[neighbour] = true;
            queue.push(neighbour);
        }
    }
    let mut new_mol = MolGraph::new();
    let mut old_to_new = vec![usize::MAX; n];
    for (old_idx, keep) in in_set.iter().enumerate() {
        if *keep {
            let atom = &mol.atoms[old_idx];
            let new_idx = new_mol.add_atom(&atom.symbol, atom.aromatic);
            old_to_new[old_idx] = new_idx;
        }
    }
    for bond in &mol.bonds {
        let a = old_to_new[bond.a];
        let b = old_to_new[bond.b];
        if a != usize::MAX && b != usize::MAX {
            new_mol.add_bond(a, b, bond.order);
        }
    }
    (new_mol, old_to_new[p])
}

// ═══════════════════════════════════════════════════════════════════════════
// Measurement
// ═══════════════════════════════════════════════════════════════════════════

#[allow(dead_code)]
struct Measurement {
    name: &'static str,
    theta: f64,
    class: Class,
    n_atoms: usize,
    s_tau_w: f64,
    s_tau_pa: f64,
    s_tau_p: f64,
    n_lig: usize,
    s_tau_p_lig: f64,
    s_tau_pa_lig: f64,
}

fn measure_one(p: &Phosphine, tau: u32) -> Measurement {
    let (mol, p_idx, _lig_n) = build_complex(p.smiles);
    let n = mol.n();
    let s_tau_w = mol.s_tau(tau, true);
    let s_tau_pa = s_tau_w / n as f64;
    let s_tau_p = mol.s_tau_node(tau, p_idx, true);

    let (lig_mol, p_lig) = extract_ligand(&mol, p_idx, 0);
    let n_lig = lig_mol.n();
    let s_tau_w_lig = lig_mol.s_tau(tau, true);
    let s_tau_pa_lig = s_tau_w_lig / n_lig as f64;
    let s_tau_p_lig = lig_mol.s_tau_node(tau, p_lig, true);

    Measurement {
        name: p.name,
        theta: p.theta,
        class: p.class,
        n_atoms: n,
        s_tau_w,
        s_tau_pa,
        s_tau_p,
        n_lig,
        s_tau_p_lig,
        s_tau_pa_lig,
    }
}

fn correlations(label: &str, subset: &[&Measurement]) {
    if subset.len() < 4 {
        return;
    }
    let theta_vals: Vec<f64> = subset.iter().map(|m| m.theta).collect();
    let spa: Vec<f64> = subset.iter().map(|m| m.s_tau_pa).collect();
    let sp: Vec<f64> = subset.iter().map(|m| m.s_tau_p).collect();
    let spl: Vec<f64> = subset.iter().map(|m| m.s_tau_p_lig).collect();
    let spal: Vec<f64> = subset.iter().map(|m| m.s_tau_pa_lig).collect();
    println!(
        "   {:<28} n={:<3}  S/at={:+.3}  S(P)={:+.3}  S(P)_lig={:+.3}  S/at_lig={:+.3}",
        label,
        subset.len(),
        spearman_rho(&spa, &theta_vals),
        spearman_rho(&sp, &theta_vals),
        spearman_rho(&spl, &theta_vals),
        spearman_rho(&spal, &theta_vals)
    );
}

fn main() {
    println!("═══ Tolman-50 — S_τ vs Tolman cone angle on n≥50 phosphines ═══\n");

    let phosphines = ligand_table();
    println!("  Loaded {} phosphines from SMILES.", phosphines.len());
    println!(
        "  Range: θ = {:.0}° to {:.0}°",
        phosphines
            .iter()
            .map(|p| p.theta)
            .fold(f64::INFINITY, f64::min),
        phosphines
            .iter()
            .map(|p| p.theta)
            .fold(f64::NEG_INFINITY, f64::max)
    );
    println!();

    let tau: u32 = 5;
    let mut measurements: Vec<Measurement> =
        phosphines.iter().map(|p| measure_one(p, tau)).collect();
    measurements.sort_by(|a, b| a.theta.partial_cmp(&b.theta).unwrap());

    println!("─────────────────────────────────────────────────────────────────────────────");
    println!(
        " {:<18} {:>5} {:>4} {:>5} {:>9} {:>9} {:>9} {:>9}",
        "Ligand", "θ", "n", "n_lig", "S/at", "S(P)", "S/at_lig", "S(P)_lig"
    );
    println!("─────────────────────────────────────────────────────────────────────────────");
    for m in &measurements {
        let disp = if m.name.len() > 18 {
            &m.name[..18]
        } else {
            m.name
        };
        println!(
            " {:<18} {:>5.0} {:>4} {:>5} {:>9.4} {:>9.4} {:>9.4} {:>9.4}",
            disp, m.theta, m.n_atoms, m.n_lig, m.s_tau_pa, m.s_tau_p, m.s_tau_pa_lig, m.s_tau_p_lig
        );
    }
    println!("─────────────────────────────────────────────────────────────────────────────");

    // Aggregate correlations.
    let theta_vals: Vec<f64> = measurements.iter().map(|m| m.theta).collect();
    let spa_vals: Vec<f64> = measurements.iter().map(|m| m.s_tau_pa).collect();
    let sp_vals: Vec<f64> = measurements.iter().map(|m| m.s_tau_p).collect();
    let spl_vals: Vec<f64> = measurements.iter().map(|m| m.s_tau_p_lig).collect();
    let spal_vals: Vec<f64> = measurements.iter().map(|m| m.s_tau_pa_lig).collect();
    let sw_vals: Vec<f64> = measurements.iter().map(|m| m.s_tau_w).collect();

    println!(
        "\n Correlations vs Tolman cone angle θ (n = {}, τ = {}):",
        measurements.len(),
        tau
    );
    println!(
        "   [complex]  S_τ(w)     : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&sw_vals, &theta_vals),
        spearman_rho(&sw_vals, &theta_vals)
    );
    println!(
        "   [complex]  S_τ/atom   : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&spa_vals, &theta_vals),
        spearman_rho(&spa_vals, &theta_vals)
    );
    println!(
        "   [complex]  S_τ(P)     : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&sp_vals, &theta_vals),
        spearman_rho(&sp_vals, &theta_vals)
    );
    println!(
        "   [ligand]   S_τ/at_lig : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&spal_vals, &theta_vals),
        spearman_rho(&spal_vals, &theta_vals)
    );
    println!(
        "   [ligand]   S_τ(P)_lig : r = {:+.3},  ρ = {:+.3}",
        pearson_r(&spl_vals, &theta_vals),
        spearman_rho(&spl_vals, &theta_vals)
    );

    // Subset correlations.
    println!("\n Subset correlations vs Tolman cone angle θ:");
    let all_refs: Vec<&Measurement> = measurements.iter().collect();
    let alkyl: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class == Class::Alkyl)
        .collect();
    let alkyl_no_h: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class == Class::Alkyl && m.name != "PH₃")
        .collect();
    let no_phosphites: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class != Class::Phosphite)
        .collect();
    let no_phosphites_no_halides: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class != Class::Phosphite && m.class != Class::Halide)
        .collect();
    let no_ortho: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class != Class::OrthoAryl)
        .collect();
    let no_aminophos: Vec<&Measurement> = measurements
        .iter()
        .filter(|m| m.class != Class::AminoPhos)
        .collect();

    correlations("All ligands", &all_refs);
    correlations("Alkyl only", &alkyl);
    correlations("Alkyl only (no PH₃)", &alkyl_no_h);
    correlations("No phosphites", &no_phosphites);
    correlations("No phosphites, no halides", &no_phosphites_no_halides);
    correlations("No ortho-aryls", &no_ortho);
    correlations("No aminophosphines", &no_aminophos);

    println!();
    let n = measurements.len();
    let final_full = spearman_rho(&spa_vals, &theta_vals);
    println!(" ─────────────────────────────────────────────────────────────");
    println!(" SUCCESS criteria (from C3 task spec):");
    println!(
        "   ρ(S_τ/atom vs θ) ≥ 0.80 on n≥50: {}",
        if n >= 50 && final_full.abs() >= 0.80 {
            "PASS ✓"
        } else {
            "see below"
        }
    );
    println!(
        "   ρ(S_τ/atom vs θ) ≥ 0.85 on n≥50: {}",
        if n >= 50 && final_full.abs() >= 0.85 {
            "BREAKTHROUGH ★★"
        } else {
            "see below"
        }
    );
    println!(
        "   Falsifier ρ < 0.75 on n≥50: {}",
        if n >= 50 && final_full.abs() < 0.75 {
            "TRIGGERED ✗"
        } else {
            "no"
        }
    );
    println!(" ─────────────────────────────────────────────────────────────");
    println!(
        " Universal descriptor S_τ/atom: ρ = {:+.3} (n = {})",
        final_full, n
    );
}
