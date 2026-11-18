use crate::complex::{C64, Mat2};
use crate::program::{Kind, Op};
use crate::{Result, Rng, err};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Limită de siguranță: 2^28 amplitudini × 16 octeți = 4 GiB.
pub const MAX_QUBITS: usize = 28;
const PAR_MIN: usize = 1 << 15;

static THREADS: AtomicUsize = AtomicUsize::new(0);

/// Număr de fire de execuție pentru porți mari (0 = automat).
pub fn set_threads(n: usize) {
    THREADS.store(n, Ordering::Relaxed);
}
pub fn threads() -> usize {
    let t = THREADS.load(Ordering::Relaxed);
    if t > 0 {
        return t;
    }
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).min(32)
}

#[derive(Clone, Copy)]
struct Ptr(*mut C64);
unsafe impl Send for Ptr {}
unsafe impl Sync for Ptr {}
impl Ptr {
    #[inline]
    fn get(self) -> *mut C64 {
        self.0
    }
}

fn par_range(total: usize, f: &(dyn Fn(usize, usize) + Sync)) {
    let th = threads();
    if total < PAR_MIN || th <= 1 {
        f(0, total);
        return;
    }
    let chunk = total.div_ceil(th);
    std::thread::scope(|s| {
        for t in 0..th {
            let a = t * chunk;
            let b = (a + chunk).min(total);
            if a >= b {
                break;
            }
            s.spawn(move || f(a, b));
        }
    });
}

/// Vectorul de stare al unui registru de `n` qubiți.
#[derive(Clone, Debug)]
pub struct State {
    n: usize,
    amp: Vec<C64>,
}

impl State {
    pub fn new(n: usize) -> Result<State> {
        if n == 0 || n > MAX_QUBITS {
            return Err(err!("număr de qubiți invalid: {n} (1..={MAX_QUBITS})"));
        }
        let len = 1usize << n;
        let mut amp: Vec<C64> = Vec::new();
        amp.try_reserve_exact(len).map_err(|_| err!("memorie insuficientă pentru {n} qubiți ({} MiB)", len * 16 >> 20))?;
        amp.resize(len, C64::ZERO);
        amp[0] = C64::ONE;
        Ok(State { n, amp })
    }
    pub fn n_qubits(&self) -> usize {
        self.n
    }
    pub fn amplitudes(&self) -> &[C64] {
        &self.amp
    }
    pub fn reset(&mut self) {
        self.amp.fill(C64::ZERO);
        self.amp[0] = C64::ONE;
    }
    pub fn norm_sqr(&self) -> f64 {
        self.amp.iter().map(|a| a.norm_sqr()).sum()
    }

    pub fn apply(&mut self, op: &Op) -> Result<()> {
        op.validate(self.n)?;
        let cmask = op.controls.iter().fold(0usize, |m, &c| m | (1 << c));
        match op.kind {
            Kind::Swap => self.apply_swap(op.targets[0], op.targets[1], cmask),
            ref k => self.apply_matrix(&k.matrix(), op.targets[0], cmask),
        }
        Ok(())
    }

    /// Aplică o matrice 2×2 pe qubitul `t`, condiționat de toate biturile din `cmask`.
    pub fn apply_matrix(&mut self, m: &Mat2, t: usize, cmask: usize) {
        let pairs = self.amp.len() / 2;
        let ptr = Ptr(self.amp.as_mut_ptr());
        let (m00, m01, m10, m11) = (m[0][0], m[0][1], m[1][0], m[1][1]);
        let low = (1usize << t) - 1;
        let bt = 1usize << t;
        par_range(pairs, &|a, b| {
            let p = ptr.get();
            for k in a..b {
                let i0 = ((k >> t) << (t + 1)) | (k & low);
                if i0 & cmask != cmask {
                    continue;
                }
                let i1 = i0 | bt;
                // SAFETY: perechile (i0,i1) sunt disjuncte pentru k diferiți, iar
                // intervalele [a,b) ale firelor nu se suprapun; indicii < len.
                unsafe {
                    let x = *p.add(i0);
                    let y = *p.add(i1);
                    *p.add(i0) = m00 * x + m01 * y;
                    *p.add(i1) = m10 * x + m11 * y;
                }
            }
        });
    }

    pub fn apply_swap(&mut self, a: usize, b: usize, cmask: usize) {
        let len = self.amp.len();
        let ptr = Ptr(self.amp.as_mut_ptr());
        let (ba, bb) = (1usize << a, 1usize << b);
        par_range(len, &|lo, hi| {
            let p = ptr.get();
            for i in lo..hi {
                if i & ba != 0 && i & bb == 0 && i & cmask == cmask {
                    let j = (i ^ ba) ^ bb;
                    // SAFETY: fiecare pereche {i,j} este vizitată o singură dată (din i cu bit a=1, b=0).
                    unsafe {
                        std::ptr::swap(p.add(i), p.add(j));
                    }
                }
            }
        });
    }

