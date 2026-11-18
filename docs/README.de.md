# CUANTOR

> Ein Quantencomputer-Simulator für den Desktop: 5 bis 1000 Qubits, mit einer Oberfläche im Stil des IBM Quantum Composer.

**Es ist eine exakte Simulation auf dem eigenen Rechner, keine Quantenhardware.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Was es kann

- Editor im Composer-Stil: farbige Gatterpalette, Leitungen q[0..n], Messanzeigen je Qubit, Wahrscheinlichkeits-Histogramm, Q-Sphäre mit Phasenring, Live-Code in OpenQASM 3.0 / 2.0 / Qiskit.
- Drei exakte Engines: Zustandsvektor (bis 28 Qubits), Stabilisator für Clifford-Schaltungen (bis 4096 Qubits), dünn besetzte Amplituden für Schaltungen mit wenigen Zweigen.
- Dynamische Schaltungen (Messung mitten im Ablauf, Reset, if), benutzerdefinierte Gatter, Depolarisations- und Ausleserauschen, reproduzierbare Seeds.
- Import von OpenQASM 2.0 und 3.0, Export als OpenQASM 3.0 und Qiskit.
- Anbindung an IBM Quantum: Schaltungen in IBM Composer öffnen oder über die Runtime-REST-API mit dem eigenen Schlüssel auf echter IBM-Hardware ausführen.
- Oberfläche in 8 Sprachen: Rumänisch, Englisch, Französisch, Deutsch, Spanisch, Italienisch, Portugiesisch, Ungarisch.
- Windows, macOS und Linux; ein Rust-Workspace: Kern, GUI, CLI, IBM-Client.

## Physikalische Grenzen

Ein allgemeiner Zustand aus n Qubits braucht 2^n Amplituden: 24 Qubits belegen etwa 256 MiB, 28 Qubits etwa 4 GiB. Darüber hinaus funktionieren nur strukturierte Schaltungen (Clifford, GHZ-artige Zustände, wenige Überlagerungszweige). Ein GHZ-Zustand mit 1000 Qubits läuft sofort; eine beliebige Schaltung mit 1000 Qubits ist für jeden klassischen Rechner mathematisch unerreichbar.

## Schnellstart

Aus dem Quellcode mit einer aktuellen Rust-Toolchain bauen:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: Menü Datei → „In IBM Composer öffnen“ speichert eine .qasm-Datei; in Composer File → Upload .qasm file verwenden.
- Echte Hardware: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN und IBM_QUANTUM_BACKEND lokal setzen (oder ~/.config/cuantor/ibm.json) und `cuantor ibm run circuit.qasm --backend NAME` ausführen. Der Schlüssel wird nie ausgegeben, protokolliert oder an andere Stellen als IBM gesendet.

> Auf echter Hardware nicht verifiziert: Der Transpiler (native Gatter rz/sx/x + cx/cz/ecr, SWAP-Routing) ist mit dem eingebauten Simulator geprüft, der REST-Client nur gegen einen Mock-Server. Anfragedetails können Anpassungen erfordern; dynamische Schaltungen werden auf diesem Weg nicht unterstützt.

## Qualität und Audit

Version 0.2.0 durchlief ein adversariales Audit: 59 automatisierte Tests, darunter Fuzzing des Parsers, Ressourcenbomben (Speicher, Rekursion, Verschachtelung), NaN-Winkel, Kreuzprüfungen der drei Engines, Chi-Quadrat-Tests der Stichproben, Reproduzierbarkeit per Seed und feindliche REST-Antworten. Das Audit fand 7 Fehler und behob sie (siehe Änderungsprotokoll).

[Änderungsprotokoll](CHANGELOG.de.md)

## Download

Programme für Windows, macOS (Apple Silicon und Intel) und Linux hängen an jedem Release.

## Lizenz

Es wurde noch keine Lizenz festgelegt (Entscheidung des Autors).

---
Mit Claude gebaut. Nur Simulation: Es wird kein Quantenvorteil behauptet.
