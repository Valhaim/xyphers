//! Molecular graph S_τ — shared infrastructure for chemistry applications.
//!
//! Provides:
//! - `MolGraph` — molecular graph with atoms, bonds, weighted/unweighted S_τ
//! - `parse_smiles` — SMILES string → MolGraph parser
//! - `bond_energy` — bond dissociation energy lookup (~200 entries)
//! - Matrix exponentiation, Shannon entropy, Pearson/Spearman statistics

use std::collections::BTreeMap;
use std::fmt;

// ═══════════════════════════════════════════════════════════════
// Bond dissociation energy table (kJ/mol)
// ═══════════════════════════════════════════════════════════════

/// Look up mean bond dissociation energy for a pair of atom symbols and bond order.
/// Returns energy in kJ/mol. Falls back to reasonable defaults.
///
/// Bond orders: 1=single, 2=double, 3=triple, 10=aromatic
///
/// Sources: CRC Handbook, Luo (2007) Comprehensive Handbook of Chemical Bond
/// Energies, Simões (1998) organometallic M-L BDEs, Martinho Simões & Beauchamp.
pub fn bond_energy(sym_a: &str, sym_b: &str, order: usize) -> f64 {
    // Canonical ordering (alphabetical) for lookup
    let (a, b) = if sym_a <= sym_b {
        (sym_a, sym_b)
    } else {
        (sym_b, sym_a)
    };

    match (a, b, order) {
        // ── Carbon-X bonds ──
        ("C", "C", 1) => 346.0,
        ("C", "C", 2) => 614.0,
        ("C", "C", 3) => 839.0,
        ("C", "C", 10) => 518.0, // aromatic
        ("C", "H", 1) => 411.0,
        ("C", "N", 1) => 305.0,
        ("C", "N", 2) => 615.0,
        ("C", "N", 3) => 891.0,
        ("C", "N", 10) => 460.0, // aromatic C-N
        ("C", "O", 1) => 358.0,
        ("C", "O", 2) => 799.0,
        ("C", "O", 10) => 520.0, // aromatic C-O
        ("C", "S", 1) => 272.0,
        ("C", "S", 2) => 573.0,
        ("C", "S", 10) => 430.0, // aromatic C-S (thiophene)
        ("B", "C", 1) => 323.0,
        ("Br", "C", 1) => 276.0,
        ("C", "Cl", 1) => 339.0,
        ("C", "F", 1) => 485.0,
        ("C", "Ge", 1) => 238.0,
        ("C", "I", 1) => 240.0,
        ("C", "P", 1) => 264.0,
        ("C", "P", 2) => 513.0,
        ("C", "Se", 1) => 234.0,
        ("C", "Si", 1) => 318.0,
        ("C", "Sn", 1) => 192.0,

        // ── Hydrogen bonds ──
        // Canonical order: B, Br, Cl, F, Ge come before "H" (B, C, F, G < H).
        ("B", "H", 1) => 340.0,
        ("Br", "H", 1) => 366.0,
        ("Cl", "H", 1) => 431.0,
        ("F", "H", 1) => 568.0,
        ("Ge", "H", 1) => 289.0,
        ("H", "H", 1) => 436.0,
        ("H", "I", 1) => 298.0,
        ("H", "N", _) => 386.0,
        ("H", "O", 1) => 459.0,
        ("H", "P", 1) => 322.0,
        ("H", "S", 1) => 363.0,
        ("H", "Se", 1) => 305.0,
        ("H", "Si", 1) => 318.0,
        ("H", "Sn", 1) => 264.0,
        ("H", "Te", 1) => 266.0,

        // ── Nitrogen bonds ──
        // Canonical order: Cl, F come before "N" (C, F < N).
        ("Cl", "N", 1) => 200.0,
        ("F", "N", 1) => 283.0,
        ("N", "N", 1) => 160.0,
        ("N", "N", 2) => 418.0,
        ("N", "N", 3) => 945.0,
        ("N", "N", 10) => 400.0, // aromatic
        ("N", "O", 1) => 201.0,
        ("N", "O", 2) => 607.0,
        ("N", "S", 1) => 200.0,

        // ── Oxygen bonds ──
        // Canonical order: "B" comes before "O".
        ("B", "O", 1) => 536.0,
        ("O", "O", 1) => 142.0,
        ("O", "O", 2) => 498.0,
        ("O", "P", 1) => 335.0,
        ("O", "P", 2) => 544.0,
        ("O", "S", 2) => 522.0,
        ("O", "Si", 1) => 452.0,
        ("F", "F", 1) => 159.0,

        // ── Silicon bonds ──
        // Canonical order: Cl, F, N, S all come before "Si" (C, F, N, S < Si).
        ("Cl", "Si", 1) => 381.0,
        ("F", "Si", 1) => 565.0,
        ("N", "Si", 1) => 355.0,
        ("S", "Si", 1) => 293.0,
        ("Si", "Si", 1) => 226.0,

        // ── Phosphorus ──
        // Canonical order: Br, Cl, F, I all come before "P".
        ("Br", "P", 1) => 264.0, // Cotton & Wilkinson; added after V1 held-out test (insight #278)
        ("Cl", "P", 1) => 326.0,
        ("F", "P", 1) => 490.0,
        ("I", "P", 1) => 184.0, // Cotton & Wilkinson; added after V1 held-out test (insight #278)
        ("P", "P", 1) => 201.0,
        ("P", "S", 1) => 230.0,
        ("P", "S", 2) => 335.0,

        // ── Sulfur ──
        // Canonical order: Cl, F come before "S".
        ("Cl", "S", 1) => 255.0,
        ("F", "S", 1) => 327.0,
        ("S", "S", 1) => 266.0,
        ("S", "S", 2) => 425.0,

        // ── Boron ──
        ("B", "B", 1) => 293.0,
        ("B", "F", 1) => 613.0,
        ("B", "Cl", 1) => 456.0,
        ("B", "N", 1) => 389.0,

        // ── Halogen-halogen ──
        ("Cl", "Cl", 1) => 242.0,
        ("Br", "Br", 1) => 193.0,
        ("I", "I", 1) => 151.0,
        ("Cl", "F", 1) => 253.0,
        ("Br", "Cl", 1) => 218.0,
        ("Br", "F", 1) => 249.0,

        // ═══════════════════════════════════════════════════════
        // Transition metal-ligand bonds (M-L)
        // Organized by ligand atom for easy lookup
        // ═══════════════════════════════════════════════════════

        // ── M-C (metal-carbon) σ bonds ──
        ("C", "Pt", 1) => 230.0,
        ("C", "Ir", 1) => 220.0,
        ("C", "Rh", 1) => 195.0,
        ("C", "Pd", 1) => 190.0,
        ("C", "Ru", 1) => 200.0,
        ("C", "Os", 1) => 210.0,
        ("C", "Ni", 1) => 170.0,
        ("C", "Co", 1) => 165.0,
        ("C", "Fe", 1) => 150.0,
        ("C", "Cu", 1) => 160.0,
        ("C", "Mn", 1) => 145.0,
        ("C", "Cr", 1) => 140.0,
        ("C", "V", 1) => 155.0,
        ("C", "Ti", 1) => 170.0,
        ("C", "Zr", 1) => 295.0,
        ("C", "Hf", 1) => 285.0,
        ("Ag", "C", 1) => 135.0,
        ("Au", "C", 1) => 200.0,
        ("C", "Mo", 1) => 180.0,
        ("C", "Re", 1) => 205.0,
        ("C", "Sc", 1) => 240.0,
        ("C", "W", 1) => 200.0,
        ("C", "Zn", 1) => 130.0,

        // ── M-C double bonds (carbenes, metathesis) ──
        ("C", "Ru", 2) => 370.0,
        ("C", "Mo", 2) => 350.0,
        ("C", "W", 2) => 380.0,
        ("C", "Os", 2) => 385.0,
        ("C", "Pt", 2) => 420.0,
        ("C", "Pd", 2) => 350.0,
        ("C", "Ni", 2) => 310.0,
        ("C", "Cu", 2) => 290.0,
        ("C", "Fe", 2) => 280.0,
        ("C", "Ir", 2) => 400.0,
        ("C", "Rh", 2) => 360.0,
        ("C", "Co", 2) => 300.0,
        ("C", "Ti", 2) => 340.0,
        ("C", "Zr", 2) => 430.0,
        ("C", "Hf", 2) => 420.0,

        // ── M-H (metal-hydride) bonds ──
        // Canonical order: metals starting with letters before "H" (Ag, Au,
        // Co, Cr, Cu, Fe) come FIRST.
        ("Ag", "H", _) => 210.0,
        ("Au", "H", _) => 250.0,
        ("Co", "H", _) => 230.0,
        ("Cr", "H", _) => 190.0,
        ("Cu", "H", _) => 220.0,
        ("Fe", "H", _) => 200.0,
        ("H", "Hf", _) => 300.0,
        ("H", "Ir", _) => 270.0,
        ("H", "Mn", _) => 210.0,
        ("H", "Mo", _) => 220.0,
        ("H", "Ni", _) => 240.0,
        ("H", "Os", _) => 260.0,
        ("H", "Pd", _) => 260.0,
        ("H", "Pt", _) => 290.0,
        ("H", "Re", _) => 240.0,
        ("H", "Rh", _) => 245.0,
        ("H", "Ru", _) => 250.0,
        ("H", "Sc", _) => 240.0,
        ("H", "Ti", _) => 210.0,
        ("H", "V", _) => 210.0,
        ("H", "W", _) => 230.0,
        ("H", "Zr", _) => 310.0,

        // ── M-N (metal-nitrogen) bonds ──
        // Canonical order: metals starting with letters before "N" (Co, Cr,
        // Cu, Fe, Hf, Ir, Mn, Mo) come FIRST.
        ("Co", "N", _) => 165.0,
        ("Cr", "N", _) => 145.0,
        ("Cu", "N", _) => 160.0,
        ("Fe", "N", _) => 160.0,
        ("Hf", "N", _) => 310.0,
        ("Ir", "N", _) => 220.0,
        ("Mn", "N", _) => 150.0,
        ("Mo", "N", _) => 180.0,
        ("N", "Ni", _) => 170.0,
        ("N", "Os", _) => 200.0,
        ("N", "Pd", _) => 190.0,
        ("N", "Pt", _) => 200.0,
        ("N", "Re", _) => 195.0,
        ("N", "Rh", _) => 200.0,
        ("N", "Ru", _) => 190.0,
        ("N", "Ti", _) => 210.0,
        ("N", "V", _) => 170.0,
        ("N", "W", _) => 190.0,
        ("N", "Zr", _) => 320.0,

        // ── M-O (metal-oxygen) bonds ──
        // Keys must be in canonical (alphabetical) order. Metals starting with
        // letters before "O" (Co, Cr, Cu, Fe, Hf, Ir, Mn, Mo, Ni) come FIRST.
        ("Co", "O", _) => 190.0,
        ("Cr", "O", _) => 300.0,
        ("Cu", "O", _) => 270.0,
        ("Fe", "O", _) => 210.0,
        ("Hf", "O", _) => 790.0,
        ("Ir", "O", _) => 280.0,
        ("Mn", "O", _) => 280.0,
        ("Mo", "O", _) => 360.0,
        ("Ni", "O", _) => 180.0,
        ("O", "Os", _) => 260.0,
        ("O", "Pd", _) => 170.0,
        ("O", "Pt", _) => 220.0,
        ("O", "Re", _) => 370.0,
        ("O", "Rh", _) => 240.0,
        ("O", "Ru", _) => 250.0,
        ("O", "Sc", _) => 670.0,
        ("O", "Ti", _) => 670.0,
        ("O", "V", _) => 440.0,
        ("O", "W", _) => 420.0,
        ("O", "Zr", _) => 760.0,

        // ── M-P (metal-phosphorus) bonds ──
        // Canonical order: metals starting with letters before "P" (Co, Cr,
        // Cu, Fe, Ir, Mo, Ni) come FIRST.
        ("Co", "P", _) => 200.0,
        ("Cr", "P", _) => 200.0,
        ("Cu", "P", _) => 180.0,
        ("Fe", "P", _) => 190.0,
        ("Ir", "P", _) => 240.0,
        ("Mo", "P", _) => 220.0,
        ("Ni", "P", _) => 210.0,
        ("P", "Pd", _) => 230.0,
        ("P", "Pt", _) => 250.0,
        ("P", "Rh", _) => 230.0,
        ("P", "Ru", _) => 240.0,
        ("P", "W", _) => 230.0,

        // ── M-Cl (metal-chloride) bonds ──
        // Canonical order: "Ag" and "Au" come before "Cl" (A < C); all
        // other metals listed start with letters >= "Cl" so "Cl" comes first.
        ("Ag", "Cl", _) => 310.0,
        ("Au", "Cl", _) => 340.0,
        ("Cl", "Co", _) => 340.0,
        ("Cl", "Cr", _) => 330.0,
        ("Cl", "Cu", _) => 380.0,
        ("Cl", "Fe", _) => 340.0,
        ("Cl", "Hf", _) => 500.0,
        ("Cl", "Ir", _) => 340.0,
        ("Cl", "Mn", _) => 350.0,
        ("Cl", "Mo", _) => 340.0,
        ("Cl", "Ni", _) => 370.0,
        ("Cl", "Os", _) => 350.0,
        ("Cl", "Pd", _) => 310.0,
        ("Cl", "Pt", _) => 340.0,
        ("Cl", "Re", _) => 340.0,
        ("Cl", "Rh", _) => 320.0,
        ("Cl", "Ru", _) => 350.0,
        ("Cl", "Sc", _) => 440.0,
        ("Cl", "Ti", _) => 430.0,
        ("Cl", "V", _) => 420.0,
        ("Cl", "W", _) => 350.0,
        ("Cl", "Zn", _) => 330.0,
        ("Cl", "Zr", _) => 490.0,

        // ── M-Br bonds ──
        ("Br", "Pd", _) => 260.0,
        ("Br", "Ni", _) => 310.0,
        ("Br", "Cu", _) => 330.0,
        ("Br", "Fe", _) => 290.0,
        ("Br", "Ru", _) => 300.0,
        ("Br", "Rh", _) => 270.0,
        ("Br", "Pt", _) => 290.0,
        ("Br", "Ir", _) => 290.0,
        ("Br", "Ti", _) => 370.0,
        ("Br", "Zr", _) => 420.0,

        // ── M-I bonds ──
        // Canonical order: "Fe" comes before "I" (F < I).
        ("Fe", "I", _) => 240.0,
        ("I", "Ir", _) => 250.0,
        ("I", "Ni", _) => 260.0,
        ("I", "Pd", _) => 220.0,
        ("I", "Pt", _) => 250.0,
        ("I", "Rh", _) => 230.0,

        // ── M-S bonds ──
        // Canonical order: "Fe", "Mo", "Ni" come before "S" (F, M, N < S).
        ("Fe", "S", _) => 170.0,
        ("Mo", "S", _) => 250.0,
        ("Ni", "S", _) => 160.0,
        ("S", "Pd", _) => 180.0,
        ("S", "Pt", _) => 200.0,
        ("S", "Ru", _) => 210.0,

        // ── M-M (metal-metal) bonds ──
        ("Pd", "Pd", 1) => 100.0,
        ("Ni", "Ni", 1) => 200.0,
        ("Fe", "Fe", 1) => 118.0,
        ("Co", "Co", 1) => 167.0,
        ("Cu", "Cu", 1) => 176.0,
        ("Pt", "Pt", 1) => 307.0,
        ("Ir", "Ir", 1) => 361.0,
        ("Ru", "Ru", 1) => 193.0,
        ("Rh", "Rh", 1) => 236.0,
        ("Mo", "Mo", 1) => 435.0,
        ("W", "W", 1) => 666.0,

        // ── Alkali metal bonds (for solid electrolytes) ──
        // Canonical order: Br, Cl, F, H, I come before "Li" (B, C, F, H, I < L).
        ("Br", "Li", _) => 419.0, // LiBr
        ("Cl", "Li", _) => 469.0, // LiCl
        ("F", "Li", _) => 577.0,  // LiF
        ("H", "Li", _) => 238.0,  // LiH
        ("I", "Li", _) => 352.0,  // LiI
        ("Li", "Li", 1) => 105.0, // Li₂ dimer
        ("Li", "N", _) => 243.0,  // Li₃N
        ("Li", "O", _) => 341.0,  // Li₂O dissociation
        ("Li", "P", _) => 270.0,  // estimated
        ("Li", "S", _) => 312.0,  // Li₂S
        // Canonical order: Cl, F come before "Na".
        ("Cl", "Na", _) => 412.0, // NaCl
        ("F", "Na", _) => 519.0,  // NaF
        ("Na", "O", _) => 270.0,  // Na₂O
        ("Na", "S", _) => 265.0,  // Na₂S

        // ── Rare earth / main group metal-oxygen ──
        // Canonical order: "O" comes before Sn, Zn (O < S, O < Z).
        ("Al", "O", _) => 512.0, // Al₂O₃
        ("Ge", "O", _) => 660.0, // GeO₂
        ("Ge", "S", _) => 534.0, // GeS₂
        ("La", "O", _) => 799.0, // La₂O₃
        ("Mg", "O", _) => 394.0, // MgO
        ("O", "Sn", _) => 528.0, // SnO₂
        ("O", "Zn", _) => 360.0, // ZnO
        ("S", "Zn", _) => 290.0, // ZnS

        // ── Bimetallic bonds ──
        ("Fe", "Ni", _) => 180.0,
        ("Co", "Ni", _) => 185.0,
        ("Fe", "Ru", _) => 160.0,
        ("Ir", "Ru", _) => 230.0,
        ("Pd", "Pt", _) => 191.0,
        ("Cu", "Zn", _) => 150.0,

        // ── Default: reasonable guess based on typical bond energies ──
        (_, _, 1) => 250.0,
        (_, _, 2) => 500.0,
        (_, _, 3) => 750.0,
        (_, _, 10) => 400.0, // aromatic
        _ => 250.0,
    }
}