    pub fn probabilities(&self) -> Vec<f64> {
        self.amp.iter().map(|a| a.norm_sqr()).collect()
    }
    /// Probabilitatea ca qubitul `q` să fie măsurat 1.
    pub fn prob_one(&self, q: usize) -> f64 {
        let b = 1usize << q;
        self.amp.iter().enumerate().filter(|(i, _)| i & b != 0).map(|(_, a)| a.norm_sqr()).sum()
    }
    /// Probabilități marginale pentru un subset de qubiți (qubits[0] = bitul 0 al indexului).
    pub fn marginal(&self, qubits: &[usize]) -> Vec<f64> {
        let mut out = vec![0.0; 1 << qubits.len()];
        for (i, a) in self.amp.iter().enumerate() {
            let mut k = 0usize;
            for (j, &q) in qubits.iter().enumerate() {
                k |= ((i >> q) & 1) << j;
            }
            out[k] += a.norm_sqr();
        }
        out
    }
    /// Vectorul Bloch (x,y,z) al stării redus a qubitului `k`. Lungime < 1 ⇒ încâlcit.
    pub fn bloch(&self, k: usize) -> [f64; 3] {
        let b = 1usize << k;
        let (mut rr, mut ri, mut z) = (0.0, 0.0, 0.0);
        for i in 0..self.amp.len() {
            if i & b != 0 {
                continue;
            }
            let a0 = self.amp[i];
            let a1 = self.amp[i | b];
            rr += a0.re * a1.re + a0.im * a1.im;
            ri += a0.im * a1.re - a0.re * a1.im;
            z += a0.norm_sqr() - a1.norm_sqr();
        }
        [2.0 * rr, -2.0 * ri, z]
    }
    /// ⟨ψ|P|ψ⟩ pentru un produs de Pauli, ex. [(0,'Z'),(1,'Z')].
    pub fn expect_pauli(&self, paulis: &[(usize, char)]) -> Result<f64> {
        let mut t = self.clone();
        for &(q, p) in paulis {
            let kind = match p.to_ascii_uppercase() {
                'X' => Kind::X,
                'Y' => Kind::Y,
                'Z' => Kind::Z,
                'I' => continue,
                c => return Err(err!("Pauli necunoscut: {c}")),
            };
            t.apply(&Op::single(kind, q))?;
        }
        let mut s = 0.0;
        for (a, b) in self.amp.iter().zip(t.amp.iter()) {
            s += a.re * b.re + a.im * b.im;
        }
        Ok(s)
    }
    /// Măsoară qubitul `q` în baza Z, prăbușind starea.
    pub fn measure(&mut self, q: usize, rng: &mut Rng) -> bool {
        let p1 = self.prob_one(q);
        let one = rng.next_f64() < p1;
        let b = 1usize << q;
        let norm = if one { p1 } else { 1.0 - p1 };
        let k = if norm > 0.0 { 1.0 / norm.sqrt() } else { 1.0 };
        for (i, a) in self.amp.iter_mut().enumerate() {
            if (i & b != 0) == one {
                *a = *a * k;
            } else {
                *a = C64::ZERO;
            }
        }
        one
    }
    pub fn reset_qubit(&mut self, q: usize, rng: &mut Rng) {
        if self.measure(q, rng) {
            let _ = self.apply(&Op::single(Kind::X, q));
        }
    }
    /// Un rezultat de măsurare pe toți qubiții, fără a modifica starea.
    pub fn sample_index(&self, rng: &mut Rng) -> usize {
        let u = rng.next_f64();
        let mut acc = 0.0;
        for (i, a) in self.amp.iter().enumerate() {
            acc += a.norm_sqr();
            if u < acc {
                return i;
            }
        }
        self.amp.len() - 1
    }
    pub fn cdf(&self) -> Vec<f64> {
        let mut acc = 0.0;
        self.amp
            .iter()
            .map(|a| {
                acc += a.norm_sqr();
                acc
            })
            .collect()
    }
}

/// Căutare binară în CDF.
pub fn sample_cdf(cdf: &[f64], u: f64) -> usize {
    let t = u * cdf[cdf.len() - 1];
    cdf.partition_point(|&c| c <= t).min(cdf.len() - 1)
}

/// Entropia von Neumann (în biți) a unui qubit cu vector Bloch de lungime `r`.
pub fn bloch_entropy(r: f64) -> f64 {
    let h = |p: f64| if p > 1e-12 { -p * p.log2() } else { 0.0 };
    h((1.0 + r) / 2.0) + h((1.0 - r) / 2.0)
}
