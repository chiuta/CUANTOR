# CUANTOR

> Kvantumszámítógép-szimulátor az asztali gépedre: 5-től 1000 qubitig, IBM Quantum Composer stílusú felülettel.

**Pontos szimuláció, amely a saját számítógépeden fut; nem kvantumhardver.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Mit tud

- Composer stílusú szerkesztő: színes kapupaletta, q[0..n] vezetékek, qubitenkénti mérőkorongok, valószínűségi hisztogram, Q-gömb fázisgyűrűvel, élő OpenQASM 3.0 / 2.0 / Qiskit kód.
- Három pontos motor: állapotvektor (legfeljebb 28 qubit), stabilizátor Clifford-áramkörökhöz (legfeljebb 4096 qubit), ritka amplitúdók kevés ággal rendelkező áramkörökhöz.
- Dinamikus áramkörök (közbenső mérés, reset, if), felhasználói kapuk, depolarizációs és kiolvasási zaj, reprodukálható magértékek.
- OpenQASM 2.0 és 3.0 import, OpenQASM 3.0 és Qiskit export.
- IBM Quantum kapcsolat: az áramkörök megnyithatók az IBM Composerben, vagy futtathatók valódi IBM hardveren a Runtime REST API-n keresztül, a saját kulcsoddal.
- 8 nyelvű felület: román, angol, francia, német, spanyol, olasz, portugál, magyar.
- Windows, macOS és Linux; egyetlen Rust munkaterület: mag, grafikus felület, CLI, IBM kliens.

## Fizikai korlátok

Egy általános n qubites állapot 2^n amplitúdót igényel: 24 qubit körülbelül 256 MiB, 28 qubit körülbelül 4 GiB. Efelett csak szerkezettel rendelkező áramkörök működnek (Clifford, GHZ-szerű állapotok, kevés szuperpozíciós ág). Egy 1000 qubites GHZ-állapot azonnal lefut; egy tetszőleges 1000 qubites áramkör matematikailag elérhetetlen bármely klasszikus gép számára.

## Gyors kezdés

Fordítás forrásból, naprakész Rust eszközlánccal:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: a Fájl menü → „Megnyitás az IBM Composerben” .qasm fájlt ment; a Composerben használd a File → Upload .qasm file parancsot.
- Valódi hardver: állítsd be helyben az IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN és IBM_QUANTUM_BACKEND változót (vagy a ~/.config/cuantor/ibm.json fájlt), majd futtasd: `cuantor ibm run circuit.qasm --backend NÉV`. A kulcsot a program soha nem írja ki, nem naplózza, és csak az IBM-nek küldi el.

> Valódi hardveren nem ellenőrzött: a transzpiler (natív rz/sx/x + cx/cz/ecr kapuk, SWAP-útválasztás) a beépített szimulátorral van ellenőrizve, a REST kliens pedig csak egy mesterséges szerverrel. A kérések részletei módosításra szorulhatnak; dinamikus áramkörök ezen az úton nem támogatottak.

## Minőség és audit

A(z) 0.2.1 verzió ellenséges auditon esett át: 59 automatikus teszt, köztük az elemző fuzzolása, erőforrás-bombák (memória, rekurzió, egymásba ágyazás), NaN szögek, a három motor kereszt-ellenőrzése, a mintavétel khí-négyzet tesztjei, magérték-alapú reprodukálhatóság és ellenséges REST-válaszok. Az audit 7 hibát talált és javított ki (lásd a változásnaplót).

[Változásnapló](CHANGELOG.hu.md)

## Letöltés

Minden kiadáshoz tartoznak futtatható fájlok Windowsra, macOS-re (Apple silicon és Intel) és Linuxra.

## Licenc

Licenc még nincs megadva (a szerző döntése).

---
Claude segítségével készült. Csak szimuláció: kvantumelőnyt nem állít.
