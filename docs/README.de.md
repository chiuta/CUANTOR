# CUANTOR

> Ein Quantencomputer-Simulator für den Desktop: 5 bis 1000 Qubits, mit einer Oberfläche im Stil des IBM Quantum Composer.

**Es ist ein Quantencomputer zum So-tun-als-ob in deinem Computer: exakte Mathematik, aber keine echte Quanten-Hardware.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — in einem Atemzug:** CUANTOR ist ein Spielzeug-Quantencomputer auf deinem Bildschirm. Du klickst bunte Bausteine, drückst Ausführen und siehst die Antwort. Es ist nur gespielt, nicht echt.

## Was es kann

- Ein Zeichenbrett wie IBMs Composer: Du wählst bunte Bausteine (Gatter), legst sie auf Linien (Qubits) und siehst an Balken und einer Kugel, was passiert. Ein Textfenster schreibt denselben Schaltkreis live in OpenQASM und Qiskit.
- Drei Rechner im Inneren: einer, der sich jede Zahl merkt (bis 28 Qubits), einer, der für einfache Gatter blitzschnell ist (bis 4096 Qubits), und einer für Schaltkreise mit wenigen Zweigen.
- Er kann mittendrin entscheiden (Messung, reset, if). Du kannst eigene Gatter erfinden, gespielte Fehler (Rauschen) hinzufügen und jedes Ergebnis mit einem Seed wiederholen.
- Er liest OpenQASM-2.0- und -3.0-Dateien und speichert OpenQASM 3.0 oder Qiskit.
- Er spricht mit IBM Quantum: Öffne deinen Schaltkreis im IBM Composer oder schick ihn mit dem Schlüssel deines Kontos an eine echte IBM-Maschine.
- Er spricht 8 Sprachen: Rumänisch, Englisch, Französisch, Deutsch, Spanisch, Italienisch, Portugiesisch, Ungarisch.
- Er läuft unter Windows, macOS und Linux. Innen steckt ein einziges Rust-Projekt mit vier Teilen: Kern, Fenster, Kommandozeile und IBM-Anbindung.

> **ELI5 — in einem Atemzug:** Bunte Bausteine, drei Rechner, 8 Sprachen, läuft auf jedem Computer.

## Physikalische Grenzen

Jedes zusätzliche Qubit verdoppelt die Arbeit, wie ein Heft, in dem jedes neue Qubit die Seiten verdoppelt. 24 Qubits brauchen etwa 256 MiB, 28 Qubits etwa 4 GiB. Danach gehen nur noch einfache Muster. Eine Gruppe aus 1000 Qubits, die sich alle einig sind (GHZ), läuft sofort; ein beliebiger 1000-Qubit-Schaltkreis ist für jeden normalen Computer unmöglich.

> **ELI5 — in einem Atemzug:** Mehr Qubits heißt viel mehr Arbeit. Einfache Muster können riesig sein, verworrene nicht.

## Schnellstart

Am einfachsten: Programm herunterladen (siehe Download) und cuantor-gui doppelklicken. Selbst bauen mit einem aktuellen Rust:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — in einem Atemzug:** Herunterladen, cuantor-gui doppelklicken, spielen.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: Drücke in CUANTOR „In IBM Composer öffnen“. Es wird eine .qasm-Datei gespeichert. Im IBM-Composer wähle File → Upload .qasm file.
- Eine echte Maschine: Trage deinen IBM-Schlüssel, die Instanz (CRN) und den Maschinennamen in IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN und IBM_QUANTUM_BACKEND ein (oder in ~/.config/cuantor/ibm.json) und starte `cuantor ibm run circuit.qasm --backend NAME`. Der Schlüssel verlässt deinen Computer nur in Richtung IBM.

> Ehrliche Warnung: Dieser Teil wurde noch nicht an einer echten IBM-Maschine ausprobiert, nur an einem Schein-Server und mit dem eingebauten Simulator. Kleine Korrekturen können nötig sein. Schaltkreise mit reset oder if lassen sich nicht senden.

> **ELI5 — in einem Atemzug:** Du kannst deinen Schaltkreis an IBMs Webseite oder an eine echte IBM-Maschine schicken. Die echte Maschine haben wir noch nicht probiert.

## Qualität und Audit

Vor Version 0.2.1 haben wir alles versucht, das Programm kaputt zu machen: 59 automatische Tests, zufälliger Datenmüll, Schaltkreise, die den ganzen Speicher fressen sollen, unmögliche Winkel, die drei Rechner gegeneinander geprüft und falsche IBM-Antworten, die lügen. Wir haben 7 Fehler gefunden und behoben (siehe Änderungsprotokoll).

> **ELI5 — in einem Atemzug:** Wir haben alles versucht, es kaputt zu machen, 7 Fehler gefunden und repariert.

[Änderungsprotokoll](CHANGELOG.de.md)

## Download

Programme für Windows, macOS (Apple-Chip und Intel) und Linux hängen an jedem Release. Such dir unten deins aus.

**[Neuestes Release](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Benutzerhandbuch (PDF)

Erklärt wie für ein 10-jähriges Kind, mit einer Mini-Zusammenfassung für ein 5-jähriges Kind nach jedem Kapitel. Dasselbe Handbuch in 8 Sprachen.

**[Handbuch herunterladen (PDF)](guide/CUANTOR-guide.de.pdf)** · Alle Sprachen: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — in einem Atemzug:** Computer wählen, herunterladen, fertig. Das Handbuch ist ein PDF.

## Lizenz

Es wurde noch keine Lizenz festgelegt (Entscheidung des Autors).

---
Mit Claude gebaut. Nur Simulation: Es wird kein Quantenvorteil behauptet.
