# CUANTOR

> Un simulateur d'ordinateur quantique pour votre bureau : de 5 à 1000 qubits, avec une interface de type IBM Quantum Composer.

**C'est une simulation exacte exécutée sur votre propre ordinateur, pas un matériel quantique.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

## Ce que fait le programme

- Éditeur de style Composer : palette de portes colorées, fils q[0..n], cadrans de mesure par qubit, histogramme des probabilités, sphère Q avec anneau de phase, code OpenQASM 3.0 / 2.0 / Qiskit en direct.
- Trois moteurs exacts : vecteur d'état (jusqu'à 28 qubits), stabilisateur pour les circuits Clifford (jusqu'à 4096 qubits), amplitudes creuses pour les circuits à peu de branches.
- Circuits dynamiques (mesure en cours de circuit, reset, if), portes définies par l'utilisateur, bruit de dépolarisation et de lecture, graines reproductibles.
- Import OpenQASM 2.0 et 3.0, export OpenQASM 3.0 et Qiskit.
- Lien avec IBM Quantum : ouvrez les circuits dans IBM Composer ou exécutez-les sur du matériel IBM réel via l'API REST Runtime, avec votre propre clé.
- Interface en 8 langues : roumain, anglais, français, allemand, espagnol, italien, portugais, hongrois.
- Windows, macOS et Linux ; un seul espace de travail Rust : noyau, interface graphique, CLI, client IBM.

## Limites physiques

Un état général de n qubits exige 2^n amplitudes : 24 qubits occupent environ 256 Mio, 28 qubits environ 4 Gio. Au-delà, seuls les circuits structurés fonctionnent (Clifford, états de type GHZ, peu de branches de superposition). Un état GHZ de 1000 qubits s'exécute instantanément ; un circuit arbitraire de 1000 qubits est mathématiquement hors de portée de toute machine classique.

## Démarrage rapide

Compilez depuis les sources avec une chaîne d'outils Rust récente :

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

## IBM Quantum (quantum.cloud.ibm.com)

- Composer : le menu Fichier → « Ouvrir dans IBM Composer » enregistre un fichier .qasm ; dans Composer, utilisez File → Upload .qasm file.
- Matériel réel : définissez localement IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN et IBM_QUANTUM_BACKEND (ou ~/.config/cuantor/ibm.json), puis lancez `cuantor ibm run circuit.qasm --backend NOM`. La clé n'est jamais affichée, journalisée ni envoyée ailleurs qu'à IBM.

> Non vérifié sur du matériel réel : le transpileur (portes natives rz/sx/x + cx/cz/ecr, routage SWAP) est vérifié avec le simulateur intégré, et le client REST seulement avec un serveur factice. Les détails des requêtes peuvent nécessiter des ajustements ; les circuits dynamiques ne sont pas pris en charge sur cette voie.

## Qualité et audit

La version 0.2.1 a subi un audit adversarial : 59 tests automatisés, dont du fuzzing de l'analyseur, des bombes de ressources (mémoire, récursion, imbrication), des angles NaN, des recoupements entre les trois moteurs, des tests du khi-deux de l'échantillonnage, la reproductibilité par graine et des réponses REST hostiles. L'audit a trouvé et corrigé 7 défauts (voir le journal des modifications).

[Journal des modifications](CHANGELOG.fr.md)

## Téléchargement

Des binaires pour Windows, macOS (Apple silicon et Intel) et Linux sont joints à chaque version.

## Licence

Aucune licence n'a encore été définie (choix de l'auteur).

---
Créé avec Claude. Simulation uniquement : aucun avantage quantique n'est revendiqué.
