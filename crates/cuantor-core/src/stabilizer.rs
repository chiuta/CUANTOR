//! Simulator de stabilizatori (Aaronson–Gottesman, „CHP”): exact și polinomial
//! pentru circuite Clifford. Permite mii de qubiți (memorie ≈ 4n²/8 octeți).

use crate::machine::Machine;
use crate::program::{Kind, Op};
use crate::{Result, Rng, err};
use std::f64::consts::FRAC_PI_2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prim {
    H(usize),
    S(usize),
    Cx(usize, usize),
}

fn quarter(a: f64) -> Option<usize> {
    let k = a / FRAC_PI_2;
    let r = k.round();
    if (k - r).abs() < 1e-9 { Some((r as i64).rem_euclid(4) as usize) } else { None }
}
fn s_pow(q: usize, k: usize) -> Vec<Prim> {
    vec![Prim::S(q); k]
}

/// Descompune o poartă Clifford în {H, S, CNOT} (până la o fază globală pentru porți necontrolate).
pub fn decompose(op: &Op) -> Option<Vec<Prim>> {
    use Prim::*;
    let t = *op.targets.first()?;
    match op.controls.len() {
        0 => Some(match &op.kind {
            Kind::H => vec![H(t)],
            Kind::S => vec![S(t)],
            Kind::Sdg => s_pow(t, 3),
            Kind::Z => s_pow(t, 2),
            Kind::X => vec![H(t), S(t), S(t), H(t)],
            Kind::Y => vec![S(t), S(t), H(t), S(t), S(t), H(t)],
            Kind::Sx => vec![H(t), S(t), H(t)],
            Kind::T | Kind::Tdg => return None,
            Kind::Rz(a) | Kind::P(a) => s_pow(t, quarter(*a)?),
            Kind::Rx(a) => [vec![H(t)], s_pow(t, quarter(*a)?), vec![H(t)]].concat(),
            Kind::Ry(a) => [s_pow(t, 3), vec![H(t)], s_pow(t, quarter(*a)?), vec![H(t)], vec![S(t)]].concat(),
            Kind::U(th, ph, la) => {
                let (kt, kp, kl) = (quarter(*th)?, quarter(*ph)?, quarter(*la)?);
                [s_pow(t, kl), s_pow(t, 3), vec![H(t)], s_pow(t, kt), vec![H(t)], vec![S(t)], s_pow(t, kp)].concat()
            }
            Kind::Swap => {
                let b = *op.targets.get(1)?;
                vec![Cx(t, b), Cx(b, t), Cx(t, b)]
            }
        }),
        1 => {
            let a = op.controls[0];
            Some(match &op.kind {
                Kind::X => vec![Cx(a, t)],
                Kind::Z => vec![H(t), Cx(a, t), H(t)],
                Kind::Y => vec![S(t), S(t), S(t), Cx(a, t), S(t)],
                Kind::P(th) => match quarter(*th)? {
                    0 => vec![],
                    2 => vec![H(t), Cx(a, t), H(t)],
                    _ => return None,
                },
                Kind::Rz(th) => match quarter(*th) {
                    Some(0) => vec![],
                    _ => return None,
                },
                _ => return None,
            })
        }
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub struct Stabilizer {
    n: usize,
    w: usize,
    x: Vec<u64>,
    z: Vec<u64>,
    r: Vec<u8>,
}

fn two_rows(v: &mut [u64], w: usize, h: usize, i: usize) -> (&mut [u64], &[u64]) {
    if h < i {
        let (a, b) = v.split_at_mut(i * w);
        (&mut a[h * w..h * w + w], &b[..w])
    } else {
        let (a, b) = v.split_at_mut(h * w);
        (&mut b[..w], &a[i * w..i * w + w])
    }
}

/// h ← h · i (produs de operatori Pauli), actualizând faza.
fn rowsum_slices(xh: &mut [u64], zh: &mut [u64], rh: &mut u8, xi: &[u64], zi: &[u64], ri: u8) {
    let mut sum: i64 = 0;
    for k in 0..xh.len() {
        let (x1, z1, x2, z2) = (xi[k], zi[k], xh[k], zh[k]);
        let pos = (x1 & z1 & !x2 & z2) | (x1 & !z1 & x2 & z2) | (!x1 & z1 & x2 & !z2);
        let neg = (x1 & z1 & x2 & !z2) | (x1 & !z1 & !x2 & z2) | (!x1 & z1 & x2 & z2);
        sum += pos.count_ones() as i64 - neg.count_ones() as i64;
    }
    let total = 2 * (*rh as i64) + 2 * (ri as i64) + sum;
    *rh = (total.rem_euclid(4) == 2) as u8;
    for k in 0..xh.len() {
        xh[k] ^= xi[k];
        zh[k] ^= zi[k];
    }
}

impl Stabilizer {
    pub fn new(n: usize) -> Result<Stabilizer> {
        if n == 0 || n > crate::MAX_QUBITS_TOTAL {
            return Err(err!("număr de qubiți invalid: {n}"));
        }
        let w = n.div_ceil(64);
        let rows = 2 * n + 1;
        let mut s = Stabilizer { n, w, x: vec![0; rows * w], z: vec![0; rows * w], r: vec![0; rows] };
        s.init();
        Ok(s)
    }
    fn init(&mut self) {
        self.x.fill(0);
        self.z.fill(0);
        self.r.fill(0);
        for i in 0..self.n {
            self.x[i * self.w + i / 64] |= 1 << (i % 64);
            self.z[(self.n + i) * self.w + i / 64] |= 1 << (i % 64);
        }
    }
    pub fn memory_bytes(&self) -> usize {
        (self.x.len() + self.z.len()) * 8 + self.r.len()
    }
    #[inline]
    fn xb(&self, row: usize, c: usize) -> bool {
        self.x[row * self.w + c / 64] >> (c % 64) & 1 == 1
    }
    #[inline]
    fn zb(&self, row: usize, c: usize) -> bool {
        self.z[row * self.w + c / 64] >> (c % 64) & 1 == 1
    }
    pub fn h(&mut self, a: usize) {
        let (wi, m) = (a / 64, 1u64 << (a % 64));
        for row in 0..2 * self.n {
            let i = row * self.w + wi;
            let (xv, zv) = (self.x[i] & m != 0, self.z[i] & m != 0);
            if xv && zv {
                self.r[row] ^= 1;
            }
            if xv != zv {
                self.x[i] ^= m;
                self.z[i] ^= m;
            }
        }
    }
    pub fn s(&mut self, a: usize) {
        let (wi, m) = (a / 64, 1u64 << (a % 64));
        for row in 0..2 * self.n {
            let i = row * self.w + wi;
            if self.x[i] & m != 0 {
                if self.z[i] & m != 0 {
                    self.r[row] ^= 1;
                }
                self.z[i] ^= m;
            }
        }
    }
    pub fn cx(&mut self, a: usize, b: usize) {
        let (wa, ma) = (a / 64, 1u64 << (a % 64));
        let (wb, mb) = (b / 64, 1u64 << (b % 64));
        for row in 0..2 * self.n {
            let base = row * self.w;
            let xa = self.x[base + wa] & ma != 0;
            let zb = self.z[base + wb] & mb != 0;
            let xb = self.x[base + wb] & mb != 0;
            let za = self.z[base + wa] & ma != 0;
            if xa && zb && (xb == za) {
                self.r[row] ^= 1;
            }
            if xa {
                self.x[base + wb] ^= mb;
            }
            if zb {
                self.z[base + wa] ^= ma;
            }
        }
    }
    fn rowsum(&mut self, h: usize, i: usize) {
        let w = self.w;
        let (xh, xi) = two_rows(&mut self.x, w, h, i);
        let (zh, zi) = two_rows(&mut self.z, w, h, i);
        let mut rh = self.r[h];
        rowsum_slices(xh, zh, &mut rh, xi, zi, self.r[i]);
        self.r[h] = rh;
    }
    /// Rezultatul determinist al măsurării Z pe `a`, dacă există.
    pub fn z_deterministic(&self, a: usize) -> Option<bool> {
        let n = self.n;
        if (n..2 * n).any(|p| self.xb(p, a)) {
            return None;
        }
        let mut xh = vec![0u64; self.w];
        let mut zh = vec![0u64; self.w];
        let mut rh = 0u8;
        for i in 0..n {
            if self.xb(i, a) {
                let row = n + i;
                rowsum_slices(&mut xh, &mut zh, &mut rh, &self.x[row * self.w..(row + 1) * self.w], &self.z[row * self.w..(row + 1) * self.w], self.r[row]);
            }
        }
        Some(rh == 1)
    }
    /// P(qubit a = 1) fără a modifica starea: 0, ½ sau 1.
    pub fn prob_one(&self, a: usize) -> f64 {
        match self.z_deterministic(a) {
            None => 0.5,
            Some(true) => 1.0,
            Some(false) => 0.0,
        }
    }
    pub fn measure_z(&mut self, a: usize, rng: &mut Rng) -> bool {
        let n = self.n;
        let w = self.w;
        if let Some(p) = (n..2 * n).find(|&p| self.xb(p, a)) {
            for i in 0..2 * n {
                if i != p && self.xb(i, a) {
                    self.rowsum(i, p);
                }
            }
            let (src, dst) = (p * w, (p - n) * w);
            for k in 0..w {
                self.x[dst + k] = self.x[src + k];
                self.z[dst + k] = self.z[src + k];
            }
            self.r[p - n] = self.r[p];
            for k in 0..w {
                self.x[src + k] = 0;
                self.z[src + k] = 0;
            }
            self.z[src + a / 64] |= 1 << (a % 64);
            let out = rng.next_f64() < 0.5;
            self.r[p] = out as u8;
            out
        } else {
            let sc = 2 * n;
            for k in 0..w {
                self.x[sc * w + k] = 0;
                self.z[sc * w + k] = 0;
            }
            self.r[sc] = 0;
            for i in 0..n {
                if self.xb(i, a) {
                    self.rowsum(sc, n + i);
                }
            }
            self.r[sc] == 1
        }
    }
    /// Generatorii stabilizatorilor ca șiruri Pauli (ex. "+XX", "-ZI"), primii `max`.
    pub fn stabilizer_strings(&self, max: usize) -> Vec<String> {
        (0..self.n.min(max))
            .map(|i| {
                let row = self.n + i;
                let mut s = String::from(if self.r[row] == 1 { "-" } else { "+" });
                for q in 0..self.n.min(64) {
                    s.push(match (self.xb(row, q), self.zb(row, q)) {
                        (false, false) => 'I',
                        (true, false) => 'X',
                        (false, true) => 'Z',
                        (true, true) => 'Y',
                    });
                }
                if self.n > 64 {
                    s.push('…');
                }
                s
            })
            .collect()
    }
}

impl Machine for Stabilizer {
    fn n_qubits(&self) -> usize {
        self.n
    }
    fn reset_all(&mut self) {
        self.init();
    }
    fn apply(&mut self, op: &Op) -> Result<()> {
        op.validate(self.n)?;
        let prims = decompose(op).ok_or_else(|| err!("poarta {} nu este Clifford: nu poate fi simulată cu stabilizatorul", op.label()))?;
        for p in prims {
            match p {
                Prim::H(a) => self.h(a),
                Prim::S(a) => self.s(a),
                Prim::Cx(a, b) => self.cx(a, b),
            }
        }
        Ok(())
    }
    fn measure(&mut self, q: usize, rng: &mut Rng) -> Result<bool> {
        Ok(self.measure_z(q, rng))
    }
}
