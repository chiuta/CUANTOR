# CUANTOR

> Un simulatore di computer quantistico per il desktop: da 5 a 1000 qubit, con un'interfaccia in stile IBM Quantum Composer.

**È una simulazione esatta eseguita sul tuo computer, non hardware quantistico.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Cosa fa

- Editor in stile Composer: tavolozza di porte colorate, fili q[0..n], quadranti di misura per ogni qubit, istogramma delle probabilità, sfera Q con anello di fase e codice OpenQASM 3.0 / 2.0 / Qiskit in tempo reale.
- Tre motori esatti: vettore di stato (fino a 28 qubit), stabilizzatore per circuiti Clifford (fino a 4096 qubit) e ampiezze sparse per circuiti con pochi rami.
- Circuiti dinamici (misura a metà circuito, reset, if), porte definite dall'utente, rumore di depolarizzazione e di lettura, semi riproducibili.
- Importazione di OpenQASM 2.0 e 3.0, esportazione in OpenQASM 3.0 e Qiskit.
- Collegamento a IBM Quantum: apri i circuiti in IBM Composer oppure eseguili su hardware IBM reale tramite l'API REST Runtime, con la tua chiave.
- Interfaccia in 8 lingue: rumeno, inglese, francese, tedesco, spagnolo, italiano, portoghese, ungherese.
- Windows, macOS e Linux; un unico workspace Rust: nucleo, GUI, CLI, client IBM.

## Limiti fisici

Uno stato generale di n qubit richiede 2^n ampiezze: 24 qubit occupano circa 256 MiB, 28 qubit circa 4 GiB. Oltre funzionano solo circuiti strutturati (Clifford, stati di tipo GHZ, pochi rami di sovrapposizione). Un GHZ da 1000 qubit gira all'istante; un circuito arbitrario da 1000 qubit è matematicamente fuori portata per qualsiasi macchina classica.

## Avvio rapido

Compila dai sorgenti con una toolchain Rust aggiornata:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: il menu File → «Apri in IBM Composer» salva un file .qasm; in Composer usa File → Upload .qasm file.
- Hardware reale: imposta in locale IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o ~/.config/cuantor/ibm.json), poi esegui `cuantor ibm run circuit.qasm --backend NOME`. La chiave non viene mai mostrata, registrata o inviata ad altri che a IBM.

> Non verificato su hardware reale: il transpilatore (porte native rz/sx/x + cx/cz/ecr, instradamento SWAP) è verificato con il simulatore integrato, il client REST solo con un server simulato. I dettagli delle richieste possono richiedere aggiustamenti; i circuiti dinamici non sono supportati su questo percorso.

## Qualità e audit

La versione 0.2.1 ha superato un audit avversariale: 59 test automatici, tra cui fuzzing del parser, bombe di risorse (memoria, ricorsione, annidamento), angoli NaN, verifiche incrociate tra i tre motori, test chi quadrato del campionamento, riproducibilità tramite seme e risposte REST ostili. L'audit ha trovato e corretto 7 difetti (vedi il registro delle modifiche).

[Registro delle modifiche](CHANGELOG.it.md)

## Download

Ogni versione include i binari per Windows, macOS (Apple silicon e Intel) e Linux.

## Licenza

Non è ancora stata impostata una licenza (scelta dell'autore).

---
Realizzato con Claude. Solo simulazione: non si rivendica alcun vantaggio quantistico.
