use crate::complex::{C64, Mat2};
use crate::{Result, err};
use std::f64::consts::{FRAC_1_SQRT_2, PI};
use std::fmt::Write;

/// Tipul de poartă de bază (un qubit, plus SWAP). Controalele se adaugă în [`Op`].
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    H,
    X,
    Y,
    Z,
    S,
    Sdg,
    T,
    Tdg,
    Sx,
    Rx(f64),
    Ry(f64),
    Rz(f64),
    P(f64),
    U(f64, f64, f64),
    Swap,
}

impl Kind {
    /// Matricea unitară 2×2 (pentru SWAP întoarce identitatea; nu se folosește).
    pub fn matrix(&self) -> Mat2 {
        let z = C64::ZERO;
        let o = C64::ONE;
        let i = C64::I;
        let h = FRAC_1_SQRT_2;
        match *self {
            Kind::H => [[C64::new(h, 0.0), C64::new(h, 0.0)], [C64::new(h, 0.0), C64::new(-h, 0.0)]],
            Kind::X => [[z, o], [o, z]],
            Kind::Y => [[z, -i], [i, z]],
            Kind::Z => [[o, z], [z, -o]],
            Kind::S => [[o, z], [z, i]],
            Kind::Sdg => [[o, z], [z, -i]],
            Kind::T => [[o, z], [z, C64::from_polar(1.0, PI / 4.0)]],
            Kind::Tdg => [[o, z], [z, C64::from_polar(1.0, -PI / 4.0)]],
            Kind::Sx => {
                let a = C64::new(0.5, 0.5);
                let b = C64::new(0.5, -0.5);
                [[a, b], [b, a]]
            }
            Kind::Rx(t) => {
                let (c, s) = ((t / 2.0).cos(), (t / 2.0).sin());
                [[C64::new(c, 0.0), C64::new(0.0, -s)], [C64::new(0.0, -s), C64::new(c, 0.0)]]
            }
            Kind::Ry(t) => {
                let (c, s) = ((t / 2.0).cos(), (t / 2.0).sin());
                [[C64::new(c, 0.0), C64::new(-s, 0.0)], [C64::new(s, 0.0), C64::new(c, 0.0)]]
            }
            Kind::Rz(t) => [[C64::from_polar(1.0, -t / 2.0), z], [z, C64::from_polar(1.0, t / 2.0)]],
            Kind::P(t) => [[o, z], [z, C64::from_polar(1.0, t)]],
            Kind::U(t, p, l) => {
                let (c, s) = ((t / 2.0).cos(), (t / 2.0).sin());
                [
                    [C64::new(c, 0.0), -C64::from_polar(s, l)],
                    [C64::from_polar(s, p), C64::from_polar(c, p + l)],
                ]
            }
            Kind::Swap => [[o, z], [z, o]],
        }
    }
    pub fn angle(&self) -> Option<f64> {
        match *self {
            Kind::Rx(a) | Kind::Ry(a) | Kind::Rz(a) | Kind::P(a) => Some(a),
            _ => None,
        }
    }
    pub fn with_angle(&self, a: f64) -> Kind {
        match self {
            Kind::Rx(_) => Kind::Rx(a),
            Kind::Ry(_) => Kind::Ry(a),
            Kind::Rz(_) => Kind::Rz(a),
            Kind::P(_) => Kind::P(a),
            k => k.clone(),
        }
    }
}

/// O poartă aplicată: tip de bază + qubiți de control + qubiți țintă.
#[derive(Clone, Debug, PartialEq)]
pub struct Op {
    pub kind: Kind,
    pub controls: Vec<usize>,
    pub targets: Vec<usize>,
}

