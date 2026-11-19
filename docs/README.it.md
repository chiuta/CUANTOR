# CUANTOR

> Un simulatore di computer quantistico per il desktop: da 5 a 1000 qubit, con un'interfaccia in stile IBM Quantum Composer.

**È un computer quantistico per finta dentro il tuo computer: matematica esatta, ma non vero hardware quantistico.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — in una frase:** CUANTOR è un computer quantistico giocattolo sullo schermo. Clicchi blocchi colorati, premi Esegui e vedi la risposta. È per finta, non quello vero.

## Cosa fa

- Una lavagna come il Composer di IBM: scegli blocchi colorati (porte), li metti su linee (qubit) e barre e una palla mostrano cosa succede. Un pannello di testo scrive in tempo reale lo stesso circuito in OpenQASM e Qiskit.
- Tre calcolatori dentro: uno che ricorda ogni numero (fino a 28 qubit), uno velocissimo per porte semplici (fino a 4096 qubit) e uno per circuiti con pochi rami.
- Può decidere durante l’esecuzione (misura, reset, if). Puoi inventare porte tue, aggiungere errori finti (rumore) e ripetere qualsiasi risultato con un seed.
- Legge file OpenQASM 2.0 e 3.0 e salva OpenQASM 3.0 o Qiskit.
- Parla con IBM Quantum: apri il circuito in IBM Composer oppure invialo a una vera macchina IBM con la chiave del tuo account.
- Parla 8 lingue: rumeno, inglese, francese, tedesco, spagnolo, italiano, portoghese, ungherese.
- Funziona su Windows, macOS e Linux. Dentro c’è un solo progetto Rust con quattro parti: nucleo, finestra, riga di comando e collegamento IBM.

> **ELI5 — in una frase:** Blocchi colorati, tre calcolatori, 8 lingue, va su ogni computer.

## Limiti fisici

Ogni qubit in più raddoppia il lavoro, come un quaderno in cui ogni nuovo qubit raddoppia le pagine. 24 qubit richiedono circa 256 MiB, 28 qubit circa 4 GiB. Oltre, funzionano solo i disegni semplici. Un gruppo di 1000 qubit tutti d’accordo (GHZ) gira all’istante; un circuito qualsiasi da 1000 qubit è impossibile per qualunque computer normale.

> **ELI5 — in una frase:** Più qubit, molto più lavoro. I disegni semplici possono essere enormi, quelli ingarbugliati no.

## Avvio rapido

Il modo più facile: scarica il programma (vedi Download) e fai doppio clic su cuantor-gui. Per compilarlo da solo con un Rust recente:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — in una frase:** Scarichi, doppio clic su cuantor-gui e giochi.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: in CUANTOR premi «Apri in IBM Composer». Viene salvato un file .qasm. Nel Composer di IBM scegli File → Upload .qasm file.
- Una macchina vera: metti la tua chiave IBM, l’istanza (CRN) e il nome della macchina in IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o in ~/.config/cuantor/ibm.json), poi esegui `cuantor ibm run circuit.qasm --backend NOME`. La chiave esce dal tuo computer solo verso IBM.

> Avviso sincero: questa parte non è ancora stata provata su una vera macchina IBM, solo su un server finto e con il simulatore integrato. Potrebbero servire piccole correzioni. I circuiti con reset o if non si possono inviare.

> **ELI5 — in una frase:** Puoi inviare il circuito al sito di IBM o a una vera macchina IBM. Quella vera non l’abbiamo ancora provata.

## Qualità e audit

Prima della versione 0.2.1 abbiamo cercato in ogni modo di rompere il programma: 59 test automatici, dati senza senso, circuiti fatti per mangiare tutta la memoria, angoli impossibili, i tre calcolatori confrontati tra loro e risposte IBM false che mentono. Abbiamo trovato e corretto 7 errori (vedi il registro delle modifiche).

> **ELI5 — in una frase:** Abbiamo cercato di romperlo, trovato 7 errori e riparati.

[Registro delle modifiche](CHANGELOG.it.md)

## Download

I programmi per Windows, macOS (chip Apple e Intel) e Linux sono allegati a ogni versione. Scegli il tuo qui sotto.

**[Ultima versione](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Guida all’uso (PDF)

Spiegata come a un bambino di 10 anni, con un mini riassunto per uno di 5 anni dopo ogni capitolo. La stessa guida in 8 lingue.

**[Scarica la guida (PDF)](guide/CUANTOR-guide.it.pdf)** · Tutte le lingue: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — in una frase:** Scegli il tuo computer, scarichi, fatto. La guida è un PDF.

## Licenza

Non è ancora stata impostata una licenza (scelta dell'autore).

---
Realizzato con Claude. Solo simulazione: non si rivendica alcun vantaggio quantistico.