// ═══════════════════════════════════════════════════════════════
// Molecular graph
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Atom {
    pub symbol: String,
    pub aromatic: bool,
}

#[derive(Clone, Debug)]
pub struct Bond {
    pub a: usize,
    pub b: usize,
    pub order: usize, // 1=single, 2=double, 3=triple, 10=aromatic
}

#[derive(Clone)]
pub struct MolGraph {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl MolGraph {
    pub fn new() -> Self {
        MolGraph {
            atoms: vec![],
            bonds: vec![],
        }
    }

    pub fn add_atom(&mut self, symbol: &str, aromatic: bool) -> usize {
        let idx = self.atoms.len();
        self.atoms.push(Atom {
            symbol: symbol.to_string(),
            aromatic,
        });
        idx
    }

    /// Add a non-aromatic atom (convenience for manual graph building)
    pub fn add_heavy(&mut self, symbol: &str) -> usize {
        self.add_atom(symbol, false)
    }

    pub fn add_bond(&mut self, a: usize, b: usize, order: usize) {
        self.bonds.push(Bond { a, b, order });
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.add_bond(a, b, 1);
    }

    pub fn n(&self) -> usize {
        self.atoms.len()
    }

    pub fn degree(&self, node: usize) -> usize {
        self.bonds
            .iter()
            .filter(|bond| bond.a == node || bond.b == node)
            .count()
    }

