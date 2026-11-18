# CUANTOR

> Un simulator de computer cuantic pentru desktop: de la 5 la 1000 de qubiți, cu interfață în stilul IBM Quantum Composer.

**Este o simulare exactă care rulează pe propriul tău calculator, nu hardware cuantic.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Ce face

- Editor în stil Composer: paletă de porți colorate, fire q[0..n], cadrane de măsurare pe fiecare qubit, histogramă de probabilități, Q-sferă cu inel de fază, panou de cod OpenQASM 3.0 / 2.0 / Qiskit în timp real.
- Trei motoare exacte: vector de stare (până la 28 de qubiți), stabilizator pentru circuite Clifford (până la 4096 de qubiți), amplitudini rare pentru circuite cu puține ramuri.
- Circuite dinamice (măsurare la mijloc, reset, if), porți definite de utilizator, zgomot de depolarizare și de citire, semințe reproductibile.
- Import OpenQASM 2.0 și 3.0, export OpenQASM 3.0 și Qiskit.
- Legătura cu IBM Quantum: deschide circuitele în IBM Composer sau rulează-le pe hardware IBM real prin API-ul REST Runtime, cu propria ta cheie.
- Interfață în 8 limbi: română, engleză, franceză, germană, spaniolă, italiană, portugheză, maghiară.
- Windows, macOS și Linux; un singur workspace Rust: nucleu, GUI, CLI, client IBM.

## Limite fizice

O stare generală de n qubiți cere 2^n amplitudini: 24 de qubiți ocupă circa 256 MiB, 28 de qubiți circa 4 GiB. Peste această limită merg doar circuitele cu structură (Clifford, stări de tip GHZ, puține ramuri de superpoziție). Un GHZ de 1000 de qubiți rulează instant; un circuit arbitrar de 1000 de qubiți este matematic inaccesibil oricărei mașini clasice.

## Pornire rapidă

Compilează din sursă cu un toolchain Rust actual:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: meniul Fișier → «Deschide în IBM Composer» salvează un fișier .qasm; în Composer folosește File → Upload .qasm file.
- Hardware real: setează local IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN și IBM_QUANTUM_BACKEND (sau ~/.config/cuantor/ibm.json), apoi rulează `cuantor ibm run circuit.qasm --backend NUME`. Cheia nu este afișată, logată sau trimisă nicăieri în afară de IBM.

> Neverificat pe hardware real: transpilatorul (porți native rz/sx/x + cx/cz/ecr, rutare SWAP) este verificat cu simulatorul propriu, iar clientul REST doar cu un server simulat. Detaliile cererilor pot cere ajustări; circuitele dinamice nu sunt suportate pe această cale.

## Calitate și audit

Versiunea 0.2.0 a trecut printr-un audit adversarial: 59 de teste automate, inclusiv fuzzing al parserului, bombe de resurse (memorie, recursie, imbricare), unghiuri NaN, verificări încrucișate între cele trei motoare, teste chi-pătrat ale eșantionării, reproductibilitate pe sămânță și răspunsuri REST ostile. Auditul a găsit și a reparat 7 defecte (vezi jurnalul de modificări).

[Jurnal de modificări](CHANGELOG.ro.md)

## Descărcare

Fișiere executabile pentru Windows, macOS (Apple silicon și Intel) și Linux sunt atașate fiecărei versiuni.

## Licență

Nu a fost setată încă o licență (alegerea autorului).

---
Construit cu Claude. Doar simulare: nu se pretinde avantaj cuantic.
