//! CUANTOR core — mașină virtuală cuantică.
//!
//! Simulează exact (vector de stare, precizie dublă) circuite cuantice:
//! porți unitare, măsurare la mijlocul circuitului, reset, instrucțiuni `if`
//! clasice, zgomot (depolarizare + eroare de citire) și OpenQASM 2.0.
//! Convenție: qubitul 0 este bitul cel mai puțin semnificativ (ca în Qiskit).

pub mod analysis;
pub mod complex;
pub mod error;
pub mod examples;
pub mod machine;
pub mod expr;
pub mod program;
pub mod qasm;
pub mod rng;
pub mod sparse;
pub mod stabilizer;
pub mod state;
pub mod vm;

pub use complex::{C64, Mat2};
pub use error::{Error, Result};
pub use program::{Creg, Instr, Kind, Op, Program};
pub use rng::Rng;
pub use state::{MAX_QUBITS, State, bloch_entropy, set_threads, threads};
pub use analysis::{Analysis, TopState, analyze};
pub use machine::{Backend, Machine, choose_backend, make_machine};
pub use sparse::Sparse;
pub use stabilizer::Stabilizer;
pub use vm::{Bits, Counts, NoiseModel, Vm, bench_h_all, final_state, run_program, sample, sample_with};

/// Limita totală de qubiți acceptată de parser/program (motoarele Clifford/sparse).
pub const MAX_QUBITS_TOTAL: usize = 4096;
/// Limita biților clasici.
pub const MAX_CLBITS: usize = 4096;
