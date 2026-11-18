//! Transpilare: circuit logic → porți native IBM {rz, sx, x} + {cx | cz | ecr}, cu rutare SWAP pe harta de cuplare.

use cuantor_core::complex::{C64, Mat2};
use cuantor_core::{Instr, Kind, Op, Program};
use std::collections::VecDeque;
use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Native {
    Rz(usize, f64),
    Sx(usize),
    X(usize),
    Cx(usize, usize),
    Cz(usize, usize),
    Ecr(usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basis {
    Cx,
    Cz,
    Ecr,
}
impl Basis {
    pub fn from_gates(g: &[String]) -> Basis {
        if g.iter().any(|x| x == "ecr") {
            Basis::Ecr
        } else if g.iter().any(|x| x == "cz") {
            Basis::Cz
        } else {
            Basis::Cx
        }
    }
}

pub struct Transpiled {
    pub gates: Vec<Native>,
    /// (qubit fizic, bit clasic)
    pub measures: Vec<(usize, usize)>,
    pub n_clbits: usize,
    /// poziția finală fizică a fiecărui qubit logic
    pub final_map: Vec<usize>,
    pub swaps: usize,
}

impl Transpiled {
    pub fn two_qubit_count(&self) -> usize {
        self.gates.iter().filter(|g| matches!(g, Native::Cx(..) | Native::Cz(..) | Native::Ecr(..))).count()
    }
    /// OpenQASM 3 cu qubiți fizici ($k) — formatul acceptat de Runtime pentru circuite ISA.
    pub fn to_qasm3(&self) -> String {
        let mut s = String::from("OPENQASM 3.0;\ninclude \"stdgates.inc\";\n");
        s.push_str(&format!("bit[{}] c;\n", self.n_clbits.max(1)));
        for g in &self.gates {
            match g {
                Native::Rz(q, a) => s.push_str(&format!("rz({a:.15}) ${q};\n")),
                Native::Sx(q) => s.push_str(&format!("sx ${q};\n")),
                Native::X(q) => s.push_str(&format!("x ${q};\n")),
                Native::Cx(a, b) => s.push_str(&format!("cx ${a}, ${b};\n")),
                Native::Cz(a, b) => s.push_str(&format!("cz ${a}, ${b};\n")),
                Native::Ecr(a, b) => s.push_str(&format!("ecr ${a}, ${b};\n")),
            }
        }
        for (q, c) in &self.measures {
            s.push_str(&format!("c[{c}] = measure ${q};\n"));
        }
        s
    }
}

// ------------------------------------------------------------------ algebră 2x2

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}
pub fn mul(a: &Mat2, b: &Mat2) -> Mat2 {
    let mut r = [[C64::ZERO; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    r
}
pub fn rz_m(a: f64) -> Mat2 {
    [[C64::from_polar(1.0, -a / 2.0), C64::ZERO], [C64::ZERO, C64::from_polar(1.0, a / 2.0)]]
}
pub fn ry_m(a: f64) -> Mat2 {
    let (s, co) = ((a / 2.0).sin(), (a / 2.0).cos());
    [[c(co, 0.0), c(-s, 0.0)], [c(s, 0.0), c(co, 0.0)]]
}
const ID: Mat2 = [[C64::ONE, C64::ZERO], [C64::ZERO, C64::ONE]];

/// U = e^{iα} Rz(β) Ry(γ) Rz(δ)
pub fn zyz(u: &Mat2) -> (f64, f64, f64, f64) {
    let det = u[0][0] * u[1][1] - u[0][1] * u[1][0];
    let alpha = det.arg() / 2.0;
    let ph = C64::from_polar(1.0, -alpha);
    let v00 = u[0][0] * ph;
    let v10 = u[1][0] * ph;
    let gamma = 2.0 * v10.norm_sqr().sqrt().atan2(v00.norm_sqr().sqrt());
    let sum = if v00.norm_sqr() > 1e-24 { -2.0 * v00.arg() } else { 0.0 };
    let diff = if v10.norm_sqr() > 1e-24 { 2.0 * v10.arg() } else { 0.0 };
    ((alpha), (sum + diff) / 2.0, gamma, (sum - diff) / 2.0)
}

/// Emite o poartă de 1 qubit ca rz/sx (până la o fază globală).
fn emit_1q(out: &mut Vec<Native>, q: usize, u: &Mat2) {
    let (_, beta, gamma, delta) = zyz(u);
    let push_rz = |out: &mut Vec<Native>, a: f64| {
        let a = (a + PI).rem_euclid(2.0 * PI) - PI;
        if a.abs() > 1e-12 {
            out.push(Native::Rz(q, a));
        }
    };
    if gamma.abs() < 1e-12 {
        push_rz(out, beta + delta);
    } else if (gamma - PI).abs() < 1e-12 {
        // Rz(β) Ry(π) Rz(δ) ≃ Rz(β−δ+π) X  (în ordine temporală: X apoi Rz)
        out.push(Native::X(q));
        push_rz(out, beta - delta + PI);
    } else {
        // Rz(β)Ry(γ)Rz(δ) ≃ Rz(β+π) SX Rz(γ+π) SX Rz(δ)   (ordine temporală: δ întâi)
        push_rz(out, delta);
        out.push(Native::Sx(q));
        push_rz(out, gamma + PI);
        out.push(Native::Sx(q));
        push_rz(out, beta + PI);
    }
}

// ------------------------------------------------------------------ descompunere logică

#[derive(Clone, Debug)]
enum Lg {
    One(usize, Mat2),
    Cx(usize, usize),
}

fn h_m() -> Mat2 {
    Kind::H.matrix()
}

fn controlled_1q(out: &mut Vec<Lg>, ctrl: usize, t: usize, u: &Mat2) {
    let (alpha, beta, gamma, delta) = zyz(u);
    let a = mul(&rz_m(beta), &ry_m(gamma / 2.0));
    let b = mul(&ry_m(-gamma / 2.0), &rz_m(-(delta + beta) / 2.0));
    let cc = rz_m((delta - beta) / 2.0);
    out.push(Lg::One(t, cc));
    out.push(Lg::Cx(ctrl, t));
    out.push(Lg::One(t, b));
    out.push(Lg::Cx(ctrl, t));
    out.push(Lg::One(t, a));
    // P(α) pe control
    out.push(Lg::One(ctrl, [[C64::ONE, C64::ZERO], [C64::ZERO, C64::from_polar(1.0, alpha)]]));
}

fn ccx(out: &mut Vec<Lg>, c1: usize, c2: usize, t: usize) {
    let h = h_m();
    let tg = Kind::T.matrix();
    let td = Kind::Tdg.matrix();
    out.push(Lg::One(t, h));
    out.push(Lg::Cx(c2, t));
    out.push(Lg::One(t, td));
    out.push(Lg::Cx(c1, t));
    out.push(Lg::One(t, tg));
    out.push(Lg::Cx(c2, t));
    out.push(Lg::One(t, td));
    out.push(Lg::Cx(c1, t));
    out.push(Lg::One(c2, tg));
    out.push(Lg::One(t, tg));
    out.push(Lg::One(t, h));
    out.push(Lg::Cx(c1, c2));
    out.push(Lg::One(c1, tg));
    out.push(Lg::One(c2, td));
    out.push(Lg::Cx(c1, c2));
}

fn lower_op(out: &mut Vec<Lg>, op: &Op) -> Result<(), String> {
    let nc = op.controls.len();
    match (&op.kind, nc) {
        (Kind::Swap, 0) => {
            let (a, b) = (op.targets[0], op.targets[1]);
            out.extend([Lg::Cx(a, b), Lg::Cx(b, a), Lg::Cx(a, b)]);
        }
        (Kind::Swap, 1) => {
            let (cq, a, b) = (op.controls[0], op.targets[0], op.targets[1]);
            out.push(Lg::Cx(b, a));
            ccx(out, cq, a, b);
            out.push(Lg::Cx(b, a));
        }
        (Kind::X, 2) => ccx(out, op.controls[0], op.controls[1], op.targets[0]),
        (k, 0) => out.push(Lg::One(op.targets[0], k.matrix())),
        (Kind::X, 1) => out.push(Lg::Cx(op.controls[0], op.targets[0])),
        (k, 1) => controlled_1q(out, op.controls[0], op.targets[0], &k.matrix()),
        _ => return Err(format!("poarta {} cu {} controale nu e suportată de transpilator", op.label(), nc)),
    }
    Ok(())
}

// ------------------------------------------------------------------ ECR

type M4 = [[C64; 4]; 4];

fn kron(hi: &Mat2, lo: &Mat2) -> M4 {
    let mut r = [[C64::ZERO; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            r[i][j] = hi[i >> 1][j >> 1] * lo[i & 1][j & 1];
        }
    }
    r
}
fn mul4(a: &M4, b: &M4) -> M4 {
    let mut r = [[C64::ZERO; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    r
}
fn eq_phase(a: &M4, b: &M4) -> bool {
    // a ≃ e^{iφ} b ?
    let mut ph = None;
    for i in 0..4 {
        for j in 0..4 {
            if b[i][j].norm_sqr() > 1e-6 {
                let r = a[i][j] * b[i][j].conj();
                let n = r.norm_sqr().sqrt();
                ph = Some(C64::new(r.re / n, r.im / n));
                break;
            }
        }
        if ph.is_some() {
            break;
        }
    }
    let Some(ph) = ph else { return false };
    (0..4).all(|i| (0..4).all(|j| (a[i][j] - ph * b[i][j]).norm_sqr() < 1e-16))
}

fn cliffords() -> Vec<Mat2> {
    let h = h_m();
    let s = Kind::S.matrix();
    let mut v: Vec<Mat2> = vec![ID];
    let mut i = 0;
    let same = |a: &Mat2, b: &Mat2| {
        let m = [[a[0][0], a[0][1], C64::ZERO, C64::ZERO], [a[1][0], a[1][1], C64::ZERO, C64::ZERO], [C64::ZERO; 4], [C64::ZERO; 4]];
        let n = [[b[0][0], b[0][1], C64::ZERO, C64::ZERO], [b[1][0], b[1][1], C64::ZERO, C64::ZERO], [C64::ZERO; 4], [C64::ZERO; 4]];
        eq_phase(&m, &n)
    };
    while i < v.len() {
        for g in [&h, &s] {
            let n = mul(g, &v[i]);
            if !v.iter().any(|x| same(x, &n)) {
                v.push(n);
            }
        }
        i += 1;
    }
    v
}

fn ecr_matrix() -> M4 {
    let k = 1.0 / 2f64.sqrt();
    let (o, a, b) = (C64::ZERO, c(k, 0.0), c(0.0, k));
    let nb = c(0.0, -k);
    // bază little-endian: index = q1*2 + q0 ; ECR(q0,q1) ca în Qiskit
    [[o, a, o, b], [a, o, nb, o], [o, b, o, a], [nb, o, a, o]]
}
fn cx_matrix() -> M4 {
    let (o, l) = (C64::ZERO, C64::ONE);
    [[l, o, o, o], [o, o, o, l], [o, o, l, o], [o, l, o, o]]
}

/// Găsește Clifford-uri A0,A1 (înainte) și B0,B1 (după) cu B·ECR·A ≃ CX(q0→q1).
pub fn ecr_cx_identity() -> Option<([Mat2; 2], [Mat2; 2])> {
    let cl = cliffords();
    let ecr = ecr_matrix();
    let cx = cx_matrix();
    for a0 in &cl {
        for a1 in &cl {
            let left = mul4(&ecr, &kron(a1, a0));
            for b0 in &cl {
                for b1 in &cl {
                    if eq_phase(&mul4(&kron(b1, b0), &left), &cx) {
                        return Some(([*a0, *a1], [*b0, *b1]));
                    }
                }
            }
        }
    }
    None
}

// ------------------------------------------------------------------ rutare + coborâre

pub struct Target<'a> {
    pub n_qubits: usize,
    pub coupling: &'a [(usize, usize)],
    pub basis: Basis,
}

fn adjacent(t: &Target, a: usize, b: usize) -> bool {
    t.coupling.iter().any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
}
fn directed(t: &Target, a: usize, b: usize) -> bool {
    t.coupling.iter().any(|&(x, y)| x == a && y == b)
}

fn shortest_path(t: &Target, from: usize, to: usize) -> Option<Vec<usize>> {
    let mut prev = vec![usize::MAX; t.n_qubits];
    let mut q = VecDeque::from([from]);
    prev[from] = from;
    while let Some(u) = q.pop_front() {
        if u == to {
            let mut p = vec![to];
            let mut x = to;
            while x != from {
                x = prev[x];
                p.push(x);
            }
            p.reverse();
            return Some(p);
        }
        for &(x, y) in t.coupling {
            for (a, b) in [(x, y), (y, x)] {
                if a == u && prev[b] == usize::MAX {
                    prev[b] = u;
                    q.push_back(b);
                }
            }
        }
    }
    None
}

fn lower_cx(out: &mut Vec<Native>, t: &Target, a: usize, b: usize, ecr: &Option<([Mat2; 2], [Mat2; 2])>) {
    let h = h_m();
    match t.basis {
        Basis::Cx => {
            if directed(t, a, b) || !directed(t, b, a) {
                out.push(Native::Cx(a, b));
            } else {
                for q in [a, b] {
                    emit_1q(out, q, &h);
                }
                out.push(Native::Cx(b, a));
                for q in [a, b] {
                    emit_1q(out, q, &h);
                }
            }
        }
        Basis::Cz => {
            emit_1q(out, b, &h);
            out.push(Native::Cz(a, b));
            emit_1q(out, b, &h);
        }
        Basis::Ecr => {
            if directed(t, a, b) || !directed(t, b, a) {
                let (pre, post) = ecr.as_ref().expect("identitate ECR");
                emit_1q(out, a, &pre[0]);
                emit_1q(out, b, &pre[1]);
                out.push(Native::Ecr(a, b));
                emit_1q(out, a, &post[0]);
                emit_1q(out, b, &post[1]);
            } else {
                for q in [a, b] {
                    emit_1q(out, q, &h);
                }
                lower_cx(out, t, b, a, ecr);
                for q in [a, b] {
                    emit_1q(out, q, &h);
                }
            }
        }
    }
}

pub fn transpile(prog: &Program, target: &Target) -> Result<Transpiled, String> {
    let n = prog.n_qubits;
    if n > target.n_qubits {
        return Err(format!("circuitul are {n} qubiți, backend-ul are {}", target.n_qubits));
    }
    // 1. descompunere în 1q + CX
    let mut lg = vec![];
    let mut measures = vec![];
    for ins in &prog.instrs {
        match ins {
            Instr::Gate(op) => {
                if !measures.is_empty() {
                    return Err("porți după măsurare: nu sunt suportate pe calea hardware".into());
                }
                lower_op(&mut lg, op)?
            }
            Instr::Measure { q, c } => measures.push((*q, *c)),
            Instr::Barrier => {}
            _ => return Err("reset/if (circuite dinamice) nu sunt suportate pe calea hardware".into()),
        }
    }
    // 2. rutare + peephole 1q
    let mut map: Vec<usize> = (0..n).collect(); // logic → fizic
    let mut pending: Vec<Mat2> = vec![ID; n];
    let ecr = if target.basis == Basis::Ecr { Some(ecr_cx_identity().ok_or("nu găsesc identitatea ECR→CX")?) } else { None };
    let mut out = vec![];
    let mut swaps = 0;
    let flush = |out: &mut Vec<Native>, map: &[usize], pending: &mut Vec<Mat2>, l: usize| {
        let m = std::mem::replace(&mut pending[l], ID);
        emit_1q(out, map[l], &m);
    };
    for g in &lg {
        match g {
            Lg::One(q, m) => pending[*q] = mul(m, &pending[*q]),
            Lg::Cx(a, b) => {
                if !adjacent(target, map[*a], map[*b]) {
                    let path = shortest_path(target, map[*a], map[*b]).ok_or("qubiți în componente neconectate")?;
                    for w in 0..path.len() - 2 {
                        let (p, q) = (path[w], path[w + 1]);
                        for (x, y) in [(p, q), (q, p), (p, q)] {
                            lower_cx(&mut out, target, x, y, &ecr);
                        }
                        swaps += 1;
                        for m in map.iter_mut() {
                            if *m == p {
                                *m = q;
                            } else if *m == q {
                                *m = p;
                            }
                        }
                    }
                }
                flush(&mut out, &map, &mut pending, *a);
                flush(&mut out, &map, &mut pending, *b);
                lower_cx(&mut out, target, map[*a], map[*b], &ecr);
            }
        }
    }
    for l in 0..n {
        flush(&mut out, &map, &mut pending, l);
    }
    let n_clbits = measures.iter().map(|&(_, c)| c + 1).max().unwrap_or(0);
    Ok(Transpiled { gates: out, measures: measures.iter().map(|&(q, c)| (map[q], c)).collect(), n_clbits, final_map: map, swaps })
}

/// Pentru verificare: transformă porțile native (fără ECR) în Program executabil pe simulator.
pub fn native_to_program(t: &Transpiled, n_phys: usize) -> Result<Program, String> {
    let mut p = Program::new(n_phys);
    for g in &t.gates {
        match *g {
            Native::Rz(q, a) => {
                p.rz(q, a);
            }
            Native::Sx(q) => {
                p.push(Op::single(Kind::Sx, q));
            }
            Native::X(q) => {
                p.x(q);
            }
            Native::Cx(a, b) => {
                p.cx(a, b);
            }
            Native::Cz(a, b) => {
                p.cz(a, b);
            }
            Native::Ecr(..) => return Err("ECR nu poate fi simulat direct".into()),
        }
    }
    let _ = FRAC_PI_2;
    Ok(p)
}
