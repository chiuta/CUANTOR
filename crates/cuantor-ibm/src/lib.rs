//! Conectare la IBM Quantum Platform (Qiskit Runtime REST) și transpilare în porți native (ISA).
//!
//! ATENȚIE: neverificat pe hardware real (nu au existat credențiale la scriere). Transpilatorul este
//! verificat cu simulatorul propriu; clientul REST este verificat doar împotriva unui server simulat.
pub mod client;
pub mod transpile;

pub use client::{BackendConfig, Client, Config};
pub use transpile::{Basis, Native, Transpiled, transpile};
