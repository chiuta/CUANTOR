# CUANTOR

> A quantum computer simulator for your desktop: 5 to 1000 qubits, with an IBM Quantum Composer-style interface.

**It is a pretend quantum computer inside your computer: exact maths, but not real quantum hardware.**

[Română](docs/README.ro.md) · [English](README.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Español](docs/README.es.md) · [Italiano](docs/README.it.md) · [Português](docs/README.pt.md) · [Magyar](docs/README.hu.md)

> **ELI5 — in one breath:** CUANTOR is a toy quantum computer on your screen. You click colored blocks, press Run, and see the answer. It is pretend, not the real thing.

## What it does

- A drawing board like IBM’s Composer: pick colored blocks (gates), put them on lines (qubits), and watch bars and a ball show what happens. A text panel writes the same circuit in OpenQASM and Qiskit, live.
- Three calculators inside: one that remembers every number (up to 28 qubits), one super-fast for simple gates (up to 4096 qubits), and one for circuits with only a few branches.
- It can decide in the middle of a run (measure, reset, if). You can invent your own gates, add pretend mistakes (noise), and repeat any result with a seed.
- It reads OpenQASM 2.0 and 3.0 files and saves OpenQASM 3.0 or Qiskit.
- It talks to IBM Quantum: open your circuit in IBM Composer, or send it to a real IBM machine with your own account key.
- It speaks 8 languages: Romanian, English, French, German, Spanish, Italian, Portuguese, Hungarian.
- It runs on Windows, macOS and Linux. Inside is one Rust project with four parts: core, window, command line and IBM link.

> **ELI5 — in one breath:** Colored blocks, three calculators, 8 languages, works on every computer.

## Physical limits

Every extra qubit doubles the work, like a notebook where each new qubit doubles the pages. 24 qubits need about 256 MiB, 28 qubits about 4 GiB. After that, only simple patterns work. A group of 1000 qubits that all agree (GHZ) runs instantly; a random 1000-qubit circuit is impossible for any normal computer.

> **ELI5 — in one breath:** More qubits means much more work. Simple patterns can be huge; messy ones cannot.

## Quick start

Easiest: download the program (see Download) and double-click cuantor-gui. To build it yourself with a current Rust toolchain:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — in one breath:** Download, double-click cuantor-gui, play.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: in CUANTOR press “Open in IBM Composer”. It saves a .qasm file. In IBM’s Composer choose File → Upload .qasm file.
- A real machine: put your IBM key, instance (CRN) and machine name in IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN and IBM_QUANTUM_BACKEND (or in ~/.config/cuantor/ibm.json), then run `cuantor ibm run circuit.qasm --backend NAME`. The key never leaves your computer except to go to IBM.

> Honest warning: this part has not been tried on a real IBM machine yet, only on a pretend server and against the built-in simulator. Small fixes may be needed. Circuits with reset or if cannot be sent.

> **ELI5 — in one breath:** You can send your circuit to IBM’s website or to a real IBM machine. We have not tried the real machine yet.

## Quality and audit

Before version 0.2.1 we tried hard to break the program: 59 automatic tests, random garbage input, circuits made to eat all the memory, impossible angles, the three calculators checked against each other, and fake IBM replies that lie. We found and fixed 7 bugs (see the changelog).

> **ELI5 — in one breath:** We tried hard to break it, found 7 bugs and fixed them.

[Changelog](CHANGELOG.md)

## Download

Programs for Windows, macOS (Apple chip and Intel) and Linux are attached to every release. Pick yours below.

**[Latest release](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### User guide (PDF)

Explained like you are 10, with a tiny summary for a 5-year-old after every chapter. The same guide in 8 languages.

**[Download the guide (PDF)](docs/guide/CUANTOR-guide.en.pdf)** · All languages: [Română](docs/guide/CUANTOR-guide.ro.pdf) · [English](docs/guide/CUANTOR-guide.en.pdf) · [Français](docs/guide/CUANTOR-guide.fr.pdf) · [Deutsch](docs/guide/CUANTOR-guide.de.pdf) · [Español](docs/guide/CUANTOR-guide.es.pdf) · [Italiano](docs/guide/CUANTOR-guide.it.pdf) · [Português](docs/guide/CUANTOR-guide.pt.pdf) · [Magyar](docs/guide/CUANTOR-guide.hu.pdf)

> **ELI5 — in one breath:** Pick your computer, download, done. The guide is a PDF.

## License

No license has been set yet (the author's choice).

---
Built with Claude. Simulation only: no quantum advantage is claimed.
