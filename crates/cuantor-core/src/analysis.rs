//! Analiza stării pentru interfață: probabilități, vectori Bloch, ramuri dominante.

use crate::complex::C64;
use crate::machine::{Backend, Machine, choose_backend};
use crate::program::Program;
use crate::sparse::Sparse;
use crate::stabilizer::Stabilizer;
use crate::state::State;
use crate::{Result, Rng, err};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(Clone, Debug)]
pub struct TopState {
    /// Șir de biți, qubitul cel mai mare primul.
    pub bits: String,
    pub prob: f64,
    /// Amplitudinea exactă (None pentru stabilizator, unde faza nu e urmărită).
    pub amp: Option<C64>,
}

#[derive(Clone, Debug)]
pub struct Analysis {
    pub backend: Backend,
    pub n: usize,
    /// P(qubit = 1) pentru fiecare qubit.
    pub p_one: Vec<f64>,
    /// Cele mai probabile stări (exacte, sau eșantionate pentru stabilizator).
    pub top: Vec<TopState>,
    pub top_exact: bool,
    /// Numărul de amplitudini nenule, dacă e cunoscut.
    pub support: Option<usize>,
    /// Vectori Bloch (doar vector de stare cu n ≤ 12).
    pub bloch: Option<Vec<[f64; 3]>>,
    pub stabilizers: Vec<String>,
    pub memory_bytes: u128,
    pub samples_used: usize,
}

/// Analizează partea unitară a programului (măsurările/reset/if sunt ignorate).
pub fn analyze(prog: &Program, req: Backend, top_k: usize, sample_shots: usize, seed: Option<u64>) -> Result<Analysis> {
    prog.check()?;
    let backend = choose_backend(prog, req)?;
    let n = prog.n_qubits;
    let mut rng = match seed {
        Some(s) => Rng::from_seed(s),
        None => Rng::from_entropy(),
    };
    match backend {
        Backend::StateVector | Backend::Auto => {
            let mut st = State::new(n)?;
            for op in prog.gates() {
                st.apply(op)?;
            }
            let amp = st.amplitudes();
            let mut p_one = vec![0.0; n];
            let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
            let mut support = 0usize;
            for (i, a) in amp.iter().enumerate() {
                let p = a.norm_sqr();
                if p <= 1e-14 {
                    continue;
                }
                support += 1;
                let mut bits = i;
                while bits != 0 {
                    let q = bits.trailing_zeros() as usize;
                    p_one[q] += p;
                    bits &= bits - 1;
                }
                if heap.len() < top_k {
                    heap.push(Reverse((p.to_bits(), i)));
                } else if let Some(Reverse((m, _))) = heap.peek() {
                    if p.to_bits() > *m {
                        heap.pop();
                        heap.push(Reverse((p.to_bits(), i)));
                    }
                }
            }
            let mut top: Vec<TopState> = heap
                .into_iter()
                .map(|Reverse((_, i))| TopState {
                    bits: (0..n).rev().map(|q| if i >> q & 1 == 1 { '1' } else { '0' }).collect(),
                    prob: amp[i].norm_sqr(),
                    amp: Some(amp[i]),
                })
                .collect();
            top.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.bits.cmp(&b.bits)));
            let bloch = if n <= 12 { Some((0..n).map(|q| st.bloch(q)).collect()) } else { None };
            Ok(Analysis {
                backend: Backend::StateVector,
                n,
                p_one,
                top,
                top_exact: true,
                support: Some(support),
                bloch,
                stabilizers: vec![],
                memory_bytes: 16u128 << n,
                samples_used: 0,
            })
        }
        Backend::Sparse => {
            let mut sp = Sparse::new(n)?;
            for op in prog.gates() {
                sp.apply(op)?;
            }
            let p_one: Vec<f64> = (0..n).map(|q| sp.prob_one(q)).collect();
            let top = sp.top(top_k).into_iter().map(|(k, a)| TopState { bits: sp.bitstring(&k), prob: a.norm_sqr(), amp: Some(a) }).collect();
            Ok(Analysis {
                backend,
                n,
                p_one,
                top,
                top_exact: true,
                support: Some(sp.len()),
                bloch: None,
                stabilizers: vec![],
                memory_bytes: (sp.len() as u128) * (16 + 8 * n.div_ceil(64) as u128 + 48),
                samples_used: 0,
            })
        }
        Backend::Stabilizer => {
            let mut sb = Stabilizer::new(n)?;
            for op in prog.gates() {
                sb.apply(op)?;
            }
            let p_one: Vec<f64> = (0..n).map(|q| sb.prob_one(q)).collect();
            // eșantionare adaptată la cost: ~ n²·w operații pe măsurătoare completă
            let w = n.div_ceil(64) as f64;
            let cost = (n as f64) * (n as f64) * w;
            let shots = ((3e8 / cost.max(1.0)) as usize).clamp(8, sample_shots.max(8));
            let th = crate::state::threads().min(shots).max(1);
            let per = shots.div_ceil(th);
            let jobs: Vec<(usize, Rng)> = (0..th).map(|t| (per.min(shots - (t * per).min(shots)), rng.fork())).collect();
            let hist: Vec<BTreeMap<Vec<u64>, usize>> = std::thread::scope(|s| {
                let hs: Vec<_> = jobs
                    .into_iter()
                    .map(|(cnt, mut r)| {
                        let base = &sb;
                        s.spawn(move || {
                            let mut m = BTreeMap::new();
                            for _ in 0..cnt {
                                let mut c = base.clone();
                                let mut key = vec![0u64; n.div_ceil(64)];
                                for q in 0..n {
                                    if c.measure_z(q, &mut r) {
                                        key[q / 64] |= 1 << (q % 64);
                                    }
                                }
                                *m.entry(key).or_insert(0usize) += 1;
                            }
                            m
                        })
                    })
                    .collect();
                hs.into_iter().map(|h| h.join().unwrap_or_default()).collect()
            });
            let mut total: BTreeMap<Vec<u64>, usize> = BTreeMap::new();
            let mut used = 0;
            for h in hist {
                for (k, v) in h {
                    *total.entry(k).or_insert(0) += v;
                    used += v;
                }
            }
            if used == 0 {
                return Err(err!("eșantionarea stabilizatorului a eșuat"));
            }
            let mut v: Vec<_> = total.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            let top = v
                .into_iter()
                .take(top_k)
                .map(|(k, c)| TopState {
                    bits: (0..n).rev().map(|q| if k[q / 64] >> (q % 64) & 1 == 1 { '1' } else { '0' }).collect(),
                    prob: c as f64 / used as f64,
                    amp: None,
                })
                .collect();
            Ok(Analysis {
                backend,
                n,
                p_one,
                top,
                top_exact: false,
                support: None,
                bloch: None,
                stabilizers: sb.stabilizer_strings(8),
                memory_bytes: sb.memory_bytes() as u128,
                samples_used: used,
            })
        }
    }
}