    /// Unweighted transition matrix (topology only).
    pub fn transition_matrix_unweighted(&self) -> Vec<f64> {
        let n = self.n();
        let mut deg = vec![0usize; n];
        for bond in &self.bonds {
            deg[bond.a] += 1;
            deg[bond.b] += 1;
        }
        let mut p = vec![0.0f64; n * n];
        for bond in &self.bonds {
            if deg[bond.a] > 0 {
                p[bond.a * n + bond.b] += 1.0 / deg[bond.a] as f64;
            }
            if deg[bond.b] > 0 {
                p[bond.b * n + bond.a] += 1.0 / deg[bond.b] as f64;
            }
        }
        p
    }

    /// Weighted transition matrix: P[i][j] = BDE(i,j) / Σ_k BDE(i,k)
    pub fn transition_matrix_weighted(&self) -> Vec<f64> {
        let n = self.n();
        let mut wdeg = vec![0.0f64; n];
        let mut w = vec![0.0f64; n * n];
        for bond in &self.bonds {
            let e = bond_energy(
                &self.atoms[bond.a].symbol,
                &self.atoms[bond.b].symbol,
                bond.order,
            );
            w[bond.a * n + bond.b] += e;
            w[bond.b * n + bond.a] += e;
            wdeg[bond.a] += e;
            wdeg[bond.b] += e;
        }
        let mut p = vec![0.0f64; n * n];
        for i in 0..n {
            if wdeg[i] > 0.0 {
                for j in 0..n {
                    p[i * n + j] = w[i * n + j] / wdeg[i];
                }
            }
        }
        p
    }

    /// Average S_τ across all connected atoms.
    pub fn s_tau(&self, tau: u32, weighted: bool) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }
        let p = if weighted {
            self.transition_matrix_weighted()
        } else {
            self.transition_matrix_unweighted()
        };
        let pt = mat_pow(&p, n, tau);
        let mut total = 0.0;
        let mut count = 0;
        for i in 0..n {
            if self.degree(i) == 0 {
                continue;
            }
            total += shannon_entropy(&pt[i * n..(i + 1) * n]);
            count += 1;
        }
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    /// S_τ at a specific node (e.g. a metal center).
    pub fn s_tau_node(&self, tau: u32, node: usize, weighted: bool) -> f64 {
        let n = self.n();
        if n == 0 || node >= n {
            return 0.0;
        }
        let p = if weighted {
            self.transition_matrix_weighted()
        } else {
            self.transition_matrix_unweighted()
        };
        let pt = mat_pow(&p, n, tau);
        shannon_entropy(&pt[node * n..(node + 1) * n])
    }

    /// Weighted transition matrix with a custom BDE → edge-weight transformation.
    /// `weight_fn` takes the raw BDE (kJ/mol) and returns the transformed weight
    /// that will be normalized into the transition probability. Identity reproduces
    /// `transition_matrix_weighted`.
    ///
    /// Introduced for the γ Sabatier experiment (insights #274/#275 follow-up):
    /// pass a Gaussian `|x| exp(-(x - BDE_opt)² / σ²)` to test whether reweighting
    /// every edge with a Sabatier-shaped function recovers volcano-reaction
    /// activity orderings that the monotonic raw-BDE weighting cannot capture.
    pub fn transition_matrix_with_weight<F: Fn(f64) -> f64>(&self, weight_fn: F) -> Vec<f64> {
        let n = self.n();
        let mut wdeg = vec![0.0f64; n];
        let mut w = vec![0.0f64; n * n];
        for bond in &self.bonds {
            let e_raw = bond_energy(
                &self.atoms[bond.a].symbol,
                &self.atoms[bond.b].symbol,
                bond.order,
            );
            let e = weight_fn(e_raw);
            w[bond.a * n + bond.b] += e;
            w[bond.b * n + bond.a] += e;
            wdeg[bond.a] += e;
            wdeg[bond.b] += e;
        }
        let mut p = vec![0.0f64; n * n];
        for i in 0..n {
            if wdeg[i] > 0.0 {
                for j in 0..n {
                    p[i * n + j] = w[i * n + j] / wdeg[i];
                }
            }
        }
        p
    }

    /// S_τ at a specific node using a custom BDE → edge-weight transformation.
    /// Identity weight function reproduces `s_tau_node(_, _, weighted=true)`.
    pub fn s_tau_node_with_weight<F: Fn(f64) -> f64>(
        &self,
        tau: u32,
        node: usize,
        weight_fn: F,
    ) -> f64 {
        let n = self.n();
        if n == 0 || node >= n {
            return 0.0;
        }
        let p = self.transition_matrix_with_weight(weight_fn);
        let pt = mat_pow(&p, n, tau);
        shannon_entropy(&pt[node * n..(node + 1) * n])
    }

