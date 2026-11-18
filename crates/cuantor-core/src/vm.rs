use crate::machine::{Backend, Machine, choose_backend, make_machine};
use crate::program::{Instr, Kind, Op, Program};
use crate::sparse::Sparse;
use crate::state::{State, sample_cdf, threads};
use crate::{Result, Rng, err};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Model de zgomot: depolarizare după fiecare poartă + eroare de citire.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NoiseModel {
    /// Probabilitatea unei erori Pauli aleatoare după o poartă cu un qubit (per qubit).
    pub depol1: f64,
    /// Idem pentru porți cu 2+ qubiți (aplicat fiecărui qubit implicat).
    pub depol2: f64,
    /// Probabilitatea de a inversa bitul citit la măsurare.
    pub readout: f64,
}

impl NoiseModel {
    pub fn is_ideal(&self) -> bool {
        self.depol1 == 0.0 && self.depol2 == 0.0 && self.readout == 0.0
    }
    pub fn gate_noise(&self) -> bool {
        self.depol1 > 0.0 || self.depol2 > 0.0
    }
}

/// Șir de biți clasici de lungime arbitrară (little-endian pe cuvinte de 64 de biți).
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bits(pub Vec<u64>);

impl Bits {
    pub fn zeros(n: usize) -> Bits {
        Bits(vec![0; n.div_ceil(64).max(1)])
    }
    pub fn from_u64(v: u64, n: usize) -> Bits {
        let mut b = Bits::zeros(n);
        b.0[0] = v;
        b
    }
    pub fn get(&self, i: usize) -> bool {
        self.0[i / 64] >> (i % 64) & 1 == 1
    }
    pub fn set(&mut self, i: usize, v: bool) {
        if v {
            self.0[i / 64] |= 1 << (i % 64);
        } else {
            self.0[i / 64] &= !(1 << (i % 64));
        }
    }
    /// Valoarea (≤ 64 biți) a câmpului [off, off+size).
    pub fn field(&self, off: usize, size: usize) -> u64 {
        (0..size.min(64)).fold(0u64, |a, k| a | ((self.get(off + k) as u64) << k))
    }
    /// Reprezentare cu bitul cel mai semnificativ primul.
    pub fn to_string_msb(&self, n: usize) -> String {
        (0..n).rev().map(|b| if self.get(b) { '1' } else { '0' }).collect()
    }
}

/// Rulează un program pe un motor, ca o traiectorie: întoarce biții clasici.
pub fn run_program(m: &mut dyn Machine, prog: &Program, noise: &NoiseModel, rng: &mut Rng) -> Result<Bits> {
    if prog.n_qubits != m.n_qubits() {
        return Err(err!("programul cere {} qubiți, motorul are {}", prog.n_qubits, m.n_qubits()));
    }
    m.reset_all();
    let mut cl = Bits::zeros(prog.n_clbits);
    for ins in &prog.instrs {
        match ins {
            Instr::Gate(op) => gate(m, op, noise, rng)?,
            Instr::Measure { q, c } => {
                let mut b = m.measure(*q, rng)?;
                if noise.readout > 0.0 && rng.next_f64() < noise.readout {
                    b = !b;
                }
                cl.set(*c, b);
            }
            Instr::Reset(q) => m.reset_qubit(*q, rng)?,
            Instr::If { creg, value, op } => {
                let cr = &prog.cregs[*creg];
                if cl.field(cr.offset, cr.size) == *value {
                    gate(m, op, noise, rng)?;
                }
            }
            Instr::Barrier => {}
        }
    }
    Ok(cl)
}

fn gate(m: &mut dyn Machine, op: &Op, noise: &NoiseModel, rng: &mut Rng) -> Result<()> {
    m.apply(op)?;
    if noise.gate_noise() {
        let qs = op.qubits();
        let p = if qs.len() > 1 { noise.depol2 } else { noise.depol1 };
        if p > 0.0 {
            for q in qs {
                if rng.next_f64() < p {
                    let k = match (rng.next_f64() * 3.0) as u32 {
                        0 => Kind::X,
                        1 => Kind::Y,
                        _ => Kind::Z,
                    };
                    m.apply(&Op::single(k, q))?;
                }
            }
        }
    }
    Ok(())
}

