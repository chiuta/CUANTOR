# CUANTOR

> A quantum computer simulator for your desktop: 5 to 1000 qubits, with an IBM Quantum Composer-style interface.

**It is an exact simulation running on your own computer, not quantum hardware.**

[Română](docs/README.ro.md) · [English](README.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Español](docs/README.es.md) · [Italiano](docs/README.it.md) · [Português](docs/README.pt.md) · [Magyar](docs/README.hu.md)

## What it does

- Composer-style editor: colored gate palette, wires q[0..n], per-qubit measurement dials, probabilities histogram, Q-sphere with phase ring, live OpenQASM 3.0 / 2.0 / Qiskit code panel.
- Three exact engines: state vector (up to 28 qubits), stabilizer for Clifford circuits (up to 4096 qubits), sparse amplitudes for few-branch circuits.
- Dynamic circuits (mid-circuit measurement, reset, if), user-defined gates, depolarizing and readout noise, reproducible seeds.
- OpenQASM 2.0 and 3.0 import, OpenQASM 3.0 and Qiskit export.
- IBM Quantum link: open circuits in IBM Composer, or run them on real IBM hardware through the Runtime REST API with your own key.
- Interface in 8 languages: Romanian, English, French, German, Spanish, Italian, Portuguese, Hungarian.
- Windows, macOS and Linux; one Rust workspace: core, GUI, CLI, IBM client.

## Physical limits

A general state of n qubits needs 2^n amplitudes: 24 qubits take about 256 MiB, 28 qubits about 4 GiB. Beyond that only structured circuits work (Clifford circuits, GHZ-like states, few superposition branches). A 1000-qubit GHZ state runs instantly; an arbitrary 1000-qubit circuit is mathematically out of reach for any classical machine.

## Quick start

Build from source with a current Rust toolchain:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: File menu → “Open in IBM Composer” saves a .qasm file; in Composer use File → Upload .qasm file.
- Real hardware: set IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN and IBM_QUANTUM_BACKEND locally (or ~/.config/cuantor/ibm.json), then run `cuantor ibm run circuit.qasm --backend NAME`. The key is never printed, logged or sent anywhere except to IBM.

> Not verified on real hardware: the transpiler (native rz/sx/x + cx/cz/ecr gates, SWAP routing) is verified against the built-in simulator, and the REST client only against a mock server. Request details may need adjusting; dynamic circuits are not supported on this path.

## Quality and audit

Version 0.2.0 went through an adversarial audit: 59 automated tests, including fuzzing of the parser, resource bombs (memory, recursion, nesting), NaN angles, cross-checks between the three engines, chi-square sampling tests, seed reproducibility and hostile REST responses. The audit found and fixed 7 defects (see the changelog).

[Changelog](CHANGELOG.md)

## Download

Binaries for Windows, macOS (Apple silicon and Intel) and Linux are attached to each release.

## License

No license has been set yet (the author's choice).

---
Built with Claude. Simulation only: no quantum advantage is claimed.