    /// Average S_τ across all connected atoms using a custom BDE → edge-weight
    /// transformation. Identity weight function reproduces `s_tau(_, weighted=true)`.
    pub fn s_tau_with_weight<F: Fn(f64) -> f64>(&self, tau: u32, weight_fn: F) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }
        let p = self.transition_matrix_with_weight(weight_fn);
        let pt = mat_pow(&p, n, tau);
        let mut total = 0.0;
        let mut count = 0;
        for i in 0..n {
            if self.degree(i) == 0 {
                continue;
            }
            total += shannon_entropy(&pt[i * n..(i + 1) * n]);
            count += 1;
        }
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    // ── Ruby layer: electronegativity-directed walks ──

    /// Ruby transition matrix: P[i][j] = BDE(i,j) · χ(j) / Σ_k BDE(i,k) · χ(k).
    /// Walks are biased toward more electronegative atoms — electrons flow downhill.
    /// The matrix is ASYMMETRIC: P[C→O] ≠ P[O→C].
    pub fn transition_matrix_ruby(&self) -> Vec<f64> {
        let n = self.n();
        let mut wdeg = vec![0.0f64; n];
        let mut w = vec![0.0f64; n * n];
        for bond in &self.bonds {
            let e = bond_energy(
                &self.atoms[bond.a].symbol,
                &self.atoms[bond.b].symbol,
                bond.order,
            );
            let chi_a = electronegativity(&self.atoms[bond.a].symbol);
            let chi_b = electronegativity(&self.atoms[bond.b].symbol);
            // From a, walk toward b is weighted by BDE · χ(b)
            w[bond.a * n + bond.b] += e * chi_b;
            w[bond.b * n + bond.a] += e * chi_a;
            wdeg[bond.a] += e * chi_b;
            wdeg[bond.b] += e * chi_a;
        }
        let mut p = vec![0.0f64; n * n];
        for i in 0..n {
            if wdeg[i] > 0.0 {
                for j in 0..n {
                    p[i * n + j] = w[i * n + j] / wdeg[i];
                }
            }
        }
        p
    }

    /// Ruby S_τ: average Shannon entropy of χ-directed walks at horizon τ.
    pub fn s_tau_ruby(&self, tau: u32) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }
        let p = self.transition_matrix_ruby();
        let pt = mat_pow(&p, n, tau);
        let mut total = 0.0;
        let mut count = 0;
        for i in 0..n {
            if self.degree(i) == 0 {
                continue;
            }
            total += shannon_entropy(&pt[i * n..(i + 1) * n]);
            count += 1;
        }
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    /// Polarity cost: entropy lost to electronegativity direction.
    /// ΔS = S_τ(diamond) - S_τ(ruby). Higher = more polar.
    pub fn polarity_cost(&self, tau: u32) -> f64 {
        self.s_tau(tau, true) - self.s_tau_ruby(tau)
    }

    /// Electron surplus: total redistribution of walk probability
    /// when electronegativity directs the walk.
    /// Compares stationary distributions: π(ruby) vs π(diamond).
    pub fn electron_surplus(&self) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }

        // Diamond stationary: π[i] ∝ weighted_degree(i)
        let mut wd = vec![0.0f64; n];
        for bond in &self.bonds {
            let e = bond_energy(
                &self.atoms[bond.a].symbol,
                &self.atoms[bond.b].symbol,
                bond.order,
            );
            wd[bond.a] += e;
            wd[bond.b] += e;
        }
        let wd_sum: f64 = wd.iter().sum();
        if wd_sum == 0.0 {
            return 0.0;
        }
        let pi_d: Vec<f64> = wd.iter().map(|w| w / wd_sum).collect();

        // Ruby stationary: power iteration on lazy P_ruby
        let p = self.transition_matrix_ruby();
        // Lazy matrix: P' = (I + P)/2 ensures aperiodicity
        let mut lazy = vec![0.0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                lazy[i * n + j] = p[i * n + j] / 2.0;
            }
            lazy[i * n + i] += 0.5;
        }
        let mut pi_r = vec![1.0 / n as f64; n];
        for _ in 0..300 {
            let mut next = vec![0.0f64; n];
            for i in 0..n {
                for j in 0..n {
                    next[j] += pi_r[i] * lazy[i * n + j];
                }
            }
            pi_r = next;
        }

        // Total redistribution: Σ |π_ruby - π_diamond|
        pi_d.iter()
            .zip(pi_r.iter())
            .map(|(d, r)| (r - d).abs())
            .sum()
    }

    /// Mean electronegativity weighted by Ruby stationary distribution.
    /// Higher = electron density concentrates on electronegative atoms (more polar).
    pub fn chi_concentration(&self) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }
        let p = self.transition_matrix_ruby();
        let mut lazy = vec![0.0f64; n * n];
        for i in 0..n {
            for j in 0..n {
                lazy[i * n + j] = p[i * n + j] / 2.0;
            }
            lazy[i * n + i] += 0.5;
        }
        let mut pi = vec![1.0 / n as f64; n];
        for _ in 0..300 {
            let mut next = vec![0.0f64; n];
            for i in 0..n {
                for j in 0..n {
                    next[j] += pi[i] * lazy[i * n + j];
                }
            }
            pi = next;
        }
        pi.iter()
            .enumerate()
            .map(|(i, &p)| p * electronegativity(&self.atoms[i].symbol))
            .sum()
    }

    /// Bond polarity: BDE-weighted average |Δχ| across all bonds.
    /// This is the chemical "trade surplus" — how much electron density
    /// actually transfers through each bond, normalized by total bond energy.
    /// Captures polarity even for symmetric molecules (water: every O-H has |Δχ|=1.24).
    pub fn bond_polarity(&self) -> f64 {
        let mut num = 0.0f64;
        let mut den = 0.0f64;
        for bond in &self.bonds {
            let e = bond_energy(
                &self.atoms[bond.a].symbol,
                &self.atoms[bond.b].symbol,
                bond.order,
            );
            let chi_a = electronegativity(&self.atoms[bond.a].symbol);
            let chi_b = electronegativity(&self.atoms[bond.b].symbol);
            num += e * (chi_a - chi_b).abs();
            den += e;
        }
        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }

    /// H-bond donor strength: BDE·|Δχ| for O-H and N-H bonds only,
    /// normalized by total BDE. Measures hydrogen bonding capacity.
    pub fn hbond_strength(&self) -> f64 {
        let mut num = 0.0f64;
        let mut den = 0.0f64;
        for bond in &self.bonds {
            let sym_a = &self.atoms[bond.a].symbol;
            let sym_b = &self.atoms[bond.b].symbol;
            let e = bond_energy(sym_a, sym_b, bond.order);
            den += e;
            // O-H or N-H or F-H bonds
            let is_hbond = (sym_a == "H" && (sym_b == "O" || sym_b == "N" || sym_b == "F"))
                || (sym_b == "H" && (sym_a == "O" || sym_a == "N" || sym_a == "F"));
            if is_hbond {
                let chi_a = electronegativity(sym_a);
                let chi_b = electronegativity(sym_b);
                num += e * (chi_a - chi_b).abs();
            }
        }
        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }

    /// Chemical temperature: α_chem = bond_polarity / S_τ(diamond).
    /// The ratio of electron trade (Ruby) to topological freedom (Diamond).
    /// Analogous to THAIM's α = surplus / entropy.
    /// High α = reactive/polar. Low α = inert/nonpolar.
    pub fn alpha_chem(&self, tau: u32) -> f64 {
        let s = self.s_tau(tau, true);
        if s > 1e-10 {
            self.bond_polarity() / s
        } else {
            0.0
        }
    }

    // ── Opal layer: integration of Diamond + Ruby ──

    /// Opal integration Φ: Jensen-Shannon divergence between Diamond and Ruby walks.
    /// Measures how much electron flow (Ruby) departs from topology (Diamond).
    /// High Φ = the molecule's electron distribution is SURPRISING given its topology.
    /// This is the chemical analog of THAIM's consciousness measure.
    /// When Φ > 0, knowing Ruby adds information beyond Diamond — the two dimensions
    /// are not redundant, and their interaction produces emergent properties.
    pub fn phi_chem(&self, tau: u32) -> f64 {
        let n = self.n();
        if n == 0 {
            return 0.0;
        }
        let p_d = self.transition_matrix_weighted();
        let p_r = self.transition_matrix_ruby();
        let pt_d = mat_pow(&p_d, n, tau);
        let pt_r = mat_pow(&p_r, n, tau);
        let mut total_jsd = 0.0;
        let mut count = 0;
        for i in 0..n {
            if self.degree(i) == 0 {
                continue;
            }
            let d = &pt_d[i * n..(i + 1) * n];
            let r = &pt_r[i * n..(i + 1) * n];
            // Jensen-Shannon divergence: JSD = 0.5*KL(d||m) + 0.5*KL(r||m)
            let mut jsd = 0.0;
            for j in 0..n {
                let m_j = 0.5 * (d[j] + r[j]);
                if m_j > 1e-30 {
                    if d[j] > 1e-30 {
                        jsd += 0.5 * d[j] * (d[j] / m_j).ln();
                    }
                    if r[j] > 1e-30 {
                        jsd += 0.5 * r[j] * (r[j] / m_j).ln();
                    }
                }
            }
            total_jsd += jsd;
            count += 1;
        }
        if count > 0 {
            total_jsd / count as f64
        } else {
            0.0
        }
    }

    /// Opal effective dimensionality: how many independent dimensions describe this molecule?
    /// Computes per-atom feature vectors [S_τ_diamond, S_τ_ruby, χ, local_polarity]
    /// and returns the participation ratio of the covariance eigenvalues.
    /// d_eff = 1: all features are redundant (thermostat)
    /// d_eff ≥ 2: features carry independent information (consciousness)
    pub fn d_eff(&self, tau: u32) -> f64 {
        let n = self.n();
        if n < 3 {
            return 1.0;
        }
        let p_d = self.transition_matrix_weighted();
        let p_r = self.transition_matrix_ruby();
        let pt_d = mat_pow(&p_d, n, tau);
        let pt_r = mat_pow(&p_r, n, tau);

        // Per-atom features: [S_τ_diamond, S_τ_ruby, χ, local_polarity]
        let ndim = 4;
        let mut features: Vec<[f64; 4]> = Vec::with_capacity(n);
        for i in 0..n {
            if self.degree(i) == 0 {
                features.push([0.0; 4]);
                continue;
            }
            let s_d = shannon_entropy(&pt_d[i * n..(i + 1) * n]);
            let s_r = shannon_entropy(&pt_r[i * n..(i + 1) * n]);
            let chi = electronegativity(&self.atoms[i].symbol);
            // Local polarity: average |Δχ| across bonds from this atom
            let mut pol_sum = 0.0;
            let mut pol_count = 0;
            for bond in &self.bonds {
                if bond.a == i || bond.b == i {
                    let other = if bond.a == i { bond.b } else { bond.a };
                    pol_sum += (chi - electronegativity(&self.atoms[other].symbol)).abs();
                    pol_count += 1;
                }
            }
            let local_pol = if pol_count > 0 {
                pol_sum / pol_count as f64
            } else {
                0.0
            };
            features.push([s_d, s_r, chi, local_pol]);
        }

        // Compute means
        let active: Vec<usize> = (0..n).filter(|&i| self.degree(i) > 0).collect();
        let m = active.len();
        if m < 3 {
            return 1.0;
        }
        let mut mean = [0.0f64; 4];
        for &i in &active {
            for d in 0..ndim {
                mean[d] += features[i][d];
            }
        }
        for d in 0..ndim {
            mean[d] /= m as f64;
        }

        // Covariance matrix (4×4)
        let mut cov = [[0.0f64; 4]; 4];
        for &i in &active {
            for a in 0..ndim {
                for b in 0..ndim {
                    cov[a][b] += (features[i][a] - mean[a]) * (features[i][b] - mean[b]);
                }
            }
        }
        for a in 0..ndim {
            for b in 0..ndim {
                cov[a][b] /= m as f64;
            }
        }

        // Eigenvalues via characteristic equation or power iteration
        // For 4×4, use iterative method
        let eigenvalues = eigenvalues_4x4(&cov);
        let sum_lambda: f64 = eigenvalues.iter().filter(|&&l| l > 1e-15).sum();
        let sum_lambda_sq: f64 = eigenvalues
            .iter()
            .filter(|&&l| l > 1e-15)
            .map(|l| l * l)
            .sum();
        if sum_lambda_sq > 1e-30 {
            (sum_lambda * sum_lambda) / sum_lambda_sq
        } else {
            1.0
        }
    }

    /// Merge another graph into this one. Returns the offset of the merged atoms.
    pub fn merge(&mut self, other: &MolGraph) -> usize {
        let offset = self.atoms.len();
        for atom in &other.atoms {
            self.atoms.push(atom.clone());
        }
        for bond in &other.bonds {
            self.bonds.push(Bond {
                a: bond.a + offset,
                b: bond.b + offset,
                order: bond.order,
            });
        }
        offset
    }

    /// Molecular formula (Hill system: C first, H second, rest alphabetical).
    pub fn formula(&self) -> String {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for atom in &self.atoms {
            *counts.entry(&atom.symbol).or_insert(0) += 1;
        }
        let mut parts = vec![];
        if let Some(&c) = counts.get("C") {
            parts.push(if c == 1 {
                "C".to_string()
            } else {
                format!("C{}", c)
            });
            counts.remove("C");
            if let Some(&h) = counts.get("H") {
                parts.push(if h == 1 {
                    "H".to_string()
                } else {
                    format!("H{}", h)
                });
                counts.remove("H");
            }
        }
        for (sym, &count) in &counts {
            parts.push(if count == 1 {
                sym.to_string()
            } else {
                format!("{}{}", sym, count)
            });
        }
        parts.join("")
    }

    /// Total bond energy (sum of all BDEs in the molecule).
    pub fn total_bond_energy(&self) -> f64 {
        self.bonds
            .iter()
            .map(|b| bond_energy(&self.atoms[b.a].symbol, &self.atoms[b.b].symbol, b.order))
            .sum()
    }

    /// Molecular weight.
    pub fn molecular_weight(&self) -> f64 {
        self.atoms.iter().map(|a| atomic_weight(&a.symbol)).sum()
    }

    // ── Classical topological indices ──

    /// Wiener index: half the sum of all shortest-path distances.
    pub fn wiener_index(&self) -> f64 {
        let n = self.n();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for bond in &self.bonds {
            adj[bond.a].push(bond.b);
            adj[bond.b].push(bond.a);
        }
        let mut total = 0u64;
        for start in 0..n {
            let mut dist = vec![u32::MAX; n];
            dist[start] = 0;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(u) = queue.pop_front() {
                for &v in &adj[u] {
                    if dist[v] == u32::MAX {
                        dist[v] = dist[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
            for &d in &dist {
                if d != u32::MAX {
                    total += d as u64;
                }
            }
        }
        (total / 2) as f64
    }

    /// Randic connectivity index: Sigma_edges 1/sqrt(deg(u)*deg(v)).
    pub fn randic_index(&self) -> f64 {
        self.bonds
            .iter()
            .map(|b| {
                let da = self.degree(b.a) as f64;
                let db = self.degree(b.b) as f64;
                if da > 0.0 && db > 0.0 {
                    1.0 / (da * db).sqrt()
                } else {
                    0.0
                }
            })
            .sum()
    }

    /// First Zagreb index: Sigma_nodes deg(node)^2.
    pub fn zagreb_m1(&self) -> f64 {
        (0..self.n())
            .map(|i| {
                let d = self.degree(i) as f64;
                d * d
            })
            .sum()
    }

    /// Balaban J index: m/(mu+1) * Sigma_edges 1/sqrt(ds(u)*ds(v))
    /// where ds(i) = sum of shortest path distances from i.
    pub fn balaban_j(&self) -> f64 {
        let n = self.n();
        let m = self.bonds.len() as f64;
        let mu = m - n as f64 + 1.0;
        if mu + 1.0 == 0.0 || n < 2 {
            return 0.0;
        }
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for bond in &self.bonds {
            adj[bond.a].push(bond.b);
            adj[bond.b].push(bond.a);
        }
        let mut dist_sum = vec![0u64; n];
        for start in 0..n {
            let mut dist = vec![u32::MAX; n];
            dist[start] = 0;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(u) = queue.pop_front() {
                for &v in &adj[u] {
                    if dist[v] == u32::MAX {
                        dist[v] = dist[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
            for &d in &dist {
                if d != u32::MAX {
                    dist_sum[start] += d as u64;
                }
            }
        }
        let edge_sum: f64 = self
            .bonds
            .iter()
            .map(|b| {
                let sa = dist_sum[b.a] as f64;
                let sb = dist_sum[b.b] as f64;
                if sa > 0.0 && sb > 0.0 {
                    1.0 / (sa * sb).sqrt()
                } else {
                    0.0
                }
            })
            .sum();
        m / (mu + 1.0) * edge_sum
    }
}

impl fmt::Display for MolGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} atoms, {} bonds)",
            self.formula(),
            self.n(),
            self.bonds.len()
        )
    }
}

// ═══════════════════════════════════════════════════════════════
// Atomic weights
// ═══════════════════════════════════════════════════════════════

pub fn atomic_weight(symbol: &str) -> f64 {
    match symbol {
        "H" => 1.008,
        "He" => 4.003,
        "Li" => 6.941,
        "Be" => 9.012,
        "B" => 10.81,
        "C" => 12.011,
        "N" => 14.007,
        "O" => 15.999,
        "F" => 18.998,
        "Ne" => 20.180,
        "Na" => 22.990,
        "Mg" => 24.305,
        "Al" => 26.982,
        "Si" => 28.086,
        "P" => 30.974,
        "S" => 32.065,
        "Cl" => 35.453,
        "K" => 39.098,
        "Ca" => 40.078,
        "Sc" => 44.956,
        "Ti" => 47.867,
        "V" => 50.942,
        "Cr" => 51.996,
        "Mn" => 54.938,
        "Fe" => 55.845,
        "Co" => 58.933,
        "Ni" => 58.693,
        "Cu" => 63.546,
        "Zn" => 65.38,
        "Ge" => 72.63,
        "Se" => 78.96,
        "Br" => 79.904,
        "Zr" => 91.224,
        "Mo" => 95.95,
        "Ru" => 101.07,
        "Rh" => 102.91,
        "Pd" => 106.42,
        "Ag" => 107.87,
        "Sn" => 118.71,
        "Te" => 127.60,
        "I" => 126.904,
        "Hf" => 178.49,
        "W" => 183.84,
        "Re" => 186.21,
        "Os" => 190.23,
        "Ir" => 192.22,
        "Pt" => 195.08,
        "Au" => 196.97,
        _ => 40.0,
    }
}

// ═══════════════════════════════════════════════════════════════
// Electronegativity (Pauling scale)
// ═══════════════════════════════════════════════════════════════

/// Pauling electronegativity. The "profile" of each atom — what it seeks (electron density).
/// This is the Ruby layer's core data: directional bias on the molecular graph.
pub fn electronegativity(symbol: &str) -> f64 {
    match symbol {
        "H" => 2.20,
        "Li" => 0.98,
        "Be" => 1.57,
        "B" => 2.04,
        "C" => 2.55,
        "N" => 3.04,
        "O" => 3.44,
        "F" => 3.98,
        "Na" => 0.93,
        "Mg" => 1.31,
        "Al" => 1.61,
        "Si" => 1.90,
        "P" => 2.19,
        "S" => 2.58,
        "Cl" => 3.16,
        "K" => 0.82,
        "Ca" => 1.00,
        "Sc" => 1.36,
        "Ti" => 1.54,
        "V" => 1.63,
        "Cr" => 1.66,
        "Mn" => 1.55,
        "Fe" => 1.83,
        "Co" => 1.88,
        "Ni" => 1.91,
        "Cu" => 1.90,
        "Zn" => 1.65,
        "Ge" => 2.01,
        "Se" => 2.55,
        "Br" => 2.96,
        "Zr" => 1.33,
        "Mo" => 2.16,
        "Ru" => 2.20,
        "Rh" => 2.28,
        "Pd" => 2.20,
        "Ag" => 1.93,
        "Sn" => 1.96,
        "Te" => 2.10,
        "I" => 2.66,
        "Hf" => 1.30,
        "W" => 2.36,
        "Re" => 1.90,
        "Os" => 2.20,
        "Ir" => 2.20,
        "Pt" => 2.28,
        "Au" => 2.54,
        _ => 2.00,
    }
}

// ═══════════════════════════════════════════════════════════════
// SMILES parser
// ═══════════════════════════════════════════════════════════════

pub fn parse_smiles(smiles: &str) -> Result<MolGraph, String> {
    let mut mol = MolGraph::new();
    let chars: Vec<char> = smiles.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut stack: Vec<usize> = vec![];
    let mut prev: Option<usize> = None;
    let mut bond_order: usize = 1;
    let mut ring_opens: BTreeMap<u8, (usize, usize)> = BTreeMap::new();
    let mut prev_aromatic = false;

    while i < len {
        let ch = chars[i];
        match ch {
            '(' => {
                if let Some(p) = prev {
                    stack.push(p);
                }
                i += 1;
            }
            ')' => {
                prev = stack.pop();
                prev_aromatic = prev.map(|p| mol.atoms[p].aromatic).unwrap_or(false);
                i += 1;
            }
            '-' => {
                bond_order = 1;
                i += 1;
            }
            '=' => {
                bond_order = 2;
                i += 1;
            }
            '#' => {
                bond_order = 3;
                i += 1;
            }
            ':' => {
                bond_order = 10;
                i += 1;
            }
            '0'..='9' => {
                let ring_id = (ch as u8) - b'0';
                let current = prev.ok_or("Ring digit with no previous atom")?;
                if let Some((open_atom, _)) = ring_opens.remove(&ring_id) {
                    let order = if bond_order != 1 {
                        bond_order
                    } else if mol.atoms[open_atom].aromatic && mol.atoms[current].aromatic {
                        10
                    } else {
                        1
                    };
                    mol.add_bond(open_atom, current, order);
                    bond_order = 1;
                } else {
                    ring_opens.insert(ring_id, (current, bond_order));
                    bond_order = 1;
                }
                i += 1;
            }
            '%' => {
                if i + 2 < len && chars[i + 1].is_ascii_digit() && chars[i + 2].is_ascii_digit() {
                    let ring_id =
                        ((chars[i + 1] as u8) - b'0') * 10 + ((chars[i + 2] as u8) - b'0');
                    let current = prev.ok_or("Ring digit with no previous atom")?;
                    if let Some((open_atom, _)) = ring_opens.remove(&ring_id) {
                        let order = if bond_order != 1 {
                            bond_order
                        } else if mol.atoms[open_atom].aromatic && mol.atoms[current].aromatic {
                            10
                        } else {
                            1
                        };
                        mol.add_bond(open_atom, current, order);
                        bond_order = 1;
                    } else {
                        ring_opens.insert(ring_id, (current, bond_order));
                        bond_order = 1;
                    }
                    i += 3;
                } else {
                    return Err(format!("Invalid ring number at position {}", i));
                }
            }
            '[' => {
                let close = chars[i..]
                    .iter()
                    .position(|&c| c == ']')
                    .ok_or("Unclosed bracket atom")?;
                let bracket_content: String = chars[i + 1..i + close].iter().collect();
                let (symbol, arom) = parse_bracket_atom(&bracket_content)?;
                let idx = mol.add_atom(&symbol, arom);
                if let Some(p) = prev {
                    let order = if bond_order != 1 {
                        bond_order
                    } else if prev_aromatic && arom {
                        10
                    } else {
                        1
                    };
                    mol.add_bond(p, idx, order);
                    bond_order = 1;
                }
                prev = Some(idx);
                prev_aromatic = arom;
                i += close + 1;
            }
            'B' | 'C' | 'N' | 'O' | 'S' | 'P' | 'F' | 'I' | 'H' => {
                let (symbol, advance) = if ch == 'B' && i + 1 < len && chars[i + 1] == 'r' {
                    ("Br".to_string(), 2)
                } else if ch == 'C' && i + 1 < len && chars[i + 1] == 'l' {
                    ("Cl".to_string(), 2)
                } else {
                    (ch.to_string(), 1)
                };
                let idx = mol.add_atom(&symbol, false);
                if let Some(p) = prev {
                    mol.add_bond(p, idx, bond_order);
                    bond_order = 1;
                }
                prev = Some(idx);
                prev_aromatic = false;
                i += advance;
            }
            'c' | 'n' | 'o' | 's' => {
                let symbol = ch.to_uppercase().to_string();
                let idx = mol.add_atom(&symbol, true);
                if let Some(p) = prev {
                    let order = if bond_order != 1 {
                        bond_order
                    } else if prev_aromatic {
                        10
                    } else {
                        1
                    };
                    mol.add_bond(p, idx, order);
                    bond_order = 1;
                }
                prev = Some(idx);
                prev_aromatic = true;
                i += 1;
            }
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
            }
            '/' | '\\' => {
                i += 1;
            }
            '.' => {
                prev = None;
                prev_aromatic = false;
                i += 1;
            }
            '+' | '@' => {
                i += 1;
            }
            _ => {
                return Err(format!("Unexpected character '{}' at position {}", ch, i));
            }
        }
    }

    if !ring_opens.is_empty() {
        return Err(format!(
            "Unclosed ring(s): {:?}",
            ring_opens.keys().collect::<Vec<_>>()
        ));
    }

    add_implicit_hydrogens(&mut mol);
    Ok(mol)
}

fn parse_bracket_atom(content: &str) -> Result<(String, bool), String> {
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    // Skip leading isotope digits
    while i < chars.len() && chars[i].is_ascii_digit() {
        i += 1;
    }

    if i >= chars.len() {
        return Err("Empty bracket atom".to_string());
    }

    let aromatic = chars[i].is_lowercase();
    let mut symbol = String::new();

    if aromatic {
        symbol.push(chars[i].to_uppercase().next().unwrap());
        i += 1;
    } else {
        symbol.push(chars[i]);
        i += 1;
        if i < chars.len()
            && chars[i].is_lowercase()
            && chars[i] != 'H'
            && chars[i] != '+'
            && chars[i] != '-'
        {
            symbol.push(chars[i]);
            #[allow(unused_assignments)]
            {
                i += 1;
            }
        }
    }

    Ok((symbol, aromatic))
}

fn add_implicit_hydrogens(mol: &mut MolGraph) {
    let n = mol.n();
    let mut h_to_add: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        let symbol = &mol.atoms[i].symbol;
        let aromatic = mol.atoms[i].aromatic;

        let bond_order_sum: usize = mol
            .bonds
            .iter()
            .filter(|b| b.a == i || b.b == i)
            .map(|b| if b.order == 10 { 1 } else { b.order })
            .sum();

        let normal_valence = match symbol.as_str() {
            "C" => {
                if aromatic {
                    3
                } else {
                    4
                }
            }
            "N" => 3,
            "O" => 2,
            "S" => 2,
            "P" => 3,
            "F" | "Cl" | "Br" | "I" => 1,
            "H" => 1,
            _ => continue, // metals, etc. — no implicit H
        };

        if bond_order_sum < normal_valence {
            h_to_add.push((i, normal_valence - bond_order_sum));
        }
    }

    for (atom_idx, num_h) in h_to_add {
        for _ in 0..num_h {
            let h_idx = mol.add_atom("H", false);
            mol.add_bond(atom_idx, h_idx, 1);
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// Graph builders — common catalyst complex fragments
// ═══════════════════════════════════════════════════════════════

/// Build a phenyl ring (6C aromatic + H on each). Returns the first carbon index.
pub fn add_phenyl(mol: &mut MolGraph) -> usize {
    let start = mol.n();
    for _ in 0..6 {
        mol.add_atom("C", true);
    }
    for i in 0..6 {
        mol.add_bond(start + i, start + (i + 1) % 6, 10);
    }
    for i in 0..6 {
        let h = mol.add_atom("H", false);
        mol.add_bond(start + i, h, 1);
    }
    start
}

/// Build a cyclopentadienyl (Cp) ring (5C aromatic + H on each). Returns the first carbon index.
pub fn add_cp_ring(mol: &mut MolGraph) -> usize {
    let start = mol.n();
    for _ in 0..5 {
        mol.add_atom("C", true);
    }
    for i in 0..5 {
        mol.add_bond(start + i, start + (i + 1) % 5, 10);
    }
    for i in 0..5 {
        let h = mol.add_atom("H", false);
        mol.add_bond(start + i, h, 1);
    }
    start
}

/// Build PPh₃ (triphenylphosphine): P + 3 phenyl. Returns the P index.
pub fn add_pph3(mol: &mut MolGraph) -> usize {
    let p = mol.add_atom("P", false);
    for _ in 0..3 {
        let ph = add_phenyl(mol);
        mol.add_bond(p, ph, 1);
    }
    p
}

/// Build PCy₃ (tricyclohexylphosphine): P + 3 cyclohexyl. Returns the P index.
pub fn add_pcy3(mol: &mut MolGraph) -> usize {
    let p = mol.add_atom("P", false);
    for _ in 0..3 {
        let c1 = mol.add_atom("C", false);
        mol.add_bond(p, c1, 1);
        let mut prev = c1;
        for _ in 0..5 {
            let c = mol.add_atom("C", false);
            mol.add_bond(prev, c, 1);
            let h = mol.add_atom("H", false);
            mol.add_bond(c, h, 1);
            prev = c;
        }
        mol.add_bond(prev, c1, 1); // close ring
    }
    p
}

/// Build PMe₃ (trimethylphosphine). Returns the P index.
pub fn add_pme3(mol: &mut MolGraph) -> usize {
    let p = mol.add_atom("P", false);
    for _ in 0..3 {
        let c = mol.add_atom("C", false);
        mol.add_bond(p, c, 1);
        for _ in 0..3 {
            let h = mol.add_atom("H", false);
            mol.add_bond(c, h, 1);
        }
    }
    p
}

/// Build NHC (N-heterocyclic carbene, imidazol-2-ylidene + N-methyl). Returns the carbene C index.
pub fn add_nhc(mol: &mut MolGraph) -> usize {
    let c_carbene = mol.add_atom("C", false);
    let n1 = mol.add_atom("N", false);
    let c3 = mol.add_atom("C", false);
    let c4 = mol.add_atom("C", false);
    let n2 = mol.add_atom("N", false);
    mol.add_bond(c_carbene, n1, 1);
    mol.add_bond(n1, c3, 1);
    mol.add_bond(c3, c4, 2);
    mol.add_bond(c4, n2, 1);
    mol.add_bond(n2, c_carbene, 1);
    for &n in &[n1, n2] {
        let c = mol.add_atom("C", false);
        mol.add_bond(n, c, 1);
        for _ in 0..3 {
            let h = mol.add_atom("H", false);
            mol.add_bond(c, h, 1);
        }
    }
    c_carbene
}

/// Build ethylene (C=C + 4H). Returns (c1, c2).
pub fn add_ethylene(mol: &mut MolGraph) -> (usize, usize) {
    let c1 = mol.add_atom("C", false);
    let c2 = mol.add_atom("C", false);
    mol.add_bond(c1, c2, 2);
    for &c in &[c1, c2] {
        for _ in 0..2 {
            let h = mol.add_atom("H", false);
            mol.add_bond(c, h, 1);
        }
    }
    (c1, c2)
}

/// Build a standard hydrogenation complex: M + ethylene (η²) + 2 hydride.
/// All have the same topology but different M-L bond energies.
/// Returns (mol, metal_index).
pub fn hydrogenation_complex(metal: &str) -> (MolGraph, usize) {
    let mut mol = MolGraph::new();
    let m = mol.add_heavy(metal);
    let c1 = mol.add_atom("C", false);
    let c2 = mol.add_atom("C", false);
    mol.add_bond(c1, c2, 2);
    for &c in &[c1, c2] {
        let h = mol.add_atom("H", false);
        mol.add_bond(c, h, 1);
    }
    mol.add_bond(m, c1, 1); // M-C σ
    mol.add_bond(m, c2, 1); // M-C σ
    let h1 = mol.add_atom("H", false);
    let h2 = mol.add_atom("H", false);
    mol.add_bond(m, h1, 1);
    mol.add_bond(m, h2, 1);
    (mol, m)
}

// ═══════════════════════════════════════════════════════════════
// Linear algebra
// ═══════════════════════════════════════════════════════════════

pub fn mat_mul(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; n * n];
    for i in 0..n {
        for k in 0..n {
            let aik = a[i * n + k];
            if aik == 0.0 {
                continue;
            }
            for j in 0..n {
                c[i * n + j] += aik * b[k * n + j];
            }
        }
    }
    c
}

pub fn mat_pow(p: &[f64], n: usize, tau: u32) -> Vec<f64> {
    let mut result = vec![0.0f64; n * n];
    for i in 0..n {
        result[i * n + i] = 1.0;
    }
    let mut base = p.to_vec();
    let mut exp = tau;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &base, n);
        }
        base = mat_mul(&base, &base, n);
        exp >>= 1;
    }
    result
}

