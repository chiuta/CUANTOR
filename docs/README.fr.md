# CUANTOR

> Un simulateur d'ordinateur quantique pour votre bureau : de 5 à 1000 qubits, avec une interface de type IBM Quantum Composer.

**C’est un ordinateur quantique pour de faux, dans ton ordinateur : des maths exactes, mais pas du vrai matériel quantique.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — en une phrase:** CUANTOR est un ordinateur quantique jouet sur ton écran. Tu cliques sur des blocs colorés, tu appuies sur Exécuter et tu vois la réponse. C’est pour de faux, pas le vrai.

## Ce que fait le programme

- Une planche à dessin comme le Composer d’IBM : tu choisis des blocs colorés (portes), tu les poses sur des lignes (qubits) et des barres et une boule montrent ce qui se passe. Un panneau de texte écrit en direct le même circuit en OpenQASM et Qiskit.
- Trois calculateurs à l’intérieur : un qui retient tous les nombres (jusqu’à 28 qubits), un ultra rapide pour les portes simples (jusqu’à 4096 qubits) et un pour les circuits à peu de branches.
- Il peut décider en cours de route (mesure, reset, if). Tu peux inventer tes portes, ajouter de fausses erreurs (bruit) et répéter n’importe quel résultat avec une graine (seed).
- Il lit les fichiers OpenQASM 2.0 et 3.0 et enregistre en OpenQASM 3.0 ou Qiskit.
- Il parle à IBM Quantum : ouvre ton circuit dans IBM Composer ou envoie-le à une vraie machine IBM avec la clé de ton compte.
- Il parle 8 langues : roumain, anglais, français, allemand, espagnol, italien, portugais, hongrois.
- Il tourne sous Windows, macOS et Linux. À l’intérieur : un seul projet Rust en quatre parties : cœur, fenêtre, ligne de commande et lien IBM.

> **ELI5 — en une phrase:** Des blocs colorés, trois calculateurs, 8 langues, marche sur tous les ordinateurs.

## Limites physiques

Chaque qubit en plus double le travail, comme un cahier où chaque nouveau qubit double les pages. 24 qubits demandent environ 256 Mio, 28 qubits environ 4 Gio. Au-delà, seuls les motifs simples marchent. Un groupe de 1000 qubits tous d’accord (GHZ) tourne instantanément ; un circuit quelconque de 1000 qubits est impossible pour tout ordinateur normal.

> **ELI5 — en une phrase:** Plus de qubits, bien plus de travail. Les motifs simples peuvent être énormes, les compliqués non.

## Démarrage rapide

Le plus simple : télécharge le programme (voir Téléchargement) et double-clique sur cuantor-gui. Pour le compiler toi-même avec un Rust récent :

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — en une phrase:** Télécharge, double-clique sur cuantor-gui et joue.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer : dans CUANTOR, appuie sur « Ouvrir dans IBM Composer ». Un fichier .qasm est enregistré. Dans le Composer d’IBM, choisis File → Upload .qasm file.
- Une vraie machine : mets ta clé IBM, ton instance (CRN) et le nom de la machine dans IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN et IBM_QUANTUM_BACKEND (ou dans ~/.config/cuantor/ibm.json), puis lance `cuantor ibm run circuit.qasm --backend NOM`. La clé ne quitte ton ordinateur que pour aller chez IBM.

> Avertissement honnête : cette partie n’a pas encore été essayée sur une vraie machine IBM, seulement sur un faux serveur et avec le simulateur intégré. De petites corrections peuvent être nécessaires. Les circuits avec reset ou if ne peuvent pas être envoyés.

> **ELI5 — en une phrase:** Tu peux envoyer ton circuit au site d’IBM ou à une vraie machine IBM. La vraie machine, on ne l’a pas encore essayée.

## Qualité et audit

Avant la version 0.2.1, nous avons tout fait pour casser le programme : 59 tests automatiques, données absurdes, circuits conçus pour avaler toute la mémoire, angles impossibles, les trois calculateurs comparés entre eux et fausses réponses IBM qui mentent. Nous avons trouvé et corrigé 7 bugs (voir le journal des modifications).

> **ELI5 — en une phrase:** On a tout fait pour le casser, on a trouvé 7 bugs et on les a réparés.

[Journal des modifications](CHANGELOG.fr.md)

## Téléchargement

Des programmes pour Windows, macOS (puce Apple et Intel) et Linux sont joints à chaque version. Choisis le tien ci-dessous.

**[Dernière version](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Guide d’utilisation (PDF)

Expliqué comme à un enfant de 10 ans, avec un mini-résumé pour un enfant de 5 ans après chaque chapitre. Le même guide en 8 langues.

**[Télécharger le guide (PDF)](guide/CUANTOR-guide.fr.pdf)** · Toutes les langues : [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — en une phrase:** Choisis ton ordinateur, télécharge, c’est tout. Le guide est un PDF.

## Licence

Aucune licence n'a encore été définie (choix de l'auteur).

---
Créé avec Claude. Simulation uniquement : aucun avantage quantique n'est revendiqué.