/// Mașina virtuală pe vector de stare: stare cuantică + registre clasice + RNG + zgomot.
pub struct Vm {
    pub state: State,
    /// Biții clasici de la ultima rulare.
    pub clbits: Bits,
    pub rng: Rng,
    pub noise: NoiseModel,
}

impl Vm {
    pub fn new(n_qubits: usize, rng: Rng, noise: NoiseModel) -> Result<Vm> {
        Ok(Vm { state: State::new(n_qubits)?, clbits: Bits::default(), rng, noise })
    }
    /// O rulare completă (o „traiectorie”): reinițializează și execută tot programul.
    pub fn run(&mut self, prog: &Program) -> Result<()> {
        self.clbits = run_program(&mut self.state, prog, &self.noise, &mut self.rng)?;
        Ok(())
    }
}

/// Starea finală a părții unitare (măsurările, reset-urile și if-urile sunt ignorate).
pub fn final_state(prog: &Program) -> Result<State> {
    prog.check()?;
    let mut st = State::new(prog.n_qubits)?;
    for op in prog.gates() {
        st.apply(op)?;
    }
    Ok(st)
}

#[derive(Clone, Debug, Default)]
pub struct Counts {
    pub shots: usize,
    pub n_clbits: usize,
    pub map: BTreeMap<Bits, usize>,
    /// Adevărat dacă s-au rulat traiectorii complete (circuit dinamic sau zgomot).
    pub trajectories: bool,
    pub backend: Backend,
}