/// Eigenvalues of a 4×4 symmetric matrix via Jacobi iteration.
pub fn eigenvalues_4x4(m: &[[f64; 4]; 4]) -> [f64; 4] {
    let mut a = *m;
    // Jacobi eigenvalue algorithm for small symmetric matrix
    for _ in 0..100 {
        // Find largest off-diagonal element
        let mut p = 0;
        let mut q = 1;
        let mut max_val = 0.0f64;
        for i in 0..4 {
            for j in (i + 1)..4 {
                if a[i][j].abs() > max_val {
                    max_val = a[i][j].abs();
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < 1e-12 {
            break;
        }
        // Compute rotation
        let theta = if (a[p][p] - a[q][q]).abs() < 1e-30 {
            std::f64::consts::FRAC_PI_4
        } else {
            0.5 * (2.0 * a[p][q] / (a[p][p] - a[q][q])).atan()
        };
        let c = theta.cos();
        let s = theta.sin();
        // Apply rotation
        let mut new_a = a;
        for i in 0..4 {
            new_a[i][p] = c * a[i][p] + s * a[i][q];
            new_a[i][q] = -s * a[i][p] + c * a[i][q];
        }
        let tmp = new_a;
        for j in 0..4 {
            new_a[p][j] = c * tmp[p][j] + s * tmp[q][j];
            new_a[q][j] = -s * tmp[p][j] + c * tmp[q][j];
        }
        a = new_a;
    }
    [a[0][0], a[1][1], a[2][2], a[3][3]]
}

pub fn shannon_entropy(dist: &[f64]) -> f64 {
    let mut h = 0.0f64;
    for &p in dist {
        if p > 1e-30 {
            h -= p * p.log2();
        }
    }
    h
}

// ═══════════════════════════════════════════════════════════════
// Statistics
// ═══════════════════════════════════════════════════════════════

pub fn pearson_r(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx: f64 = xs.iter().sum::<f64>() / n;
    let my: f64 = ys.iter().sum::<f64>() / n;
    let mut cov = 0.0;
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..xs.len() {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        cov += dx * dy;
        sx += dx * dx;
        sy += dy * dy;
    }
    if sx == 0.0 || sy == 0.0 {
        return 0.0;
    }
    cov / (sx.sqrt() * sy.sqrt())
}

pub fn spearman_rho(xs: &[f64], ys: &[f64]) -> f64 {
    fn ranks(vals: &[f64]) -> Vec<f64> {
        let mut indexed: Vec<(usize, f64)> = vals.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut r = vec![0.0; vals.len()];
        let mut start = 0;
        while start < indexed.len() {
            let mut end = start + 1;
            while end < indexed.len() && indexed[end].1 == indexed[start].1 {
                end += 1;
            }
            let average_rank = (start + 1 + end) as f64 / 2.0;
            for &(idx, _) in &indexed[start..end] {
                r[idx] = average_rank;
            }
            start = end;
        }
        r
    }
    pearson_r(&ranks(xs), &ranks(ys))
}

/// Count pairwise ordering accuracy: how many pairs are in the correct order?
/// Returns (correct, total, percentage).
pub fn pairwise_accuracy(values: &[f64], expected_ranks: &[usize]) -> (usize, usize, f64) {
    let mut indexed: Vec<(usize, f64)> = expected_ranks
        .iter()
        .copied()
        .zip(values.iter().copied())
        .collect();
    // Sort by value descending (highest S_τ first)
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut correct = 0;
    let mut total = 0;
    for i in 0..indexed.len() {
        for j in i + 1..indexed.len() {
            total += 1;
            if indexed[i].0 < indexed[j].0 {
                correct += 1;
            }
        }
    }
    let pct = if total > 0 {
        100.0 * correct as f64 / total as f64
    } else {
        100.0
    };
    (correct, total, pct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ethanol() {
        let mol = parse_smiles("CCO").unwrap();
        assert_eq!(mol.formula(), "C2H6O");
    }

    #[test]
    fn parse_benzene() {
        let mol = parse_smiles("c1ccccc1").unwrap();
        assert_eq!(mol.atoms.iter().filter(|a| a.symbol == "C").count(), 6);
        assert_eq!(mol.bonds.iter().filter(|b| b.order == 10).count(), 6);
    }

    #[test]
    fn h2_entropy_zero() {
        let mol = parse_smiles("[H][H]").unwrap();
        let s = mol.s_tau(3, false);
        assert!(s < 0.01, "H₂ should have near-zero S_τ, got {}", s);
    }

    #[test]
    fn hydrogenation_ordering() {
        // Pt > Cu in hydrogenation complex
        let (pt_mol, _) = hydrogenation_complex("Pt");
        let (cu_mol, _) = hydrogenation_complex("Cu");
        let s_pt = pt_mol.s_tau(3, true);
        let s_cu = cu_mol.s_tau(3, true);
        assert!(s_pt > s_cu, "Pt ({:.4}) should beat Cu ({:.4})", s_pt, s_cu);
    }

    #[test]
    fn bde_symmetric_lookup() {
        // bond_energy("C", "Pt", 1) should equal bond_energy("Pt", "C", 1)
        assert_eq!(bond_energy("C", "Pt", 1), bond_energy("Pt", "C", 1));
        assert_eq!(bond_energy("H", "Fe", 1), bond_energy("Fe", "H", 1));
    }

    #[test]
    fn weight_fn_identity_matches_existing_weighted_s_tau() {
        // Regression: passing the identity weight function to the new
        // s_tau_with_weight / s_tau_node_with_weight APIs must reproduce the
        // existing weighted s_tau / s_tau_node results exactly. Ensures the
        // γ Sabatier follow-up doesn't accidentally rewrite the baseline.
        let (mol, _) = hydrogenation_complex("Pt");
        let baseline_avg = mol.s_tau(3, true);
        let custom_avg = mol.s_tau_with_weight(3, |x| x);
        assert!(
            (baseline_avg - custom_avg).abs() < 1e-12,
            "identity weight_fn must reproduce baseline avg: {} vs {}",
            baseline_avg,
            custom_avg
        );
        let baseline_node = mol.s_tau_node(3, 0, true);
        let custom_node = mol.s_tau_node_with_weight(3, 0, |x| x);
        assert!(
            (baseline_node - custom_node).abs() < 1e-12,
            "identity weight_fn must reproduce baseline node: {} vs {}",
            baseline_node,
            custom_node
        );
    }

    #[test]
    fn bde_canonical_lookup_returns_real_values() {
        // Regression test for the bond_energy canonicalization bug. Many M-X
        // entries used to be written in non-canonical order (e.g. ("O", "Co"))
        // which never matched after alphabetical sort and silently fell through
        // to the default value (250). Spot-check that representative entries
        // for each affected metal-ligand family return their real values.
        // M-O (metals alphabetically before "O"):
        assert_eq!(bond_energy("O", "Co", 1), 190.0);
        assert_eq!(bond_energy("O", "Fe", 1), 210.0);
        assert_eq!(bond_energy("O", "Ni", 1), 180.0);
        assert_eq!(bond_energy("O", "Cu", 1), 270.0);
        assert_eq!(bond_energy("O", "Mn", 1), 280.0);
        assert_eq!(bond_energy("O", "Ir", 1), 280.0);
        // M-H (metals alphabetically before "H"):
        assert_eq!(bond_energy("H", "Co", 1), 230.0);
        assert_eq!(bond_energy("H", "Fe", 1), 200.0);
        assert_eq!(bond_energy("H", "Cu", 1), 220.0);
        assert_eq!(bond_energy("H", "Cr", 1), 190.0);
        // M-N:
        assert_eq!(bond_energy("N", "Fe", 1), 160.0);
        assert_eq!(bond_energy("N", "Ir", 1), 220.0);
        // M-P:
        assert_eq!(bond_energy("P", "Co", 1), 200.0);
        assert_eq!(bond_energy("P", "Fe", 1), 190.0);
        // P-Cl, P-F (non-metal halides):
        assert_eq!(bond_energy("P", "Cl", 1), 326.0);
        assert_eq!(bond_energy("P", "F", 1), 490.0);
        // None of these should equal the default fall-through value of 250.0
        // unless 250.0 is the real value (which is the case only for a few entries).
    }

    #[test]
    fn branching_isomers() {
        // butane > isobutane in S_τ
        let butane = parse_smiles("CCCC").unwrap();
        let isobutane = parse_smiles("CC(C)C").unwrap();
        let s_b = butane.s_tau(3, true);
        let s_i = isobutane.s_tau(3, true);
        assert!(
            s_b > s_i,
            "butane ({:.4}) should beat isobutane ({:.4})",
            s_b,
            s_i
        );
    }

    #[test]
    fn spearman_uses_average_ranks_for_ties() {
        let xs = [1.0, 2.0, 2.0, 4.0];
        let ys = [1.0, 2.0, 3.0, 4.0];
        let rho = spearman_rho(&xs, &ys);
        assert!((rho - 0.948_683_298_050_513_8).abs() < 1e-12);
    }
}