impl Op {
    pub fn new(kind: Kind, controls: Vec<usize>, targets: Vec<usize>) -> Op {
        Op { kind, controls, targets }
    }
    pub fn single(kind: Kind, q: usize) -> Op {
        Op { kind, controls: vec![], targets: vec![q] }
    }
    /// Toți qubiții implicați: controalele, apoi țintele.
    pub fn qubits(&self) -> Vec<usize> {
        self.controls.iter().chain(self.targets.iter()).copied().collect()
    }
    pub fn validate(&self, n: usize) -> Result<()> {
        let want = if self.kind == Kind::Swap { 2 } else { 1 };
        if self.targets.len() != want {
            return Err(err!("poarta cere {want} qubit(i) țintă"));
        }
        let qs = self.qubits();
        for (i, &q) in qs.iter().enumerate() {
            if q >= n {
                return Err(err!("qubit {q} în afara registrului ({n} qubiți)"));
            }
            if qs[..i].contains(&q) {
                return Err(err!("qubit {q} repetat în aceeași poartă"));
            }
        }
        Ok(())
    }
    /// Numele OpenQASM (qelib1) și parametrii, dacă poarta are o formă standard.
    pub fn qasm_name(&self) -> Option<(&'static str, Vec<f64>)> {
        let c = self.controls.len();
        Some(match (&self.kind, c) {
            (Kind::H, 0) => ("h", vec![]),
            (Kind::X, 0) => ("x", vec![]),
            (Kind::Y, 0) => ("y", vec![]),
            (Kind::Z, 0) => ("z", vec![]),
            (Kind::S, 0) => ("s", vec![]),
            (Kind::Sdg, 0) => ("sdg", vec![]),
            (Kind::T, 0) => ("t", vec![]),
            (Kind::Tdg, 0) => ("tdg", vec![]),
            (Kind::Sx, 0) => ("sx", vec![]),
            (Kind::Rx(a), 0) => ("rx", vec![*a]),
            (Kind::Ry(a), 0) => ("ry", vec![*a]),
            (Kind::Rz(a), 0) => ("rz", vec![*a]),
            (Kind::P(a), 0) => ("u1", vec![*a]),
            (Kind::U(a, b, d), 0) => ("u3", vec![*a, *b, *d]),
            (Kind::Swap, 0) => ("swap", vec![]),
            (Kind::X, 1) => ("cx", vec![]),
            (Kind::X, 2) => ("ccx", vec![]),
            (Kind::Y, 1) => ("cy", vec![]),
            (Kind::Z, 1) => ("cz", vec![]),
            (Kind::Z, 2) => ("ccz", vec![]),
            (Kind::H, 1) => ("ch", vec![]),
            (Kind::P(a), 1) => ("cu1", vec![*a]),
            (Kind::Rz(a), 1) => ("crz", vec![*a]),
            (Kind::Swap, 1) => ("cswap", vec![]),
            _ => return None,
        })
    }
    /// Etichetă scurtă pentru interfață (ex. «CNOT», «Ry»).
    pub fn label(&self) -> String {
        let base = match &self.kind {
            Kind::H => "H",
            Kind::X => "X",
            Kind::Y => "Y",
            Kind::Z => "Z",
            Kind::S => "S",
            Kind::Sdg => "S†",
            Kind::T => "T",
            Kind::Tdg => "T†",
            Kind::Sx => "√X",
            Kind::Rx(_) => "Rx",
            Kind::Ry(_) => "Ry",
            Kind::Rz(_) => "Rz",
            Kind::P(_) => "P",
            Kind::U(..) => "U",
            Kind::Swap => "SWAP",
        };
        match (&self.kind, self.controls.len()) {
            (Kind::X, 1) => "CNOT".into(),
            (Kind::X, 2) => "CCX".into(),
            (_, 0) => base.into(),
            (_, c) => format!("{}{}", "C".repeat(c), base),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creg {
    pub name: String,
    pub offset: usize,
    pub size: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    Gate(Op),
    Measure { q: usize, c: usize },
    Reset(usize),
    /// `if (creg == value) op;`
    If { creg: usize, value: u64, op: Op },
    Barrier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub n_qubits: usize,
    pub n_clbits: usize,
    pub cregs: Vec<Creg>,
    pub instrs: Vec<Instr>,
}

pub fn fmt_angle_q(a: f64) -> String {
    if a.abs() < 1e-12 {
        return "0".into();
    }
    let r = a / PI;
    for q in [1i64, 2, 3, 4, 6, 8, 12, 16, 5, 10] {
        let p = (r * q as f64).round();
        if p != 0.0 && (r * q as f64 - p).abs() < 1e-9 {
            let p = p as i64;
            let sg = if p < 0 { "-" } else { "" };
            let ap = p.abs();
            let num = if ap == 1 { "pi".to_string() } else { format!("{ap}*pi") };
            return if q == 1 { format!("{sg}{num}") } else { format!("{sg}{num}/{q}") };
        }
    }
    format!("{a:.9}")
}

/// Format scurt cu π pentru afișare (π/3, −π/2, 0,785).
pub fn fmt_angle(a: f64) -> String {
    if a.abs() < 1e-12 {
        return "0".into();
    }
    let r = a / PI;
    for q in [1i64, 2, 3, 4, 6, 8, 12, 16, 5, 10] {
        let p = (r * q as f64).round();
        if p != 0.0 && (r * q as f64 - p).abs() < 1e-9 {
            let p = p as i64;
            let sg = if p < 0 { "−" } else { "" };
            let ap = p.abs();
            let num = if ap == 1 { "π".to_string() } else { format!("{ap}π") };
            return if q == 1 { format!("{sg}{num}") } else { format!("{sg}{num}/{q}") };
        }
    }
    format!("{:.3}", a)
}

impl Program {
    pub fn new(n_qubits: usize) -> Program {
        Program { n_qubits, n_clbits: 0, cregs: vec![], instrs: vec![] }
    }
    pub fn add_creg(&mut self, name: &str, size: usize) -> Result<usize> {
        if self.n_clbits + size > crate::MAX_CLBITS {
            return Err(err!("maximum {} biți clasici", crate::MAX_CLBITS));
        }
        self.cregs.push(Creg { name: name.into(), offset: self.n_clbits, size });
        self.n_clbits += size;
        Ok(self.cregs.len() - 1)
    }
    pub fn push(&mut self, op: Op) -> &mut Self {
        self.instrs.push(Instr::Gate(op));
        self
    }
    pub fn gate(&mut self, kind: Kind, q: usize) -> &mut Self {
        self.push(Op::single(kind, q))
    }
    pub fn h(&mut self, q: usize) -> &mut Self {
        self.gate(Kind::H, q)
    }
    pub fn x(&mut self, q: usize) -> &mut Self {
        self.gate(Kind::X, q)
    }
    pub fn z(&mut self, q: usize) -> &mut Self {
        self.gate(Kind::Z, q)
    }
    pub fn rx(&mut self, q: usize, a: f64) -> &mut Self {
        self.gate(Kind::Rx(a), q)
    }
    pub fn ry(&mut self, q: usize, a: f64) -> &mut Self {
        self.gate(Kind::Ry(a), q)
    }
    pub fn rz(&mut self, q: usize, a: f64) -> &mut Self {
        self.gate(Kind::Rz(a), q)
    }
    pub fn p(&mut self, q: usize, a: f64) -> &mut Self {
        self.gate(Kind::P(a), q)
    }
    pub fn cx(&mut self, c: usize, t: usize) -> &mut Self {
        self.push(Op::new(Kind::X, vec![c], vec![t]))
    }
    pub fn cz(&mut self, c: usize, t: usize) -> &mut Self {
        self.push(Op::new(Kind::Z, vec![c], vec![t]))
    }
    pub fn cp(&mut self, c: usize, t: usize, a: f64) -> &mut Self {
        self.push(Op::new(Kind::P(a), vec![c], vec![t]))
    }
    pub fn ccx(&mut self, c1: usize, c2: usize, t: usize) -> &mut Self {
        self.push(Op::new(Kind::X, vec![c1, c2], vec![t]))
    }
    pub fn swap(&mut self, a: usize, b: usize) -> &mut Self {
        self.push(Op::new(Kind::Swap, vec![], vec![a, b]))
    }
    pub fn measure(&mut self, q: usize, c: usize) -> &mut Self {
        self.instrs.push(Instr::Measure { q, c });
        self
    }
    pub fn gates(&self) -> impl Iterator<Item = &Op> {
        self.instrs.iter().filter_map(|i| if let Instr::Gate(o) = i { Some(o) } else { None })
    }
    pub fn has_measure(&self) -> bool {
        self.instrs.iter().any(|i| matches!(i, Instr::Measure { .. }))
    }
    pub fn is_unitary(&self) -> bool {
        self.instrs.iter().all(|i| matches!(i, Instr::Gate(_) | Instr::Barrier))
    }
    /// Toate porțile sunt Clifford (simulabile exact cu motorul stabilizator)?
    pub fn is_clifford(&self) -> bool {
        self.instrs.iter().all(|i| match i {
            Instr::Gate(o) | Instr::If { op: o, .. } => crate::stabilizer::decompose(o).is_some(),
            _ => true,
        })
    }
    /// Validează indicii și numărul de qubiți/biți.
    pub fn check(&self) -> Result<()> {
        if self.n_qubits == 0 || self.n_qubits > crate::MAX_QUBITS_TOTAL {
            return Err(err!("număr de qubiți invalid: {} (1..={})", self.n_qubits, crate::MAX_QUBITS_TOTAL));
        }
        if self.n_clbits > crate::MAX_CLBITS {
            return Err(err!("maximum {} biți clasici", crate::MAX_CLBITS));
        }
        for ins in &self.instrs {
            match ins {
                Instr::Gate(o) => o.validate(self.n_qubits)?,
                Instr::If { creg, op, .. } => {
                    if *creg >= self.cregs.len() {
                        return Err(err!("registru clasic inexistent"));
                    }
                    op.validate(self.n_qubits)?;
                }
                Instr::Measure { q, c } => {
                    if *q >= self.n_qubits || *c >= self.n_clbits {
                        return Err(err!("measure q[{q}] -> c[{c}] în afara limitelor"));
                    }
                }
                Instr::Reset(q) => {
                    if *q >= self.n_qubits {
                        return Err(err!("reset q[{q}] în afara limitelor"));
                    }
                }
                Instr::Barrier => {}
            }
        }
        Ok(())
    }
    /// Copie cu măsurare implicită: qubitul k → bitul clasic k (se completează registrul dacă e prea mic).
    pub fn with_measure_all(&self) -> Program {
        let mut p = self.clone();
        if p.n_clbits < self.n_qubits {
            let missing = self.n_qubits - p.n_clbits;
            let off = p.n_clbits;
            p.cregs.push(Creg { name: "meas".into(), offset: off, size: missing });
            p.n_clbits += missing;
        }
        for q in 0..self.n_qubits {
            p.instrs.push(Instr::Measure { q, c: q });
        }
        p
    }
    /// Export OpenQASM 2.0.
    pub fn to_qasm(&self) -> String {
        let mut s = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n");
        let _ = writeln!(s, "qreg q[{}];", self.n_qubits);
        for c in &self.cregs {
            let _ = writeln!(s, "creg {}[{}];", c.name, c.size);
        }
        let fmt_op = |o: &Op| -> String {
            match o.qasm_name() {
                Some((nm, ps)) => {
                    let par = if ps.is_empty() {
                        String::new()
                    } else {
                        format!("({})", ps.iter().map(|a| fmt_angle_q(*a)).collect::<Vec<_>>().join(","))
                    };
                    let qs = o.qubits().iter().map(|q| format!("q[{q}]")).collect::<Vec<_>>().join(",");
                    format!("{nm}{par} {qs};")
                }
                None => format!("// poartă fără nume standard: {}", o.label()),
            }
        };
        for ins in &self.instrs {
            match ins {
                Instr::Gate(o) => {
                    let _ = writeln!(s, "{}", fmt_op(o));
                }
                Instr::Measure { q, c } => {
                    let cr = self.cregs.iter().find(|r| *c >= r.offset && *c < r.offset + r.size);
                    if let Some(r) = cr {
                        let _ = writeln!(s, "measure q[{q}] -> {}[{}];", r.name, c - r.offset);
                    }
                }
                Instr::Reset(q) => {
                    let _ = writeln!(s, "reset q[{q}];");
                }
                Instr::If { creg, value, op } => {
                    let _ = writeln!(s, "if({}=={}) {}", self.cregs[*creg].name, value, fmt_op(op));
                }
                Instr::Barrier => {
                    let _ = writeln!(s, "barrier q;");
                }
            }
        }
        s
    }

    /// Export OpenQASM 3.0 (stdgates.inc), ca în IBM Quantum Composer.
    pub fn to_qasm3(&self) -> String {
        let mut s = String::from("OPENQASM 3.0;\ninclude \"stdgates.inc\";\n\n");
        let _ = writeln!(s, "qubit[{}] q;", self.n_qubits);
        for c in &self.cregs {
            let _ = writeln!(s, "bit[{}] {};", c.size, c.name);
        }
        s.push('\n');
        let fmt_op = |o: &Op| -> String {
            match o.qasm_name() {
                Some((nm, ps)) => {
                    let nm = match nm {
                        "u1" => "p",
                        "cu1" => "cp",
                        x => x,
                    };
                    let par = if ps.is_empty() { String::new() } else { format!("({})", ps.iter().map(|a| fmt_angle_q(*a)).collect::<Vec<_>>().join(", ")) };
                    let qs = o.qubits().iter().map(|q| format!("q[{q}]")).collect::<Vec<_>>().join(", ");
                    format!("{nm}{par} {qs};")
                }
                None => format!("// poartă fără nume standard: {}", o.label()),
            }
        };
        for ins in &self.instrs {
            match ins {
                Instr::Gate(o) => {
                    let _ = writeln!(s, "{}", fmt_op(o));
                }
                Instr::Measure { q, c } => {
                    if let Some(r) = self.cregs.iter().find(|r| *c >= r.offset && *c < r.offset + r.size) {
                        let _ = writeln!(s, "{}[{}] = measure q[{q}];", r.name, c - r.offset);
                    }
                }
                Instr::Reset(q) => {
                    let _ = writeln!(s, "reset q[{q}];");
                }
                Instr::If { creg, value, op } => {
                    let _ = writeln!(s, "if ({} == {}) {{ {} }}", self.cregs[*creg].name, value, fmt_op(op));
                }
                Instr::Barrier => {
                    let _ = writeln!(s, "barrier q;");
                }
            }
        }
        s
    }
    /// Export Qiskit (Python), doar pentru citire.
    pub fn to_qiskit(&self) -> String {
        let mut s = String::from("from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit\n\n");
        let _ = writeln!(s, "qreg_q = QuantumRegister({}, 'q')", self.n_qubits);
        for c in &self.cregs {
            let _ = writeln!(s, "creg_{0} = ClassicalRegister({1}, '{0}')", c.name, c.size);
        }
        let regs = self.cregs.iter().map(|c| format!("creg_{}", c.name)).collect::<Vec<_>>().join(", ");
        let _ = writeln!(s, "circuit = QuantumCircuit(qreg_q{}{})\n", if regs.is_empty() { "" } else { ", " }, regs);
        let fmt_op = |o: &Op| -> String {
            match o.qasm_name() {
                Some((nm, ps)) => {
                    let nm = match nm {
                        "u1" | "cu1" => if nm == "u1" { "p" } else { "cp" },
                        "u3" => "u",
                        x => x,
                    };
                    let mut args: Vec<String> = ps.iter().map(|a| format!("{a:.12}")).collect();
                    args.extend(o.qubits().iter().map(|q| format!("qreg_q[{q}]")));
                    format!("circuit.{nm}({})", args.join(", "))
                }
                None => format!("# poartă fără echivalent direct: {}", o.label()),
            }
        };
        for ins in &self.instrs {
            match ins {
                Instr::Gate(o) => {
                    let _ = writeln!(s, "{}", fmt_op(o));
                }
                Instr::Measure { q, c } => {
                    if let Some(r) = self.cregs.iter().find(|r| *c >= r.offset && *c < r.offset + r.size) {
                        let _ = writeln!(s, "circuit.measure(qreg_q[{q}], creg_{}[{}])", r.name, c - r.offset);
                    }
                }
                Instr::Reset(q) => {
                    let _ = writeln!(s, "circuit.reset(qreg_q[{q}])");
                }
                Instr::If { creg, value, op } => {
                    let _ = writeln!(s, "{}.c_if(creg_{}, {})", fmt_op(op), self.cregs[*creg].name, value);
                }
                Instr::Barrier => {
                    let _ = writeln!(s, "circuit.barrier()");
                }
            }
        }
        s
    }
}