impl Counts {
    pub fn bitstring(&self, key: &Bits) -> String {
        key.to_string_msb(self.n_clbits)
    }
    /// Rezultatele sortate descrescător după frecvență.
    pub fn sorted(&self) -> Vec<(Bits, usize)> {
        let mut v: Vec<_> = self.map.iter().map(|(k, c)| (k.clone(), *c)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        v
    }
    pub fn freq_bits(&self, key: &Bits) -> f64 {
        *self.map.get(key).unwrap_or(&0) as f64 / self.shots.max(1) as f64
    }
    /// Comoditate pentru ≤ 64 biți clasici.
    pub fn freq(&self, key: u64) -> f64 {
        self.freq_bits(&Bits::from_u64(key, self.n_clbits))
    }
}

fn is_static(prog: &Program) -> bool {
    let mut measured = vec![false; prog.n_qubits];
    for ins in &prog.instrs {
        match ins {
            Instr::Reset(_) | Instr::If { .. } => return false,
            Instr::Gate(op) => {
                if op.qubits().iter().any(|&q| measured[q]) {
                    return false;
                }
            }
            Instr::Measure { q, .. } => {
                if measured[*q] {
                    return false;
                }
                measured[*q] = true;
            }
            Instr::Barrier => {}
        }
    }
    true
}

pub fn sample(prog: &Program, shots: usize, noise: &NoiseModel, seed: Option<u64>) -> Result<Counts> {
    sample_with(prog, shots, noise, seed, Backend::Auto)
}

/// Rulează programul de `shots` ori și întoarce histograma biților clasici.
///
/// Dacă programul nu conține `measure`, toți qubiții sunt măsurați la final.
/// Circuitele statice fără zgomot de poartă pe vector de stare sau sparse se simulează
/// o singură dată și se eșantionează; restul se rulează ca traiectorii independente,
/// în paralel pe mai multe fire.
pub fn sample_with(prog: &Program, shots: usize, noise: &NoiseModel, seed: Option<u64>, req: Backend) -> Result<Counts> {
    prog.check()?;
    for (name, p) in [("depol1", noise.depol1), ("depol2", noise.depol2), ("readout", noise.readout)] {
        if !(0.0..=1.0).contains(&p) {
            return Err(err!("zgomot invalid: {name} = {p} (trebuie să fie o probabilitate în [0, 1])"));
        }
    }
    let backend = choose_backend(prog, req)?;
    let mut master = match seed {
        Some(s) => Rng::from_seed(s),
        None => Rng::from_entropy(),
    };
    let prog: Cow<Program> = if prog.has_measure() { Cow::Borrowed(prog) } else { Cow::Owned(prog.with_measure_all()) };
    let mut counts = Counts { shots, n_clbits: prog.n_clbits, map: BTreeMap::new(), trajectories: false, backend };
    if shots == 0 {
        return Ok(counts);
    }
    let fast = !noise.gate_noise() && is_static(&prog) && backend != Backend::Stabilizer;
    if fast {
        let mut map: Vec<(usize, usize)> = vec![];
        for ins in &prog.instrs {
            if let Instr::Measure { q, c } = ins {
                map.push((*q, *c));
            }
        }
        let emit = |counts: &mut Counts, get: &dyn Fn(usize) -> bool, rng: &mut Rng| {
            let mut bits = Bits::zeros(prog.n_clbits);
            for &(q, c) in &map {
                let mut b = get(q);
                if noise.readout > 0.0 && rng.next_f64() < noise.readout {
                    b = !b;
                }
                bits.set(c, b);
            }
            *counts.map.entry(bits).or_insert(0) += 1;
        };
        if backend == Backend::Sparse {
            let mut sp = Sparse::new(prog.n_qubits)?;
            for op in prog.gates() {
                sp.apply(op)?;
            }
            let (keys, cdf) = sp.cdf();
            for _ in 0..shots {
                let key = &keys[sample_cdf(&cdf, master.next_f64())];
                emit(&mut counts, &|q| Sparse::bit_of(key, q), &mut master);
            }
        } else {
            let mut st = State::new(prog.n_qubits)?;
            for op in prog.gates() {
                st.apply(op)?;
            }
            let cdf = st.cdf();
            for _ in 0..shots {
                let idx = sample_cdf(&cdf, master.next_f64());
                emit(&mut counts, &|q| idx >> q & 1 == 1, &mut master);
            }
        }
        return Ok(counts);
    }
    counts.trajectories = true;
    // Blocuri fixe de măsurători, fiecare cu propriul generator derivat în ordine: rezultatul depinde
    // doar de sămânță, nu de numărul de fire (reproductibilitate).
    const CHUNK: usize = 32;
    let n_chunks = shots.div_ceil(CHUNK);
    let rngs: Vec<Rng> = (0..n_chunks).map(|_| master.fork()).collect();
    let th = if backend == Backend::StateVector && prog.n_qubits > 14 { 1 } else { threads().min(n_chunks).max(1) };
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: Vec<Result<BTreeMap<Bits, usize>>> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..th)
            .map(|_| {
                let (prog, next, rngs) = (&*prog, &next, &rngs);
                let noise = noise.clone();
                s.spawn(move || -> Result<BTreeMap<Bits, usize>> {
                    let mut m = make_machine(backend, prog.n_qubits)?;
                    let mut map = BTreeMap::new();
                    loop {
                        let c = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if c >= n_chunks {
                            break;
                        }
                        let mut rng = rngs[c].clone();
                        let n = CHUNK.min(shots - c * CHUNK);
                        for _ in 0..n {
                            let cl = run_program(&mut *m, prog, &noise, &mut rng)?;
                            *map.entry(cl).or_insert(0usize) += 1;
                        }
                    }
                    Ok(map)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(err!("fir de execuție prăbușit")))).collect()
    });
    for r in results {
        for (k, v) in r? {
            *counts.map.entry(k).or_insert(0) += v;
        }
    }
    Ok(counts)
}

/// Cronometrează aplicarea lui H pe toți cei `n` qubiți (milisecunde).
pub fn bench_h_all(n: usize) -> Result<f64> {
    let mut st = State::new(n)?;
    let t0 = std::time::Instant::now();
    for q in 0..n {
        st.apply(&Op::single(Kind::H, q))?;
    }
    Ok(t0.elapsed().as_secs_f64() * 1000.0)
}
