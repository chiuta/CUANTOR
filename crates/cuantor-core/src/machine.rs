use crate::program::{Kind, Op, Program};
use crate::state::State;
use crate::{Result, Rng, err};

/// Interfața comună a motoarelor de simulare (vector de stare, stabilizator, sparse).
pub trait Machine: Send {
    fn n_qubits(&self) -> usize;
    fn reset_all(&mut self);
    fn apply(&mut self, op: &Op) -> Result<()>;
    fn measure(&mut self, q: usize, rng: &mut Rng) -> Result<bool>;
    fn reset_qubit(&mut self, q: usize, rng: &mut Rng) -> Result<()> {
        if self.measure(q, rng)? {
            self.apply(&Op::single(Kind::X, q))?;
        }
        Ok(())
    }
}

impl Machine for State {
    fn n_qubits(&self) -> usize {
        State::n_qubits(self)
    }
    fn reset_all(&mut self) {
        self.reset()
    }
    fn apply(&mut self, op: &Op) -> Result<()> {
        State::apply(self, op)
    }
    fn measure(&mut self, q: usize, rng: &mut Rng) -> Result<bool> {
        Ok(State::measure(self, q, rng))
    }
}

/// Motorul de simulare folosit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Backend {
    /// Alege automat: vector de stare ≤ 24 qubiți, apoi stabilizator (Clifford) sau sparse.
    #[default]
    Auto,
    /// Exact, orice poartă, memorie 16·2ⁿ octeți (maxim 28 qubiți).
    StateVector,
    /// Exact, doar porți Clifford (H, S, CNOT, Pauli, CZ, SWAP…), mii de qubiți.
    Stabilizer,
    /// Exact, orice poartă, dar doar cât timp numărul de ramuri nenule rămâne mic.
    Sparse,
}

impl Backend {
    pub fn name(self) -> &'static str {
        match self {
            Backend::Auto => "Automat",
            Backend::StateVector => "Vector de stare",
            Backend::Stabilizer => "Stabilizator (Clifford)",
            Backend::Sparse => "Sparse (puține ramuri)",
        }
    }
    pub const ALL: [Backend; 4] = [Backend::Auto, Backend::StateVector, Backend::Stabilizer, Backend::Sparse];
}

/// Limita până la care `Auto` alege vectorul de stare (256 MiB).
pub const SV_AUTO_MAX: usize = 24;

pub fn choose_backend(prog: &Program, req: Backend) -> Result<Backend> {
    let n = prog.n_qubits;
    match req {
        Backend::Auto => Ok(if n <= SV_AUTO_MAX {
            Backend::StateVector
        } else if prog.is_clifford() {
            Backend::Stabilizer
        } else {
            Backend::Sparse
        }),
        Backend::StateVector => {
            if n > crate::MAX_QUBITS {
                Err(err!("vectorul de stare suportă cel mult {} qubiți (cerut: {n}); folosește Stabilizator sau Sparse", crate::MAX_QUBITS))
            } else {
                Ok(req)
            }
        }
        Backend::Stabilizer => {
            if prog.is_clifford() {
                Ok(req)
            } else {
                Err(err!("circuitul conține porți non-Clifford (T, rotații generale, Toffoli…): stabilizatorul nu le poate simula"))
            }
        }
        Backend::Sparse => Ok(req),
    }
}

pub fn make_machine(b: Backend, n: usize) -> Result<Box<dyn Machine>> {
    Ok(match b {
        Backend::StateVector | Backend::Auto => Box::new(State::new(n)?),
        Backend::Stabilizer => Box::new(crate::stabilizer::Stabilizer::new(n)?),
        Backend::Sparse => Box::new(crate::sparse::Sparse::new(n)?),
    })
}
