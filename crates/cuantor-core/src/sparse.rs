//! Simulator „sparse”: stochează doar amplitudinile nenule. Exact pentru orice
//! poartă, cât timp numărul de ramuri de superpoziție rămâne mic (GHZ, Bernstein–Vazirani,
//! circuite aritmetice reversibile pe mii de qubiți).

use crate::complex::C64;
use crate::machine::Machine;
use crate::program::{Kind, Op};
use crate::{Result, Rng, err};
use std::collections::{HashMap, HashSet};

pub type Key = Box<[u64]>;
pub const DEFAULT_MAX_TERMS: usize = 1 << 21;

#[derive(Clone, Debug)]
pub struct Sparse {
    n: usize,
    w: usize,
    pub max_terms: usize,
    terms: HashMap<Key, C64>,
}

#[inline]
fn bit(k: &[u64], q: usize) -> bool {
    k[q / 64] >> (q % 64) & 1 == 1
}

impl Sparse {
    pub fn new(n: usize) -> Result<Sparse> {
        if n == 0 || n > crate::MAX_QUBITS_TOTAL {
            return Err(err!("număr de qubiți invalid: {n}"));
        }
        let mut s = Sparse { n, w: n.div_ceil(64), max_terms: DEFAULT_MAX_TERMS, terms: HashMap::new() };
        s.init();
        Ok(s)
    }
    fn init(&mut self) {
        self.terms.clear();
        self.terms.insert(vec![0u64; self.w].into_boxed_slice(), C64::ONE);
    }
    pub fn len(&self) -> usize {
        self.terms.len()
    }
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
    pub fn bitstring(&self, k: &[u64]) -> String {
        (0..self.n).rev().map(|q| if bit(k, q) { '1' } else { '0' }).collect()
    }
    pub fn prob_one(&self, q: usize) -> f64 {
        self.terms.iter().filter(|(k, _)| bit(k, q)).map(|(_, a)| a.norm_sqr()).sum()
    }
    /// Cele mai probabile `k` ramuri: (cheie, amplitudine).
    pub fn top(&self, k: usize) -> Vec<(Key, C64)> {
        let mut v: Vec<(Key, C64)> = self.terms.iter().map(|(a, b)| (a.clone(), *b)).collect();
        v.sort_by(|a, b| b.1.norm_sqr().partial_cmp(&a.1.norm_sqr()).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0)));
        v.truncate(k);
        v
    }
    /// Distribuția cumulativă pentru eșantionare: (chei, cdf).
    pub fn cdf(&self) -> (Vec<Key>, Vec<f64>) {
        let mut keys = Vec::with_capacity(self.terms.len());
        let mut cdf = Vec::with_capacity(self.terms.len());
        let mut acc = 0.0;
        let mut items: Vec<(&Key, &C64)> = self.terms.iter().collect();
        items.sort_by(|a, b| a.0.cmp(b.0));
        for (k, a) in items {
            acc += a.norm_sqr();
            keys.push(k.clone());
            cdf.push(acc);
        }
        (keys, cdf)
    }
    pub fn bit_of(key: &[u64], q: usize) -> bool {
        bit(key, q)
    }
    fn check_size(&self, len: usize) -> Result<()> {
        if len > self.max_terms {
            Err(err!(
                "prea multe ramuri de superpoziție ({len} > {}): circuitul nu este potrivit motorului sparse; reduceți numărul de porți H/rotații sau folosiți Stabilizator/Vector de stare",
                self.max_terms
            ))
        } else {
            Ok(())
        }
    }
}

impl Machine for Sparse {
    fn n_qubits(&self) -> usize {
        self.n
    }
    fn reset_all(&mut self) {
        self.init();
    }
    fn apply(&mut self, op: &Op) -> Result<()> {
        op.validate(self.n)?;
        let ctrl = &op.controls;
        let ok = |k: &[u64]| ctrl.iter().all(|&c| bit(k, c));
        match &op.kind {
            Kind::Swap => {
                let (a, b) = (op.targets[0], op.targets[1]);
                let mut next = HashMap::with_capacity(self.terms.len());
                for (k, amp) in self.terms.drain() {
                    let mut k = k;
                    if ok(&k) && bit(&k, a) != bit(&k, b) {
                        k[a / 64] ^= 1 << (a % 64);
                        k[b / 64] ^= 1 << (b % 64);
                    }
                    next.insert(k, amp);
                }
                self.terms = next;
            }
            kind => {
                let m = kind.matrix();
                let t = op.targets[0];
                let tm = 1u64 << (t % 64);
                let tw = t / 64;
                let diagonal = m[0][1] == C64::ZERO && m[1][0] == C64::ZERO;
                if diagonal {
                    for (k, amp) in self.terms.iter_mut() {
                        if ok(k) {
                            *amp = *amp * if bit(k, t) { m[1][1] } else { m[0][0] };
                        }
                    }
                    return Ok(());
                }
                let mut next: HashMap<Key, C64> = HashMap::with_capacity(self.terms.len() * 2);
                let mut done: HashSet<Key> = HashSet::new();
                for (k, &a) in &self.terms {
                    if !ok(k) {
                        next.insert(k.clone(), a);
                        continue;
                    }
                    let mut base = k.clone();
                    base[tw] &= !tm;
                    if !done.insert(base.clone()) {
                        continue;
                    }
                    let mut one = base.clone();
                    one[tw] |= tm;
                    let a0 = self.terms.get(&base).copied().unwrap_or(C64::ZERO);
                    let a1 = self.terms.get(&one).copied().unwrap_or(C64::ZERO);
                    let n0 = m[0][0] * a0 + m[0][1] * a1;
                    let n1 = m[1][0] * a0 + m[1][1] * a1;
                    if n0.norm_sqr() > 1e-26 {
                        next.insert(base, n0);
                    }
                    if n1.norm_sqr() > 1e-26 {
                        next.insert(one, n1);
                    }
                    self.check_size(next.len())?;
                }
                self.terms = next;
            }
        }
        Ok(())
    }
    fn measure(&mut self, q: usize, rng: &mut Rng) -> Result<bool> {
        let p1 = self.prob_one(q);
        let out = rng.next_f64() < p1;
        let norm = if out { p1 } else { 1.0 - p1 };
        let k = if norm > 0.0 { 1.0 / norm.sqrt() } else { 1.0 };
        self.terms.retain(|key, _| bit(key, q) == out);
        for a in self.terms.values_mut() {
            *a = *a * k;
        }
        Ok(out)
    }
}
