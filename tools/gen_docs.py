#!/usr/bin/env python3
"""Generează README-urile, CHANGELOG-urile și pagina de prezentare în 8 limbi dintr-o singură sursă.

Rulare:  python3 tools/gen_docs.py [numar_teste]
Rezultat: README.md (EN), docs/README.<lang>.md, docs/CHANGELOG.<lang>.md, CHANGELOG.md (EN), site/**
"""
import html
import os
import sys

VER = "0.2.0"
TESTS = sys.argv[1] if len(sys.argv) > 1 else "59"
LANGS = ["ro", "en", "fr", "de", "es", "it", "pt", "hu"]
NAMES = {"ro": "Română", "en": "English", "fr": "Français", "de": "Deutsch", "es": "Español", "it": "Italiano", "pt": "Português", "hu": "Magyar"}

T = {}

T["en"] = dict(
    tagline="A quantum computer simulator for your desktop: 5 to 1000 qubits, with an IBM Quantum Composer-style interface.",
    honest="It is an exact simulation running on your own computer, not quantum hardware.",
    nav=["Features", "Limits", "Quick start", "IBM Quantum", "Quality", "Download"],
    h_feat="What it does",
    feat=[
        "Composer-style editor: colored gate palette, wires q[0..n], per-qubit measurement dials, probabilities histogram, Q-sphere with phase ring, live OpenQASM 3.0 / 2.0 / Qiskit code panel.",
        "Three exact engines: state vector (up to 28 qubits), stabilizer for Clifford circuits (up to 4096 qubits), sparse amplitudes for few-branch circuits.",
        "Dynamic circuits (mid-circuit measurement, reset, if), user-defined gates, depolarizing and readout noise, reproducible seeds.",
        "OpenQASM 2.0 and 3.0 import, OpenQASM 3.0 and Qiskit export.",
        "IBM Quantum link: open circuits in IBM Composer, or run them on real IBM hardware through the Runtime REST API with your own key.",
        "Interface in 8 languages: Romanian, English, French, German, Spanish, Italian, Portuguese, Hungarian.",
        "Windows, macOS and Linux; one Rust workspace: core, GUI, CLI, IBM client.",
    ],
    h_lim="Physical limits",
    lim="A general state of n qubits needs 2^n amplitudes: 24 qubits take about 256 MiB, 28 qubits about 4 GiB. Beyond that only structured circuits work (Clifford circuits, GHZ-like states, few superposition branches). A 1000-qubit GHZ state runs instantly; an arbitrary 1000-qubit circuit is mathematically out of reach for any classical machine.",
    h_start="Quick start",
    start="Build from source with a current Rust toolchain:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: File menu → “Open in IBM Composer” saves a .qasm file; in Composer use File → Upload .qasm file.",
        "Real hardware: set IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN and IBM_QUANTUM_BACKEND locally (or ~/.config/cuantor/ibm.json), then run `cuantor ibm run circuit.qasm --backend NAME`. The key is never printed, logged or sent anywhere except to IBM.",
    ],
    ibm_warn="Not verified on real hardware: the transpiler (native rz/sx/x + cx/cz/ecr gates, SWAP routing) is verified against the built-in simulator, and the REST client only against a mock server. Request details may need adjusting; dynamic circuits are not supported on this path.",
    h_q="Quality and audit",
    q=f"Version {VER} went through an adversarial audit: {TESTS} automated tests, including fuzzing of the parser, resource bombs (memory, recursion, nesting), NaN angles, cross-checks between the three engines, chi-square sampling tests, seed reproducibility and hostile REST responses. The audit found and fixed 7 defects (see the changelog).",
    h_dl="Download",
    dl="Binaries for Windows, macOS (Apple silicon and Intel) and Linux are attached to each release.",
    dl_btn="Latest release",
    h_lic="License",
    lic="No license has been set yet (the author's choice).",
    foot="Built with Claude. Simulation only: no quantum advantage is claimed.",
    cl_title="Changelog",
    cl_added="Added",
    cl_fixed="Fixed",
    cl_notes="Notes",
    added=[
        "Interface in 8 languages (GUI with a language selector, CLI help and errors).",
        "Adversarial test suite: parser fuzzing, resource bombs, engine cross-checks, statistics, hostile REST responses.",
        "IBM Quantum integration: transpiler to native gates with routing, Runtime REST client, Composer export.",
        "Presentation website and documentation in 8 languages.",
    ],
    fixed=[
        "Exponential expansion of nested user-defined gates could allocate gigabytes and abort the process; expansion is now bounded.",
        "Deeply nested expressions overflowed the stack and aborted the process; nesting depth is now limited to 128.",
        "Sampling results depended on the number of threads; results now depend only on the seed.",
        "The GUI started an unbounded number of analysis threads under rapid edits (memory exhaustion at 4096 qubits); there is now one worker plus one queued request.",
        "Invalid noise probabilities (above 1, negative, NaN) were silently accepted; they are now rejected.",
        "The CLI silently ignored invalid numeric options such as --shots abc and unknown engines; they are now errors.",
        "Several symbols were missing from the default font and rendered as empty boxes; replaced with text labels.",
    ],
    notes=["Not verified on real IBM hardware.", "Dynamic circuits cannot be drawn in the editor or sent to IBM hardware.", "Example titles remain in Romanian."],
)

T["ro"] = dict(
    tagline="Un simulator de computer cuantic pentru desktop: de la 5 la 1000 de qubiți, cu interfață în stilul IBM Quantum Composer.",
    honest="Este o simulare exactă care rulează pe propriul tău calculator, nu hardware cuantic.",
    nav=["Funcții", "Limite", "Pornire rapidă", "IBM Quantum", "Calitate", "Descărcare"],
    h_feat="Ce face",
    feat=[
        "Editor în stil Composer: paletă de porți colorate, fire q[0..n], cadrane de măsurare pe fiecare qubit, histogramă de probabilități, Q-sferă cu inel de fază, panou de cod OpenQASM 3.0 / 2.0 / Qiskit în timp real.",
        "Trei motoare exacte: vector de stare (până la 28 de qubiți), stabilizator pentru circuite Clifford (până la 4096 de qubiți), amplitudini rare pentru circuite cu puține ramuri.",
        "Circuite dinamice (măsurare la mijloc, reset, if), porți definite de utilizator, zgomot de depolarizare și de citire, semințe reproductibile.",
        "Import OpenQASM 2.0 și 3.0, export OpenQASM 3.0 și Qiskit.",
        "Legătura cu IBM Quantum: deschide circuitele în IBM Composer sau rulează-le pe hardware IBM real prin API-ul REST Runtime, cu propria ta cheie.",
        "Interfață în 8 limbi: română, engleză, franceză, germană, spaniolă, italiană, portugheză, maghiară.",
        "Windows, macOS și Linux; un singur workspace Rust: nucleu, GUI, CLI, client IBM.",
    ],
    h_lim="Limite fizice",
    lim="O stare generală de n qubiți cere 2^n amplitudini: 24 de qubiți ocupă circa 256 MiB, 28 de qubiți circa 4 GiB. Peste această limită merg doar circuitele cu structură (Clifford, stări de tip GHZ, puține ramuri de superpoziție). Un GHZ de 1000 de qubiți rulează instant; un circuit arbitrar de 1000 de qubiți este matematic inaccesibil oricărei mașini clasice.",
    h_start="Pornire rapidă",
    start="Compilează din sursă cu un toolchain Rust actual:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: meniul Fișier → «Deschide în IBM Composer» salvează un fișier .qasm; în Composer folosește File → Upload .qasm file.",
        "Hardware real: setează local IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN și IBM_QUANTUM_BACKEND (sau ~/.config/cuantor/ibm.json), apoi rulează `cuantor ibm run circuit.qasm --backend NUME`. Cheia nu este afișată, logată sau trimisă nicăieri în afară de IBM.",
    ],
    ibm_warn="Neverificat pe hardware real: transpilatorul (porți native rz/sx/x + cx/cz/ecr, rutare SWAP) este verificat cu simulatorul propriu, iar clientul REST doar cu un server simulat. Detaliile cererilor pot cere ajustări; circuitele dinamice nu sunt suportate pe această cale.",
    h_q="Calitate și audit",
    q=f"Versiunea {VER} a trecut printr-un audit adversarial: {TESTS} de teste automate, inclusiv fuzzing al parserului, bombe de resurse (memorie, recursie, imbricare), unghiuri NaN, verificări încrucișate între cele trei motoare, teste chi-pătrat ale eșantionării, reproductibilitate pe sămânță și răspunsuri REST ostile. Auditul a găsit și a reparat 7 defecte (vezi jurnalul de modificări).",
    h_dl="Descărcare",
    dl="Fișiere executabile pentru Windows, macOS (Apple silicon și Intel) și Linux sunt atașate fiecărei versiuni.",
    dl_btn="Ultima versiune",
    h_lic="Licență",
    lic="Nu a fost setată încă o licență (alegerea autorului).",
    foot="Construit cu Claude. Doar simulare: nu se pretinde avantaj cuantic.",
    cl_title="Jurnal de modificări",
    cl_added="Adăugat",
    cl_fixed="Reparat",
    cl_notes="Note",
    added=[
        "Interfață în 8 limbi (GUI cu selector de limbă, ajutor și erori în CLI).",
        "Suită de teste adversariale: fuzzing de parser, bombe de resurse, verificări între motoare, statistică, răspunsuri REST ostile.",
        "Integrare IBM Quantum: transpilator în porți native cu rutare, client REST Runtime, export Composer.",
        "Site de prezentare și documentație în 8 limbi.",
    ],
    fixed=[
        "Expansiunea exponențială a porților definite de utilizator putea aloca gigaocteți și opri procesul; expansiunea este acum limitată.",
        "Expresiile imbricate adânc provocau depășirea stivei și oprirea procesului; adâncimea e limitată la 128.",
        "Rezultatele eșantionării depindeau de numărul de fire; acum depind doar de sămânță.",
        "GUI pornea un număr nelimitat de fire de analiză la editări rapide (memorie epuizată la 4096 de qubiți); acum există un fir de lucru și o cerere în coadă.",
        "Probabilități de zgomot invalide (peste 1, negative, NaN) erau acceptate tăcut; acum sunt respinse.",
        "CLI ignora tăcut opțiuni numerice invalide, precum --shots abc, și motoare necunoscute; acum sunt erori.",
        "Mai multe simboluri lipseau din fontul implicit și apăreau ca pătrate goale; au fost înlocuite cu etichete text.",
    ],
    notes=["Neverificat pe hardware IBM real.", "Circuitele dinamice nu pot fi desenate în editor sau trimise pe hardware IBM.", "Titlurile exemplelor rămân în română."],
)

T["fr"] = dict(
    tagline="Un simulateur d'ordinateur quantique pour votre bureau : de 5 à 1000 qubits, avec une interface de type IBM Quantum Composer.",
    honest="C'est une simulation exacte exécutée sur votre propre ordinateur, pas un matériel quantique.",
    nav=["Fonctions", "Limites", "Démarrage rapide", "IBM Quantum", "Qualité", "Téléchargement"],
    h_feat="Ce que fait le programme",
    feat=[
        "Éditeur de style Composer : palette de portes colorées, fils q[0..n], cadrans de mesure par qubit, histogramme des probabilités, sphère Q avec anneau de phase, code OpenQASM 3.0 / 2.0 / Qiskit en direct.",
        "Trois moteurs exacts : vecteur d'état (jusqu'à 28 qubits), stabilisateur pour les circuits Clifford (jusqu'à 4096 qubits), amplitudes creuses pour les circuits à peu de branches.",
        "Circuits dynamiques (mesure en cours de circuit, reset, if), portes définies par l'utilisateur, bruit de dépolarisation et de lecture, graines reproductibles.",
        "Import OpenQASM 2.0 et 3.0, export OpenQASM 3.0 et Qiskit.",
        "Lien avec IBM Quantum : ouvrez les circuits dans IBM Composer ou exécutez-les sur du matériel IBM réel via l'API REST Runtime, avec votre propre clé.",
        "Interface en 8 langues : roumain, anglais, français, allemand, espagnol, italien, portugais, hongrois.",
        "Windows, macOS et Linux ; un seul espace de travail Rust : noyau, interface graphique, CLI, client IBM.",
    ],
    h_lim="Limites physiques",
    lim="Un état général de n qubits exige 2^n amplitudes : 24 qubits occupent environ 256 Mio, 28 qubits environ 4 Gio. Au-delà, seuls les circuits structurés fonctionnent (Clifford, états de type GHZ, peu de branches de superposition). Un état GHZ de 1000 qubits s'exécute instantanément ; un circuit arbitraire de 1000 qubits est mathématiquement hors de portée de toute machine classique.",
    h_start="Démarrage rapide",
    start="Compilez depuis les sources avec une chaîne d'outils Rust récente :",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer : le menu Fichier → « Ouvrir dans IBM Composer » enregistre un fichier .qasm ; dans Composer, utilisez File → Upload .qasm file.",
        "Matériel réel : définissez localement IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN et IBM_QUANTUM_BACKEND (ou ~/.config/cuantor/ibm.json), puis lancez `cuantor ibm run circuit.qasm --backend NOM`. La clé n'est jamais affichée, journalisée ni envoyée ailleurs qu'à IBM.",
    ],
    ibm_warn="Non vérifié sur du matériel réel : le transpileur (portes natives rz/sx/x + cx/cz/ecr, routage SWAP) est vérifié avec le simulateur intégré, et le client REST seulement avec un serveur factice. Les détails des requêtes peuvent nécessiter des ajustements ; les circuits dynamiques ne sont pas pris en charge sur cette voie.",
    h_q="Qualité et audit",
    q=f"La version {VER} a subi un audit adversarial : {TESTS} tests automatisés, dont du fuzzing de l'analyseur, des bombes de ressources (mémoire, récursion, imbrication), des angles NaN, des recoupements entre les trois moteurs, des tests du khi-deux de l'échantillonnage, la reproductibilité par graine et des réponses REST hostiles. L'audit a trouvé et corrigé 7 défauts (voir le journal des modifications).",
    h_dl="Téléchargement",
    dl="Des binaires pour Windows, macOS (Apple silicon et Intel) et Linux sont joints à chaque version.",
    dl_btn="Dernière version",
    h_lic="Licence",
    lic="Aucune licence n'a encore été définie (choix de l'auteur).",
    foot="Créé avec Claude. Simulation uniquement : aucun avantage quantique n'est revendiqué.",
    cl_title="Journal des modifications",
    cl_added="Ajouté",
    cl_fixed="Corrigé",
    cl_notes="Notes",
    added=[
        "Interface en 8 langues (interface graphique avec sélecteur de langue, aide et erreurs de la CLI).",
        "Suite de tests adversariaux : fuzzing de l'analyseur, bombes de ressources, recoupements entre moteurs, statistiques, réponses REST hostiles.",
        "Intégration IBM Quantum : transpileur vers les portes natives avec routage, client REST Runtime, export vers Composer.",
        "Site de présentation et documentation en 8 langues.",
    ],
    fixed=[
        "L'expansion exponentielle de portes définies par l'utilisateur pouvait allouer des gigaoctets et arrêter le processus ; l'expansion est désormais bornée.",
        "Des expressions profondément imbriquées provoquaient un débordement de pile et l'arrêt du processus ; la profondeur est limitée à 128.",
        "Les résultats d'échantillonnage dépendaient du nombre de threads ; ils ne dépendent plus que de la graine.",
        "L'interface lançait un nombre illimité de threads d'analyse lors de modifications rapides (mémoire épuisée à 4096 qubits) ; il y a maintenant un thread de travail et une requête en file.",
        "Les probabilités de bruit invalides (supérieures à 1, négatives, NaN) étaient acceptées silencieusement ; elles sont maintenant rejetées.",
        "La CLI ignorait silencieusement des options numériques invalides comme --shots abc et des moteurs inconnus ; ce sont maintenant des erreurs.",
        "Plusieurs symboles manquaient dans la police par défaut et s'affichaient comme des cases vides ; ils sont remplacés par des étiquettes textuelles.",
    ],
    notes=["Non vérifié sur du matériel IBM réel.", "Les circuits dynamiques ne peuvent être ni dessinés dans l'éditeur ni envoyés au matériel IBM.", "Les titres des exemples restent en roumain."],
)

T["de"] = dict(
    tagline="Ein Quantencomputer-Simulator für den Desktop: 5 bis 1000 Qubits, mit einer Oberfläche im Stil des IBM Quantum Composer.",
    honest="Es ist eine exakte Simulation auf dem eigenen Rechner, keine Quantenhardware.",
    nav=["Funktionen", "Grenzen", "Schnellstart", "IBM Quantum", "Qualität", "Download"],
    h_feat="Was es kann",
    feat=[
        "Editor im Composer-Stil: farbige Gatterpalette, Leitungen q[0..n], Messanzeigen je Qubit, Wahrscheinlichkeits-Histogramm, Q-Sphäre mit Phasenring, Live-Code in OpenQASM 3.0 / 2.0 / Qiskit.",
        "Drei exakte Engines: Zustandsvektor (bis 28 Qubits), Stabilisator für Clifford-Schaltungen (bis 4096 Qubits), dünn besetzte Amplituden für Schaltungen mit wenigen Zweigen.",
        "Dynamische Schaltungen (Messung mitten im Ablauf, Reset, if), benutzerdefinierte Gatter, Depolarisations- und Ausleserauschen, reproduzierbare Seeds.",
        "Import von OpenQASM 2.0 und 3.0, Export als OpenQASM 3.0 und Qiskit.",
        "Anbindung an IBM Quantum: Schaltungen in IBM Composer öffnen oder über die Runtime-REST-API mit dem eigenen Schlüssel auf echter IBM-Hardware ausführen.",
        "Oberfläche in 8 Sprachen: Rumänisch, Englisch, Französisch, Deutsch, Spanisch, Italienisch, Portugiesisch, Ungarisch.",
        "Windows, macOS und Linux; ein Rust-Workspace: Kern, GUI, CLI, IBM-Client.",
    ],
    h_lim="Physikalische Grenzen",
    lim="Ein allgemeiner Zustand aus n Qubits braucht 2^n Amplituden: 24 Qubits belegen etwa 256 MiB, 28 Qubits etwa 4 GiB. Darüber hinaus funktionieren nur strukturierte Schaltungen (Clifford, GHZ-artige Zustände, wenige Überlagerungszweige). Ein GHZ-Zustand mit 1000 Qubits läuft sofort; eine beliebige Schaltung mit 1000 Qubits ist für jeden klassischen Rechner mathematisch unerreichbar.",
    h_start="Schnellstart",
    start="Aus dem Quellcode mit einer aktuellen Rust-Toolchain bauen:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: Menü Datei → „In IBM Composer öffnen“ speichert eine .qasm-Datei; in Composer File → Upload .qasm file verwenden.",
        "Echte Hardware: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN und IBM_QUANTUM_BACKEND lokal setzen (oder ~/.config/cuantor/ibm.json) und `cuantor ibm run circuit.qasm --backend NAME` ausführen. Der Schlüssel wird nie ausgegeben, protokolliert oder an andere Stellen als IBM gesendet.",
    ],
    ibm_warn="Auf echter Hardware nicht verifiziert: Der Transpiler (native Gatter rz/sx/x + cx/cz/ecr, SWAP-Routing) ist mit dem eingebauten Simulator geprüft, der REST-Client nur gegen einen Mock-Server. Anfragedetails können Anpassungen erfordern; dynamische Schaltungen werden auf diesem Weg nicht unterstützt.",
    h_q="Qualität und Audit",
    q=f"Version {VER} durchlief ein adversariales Audit: {TESTS} automatisierte Tests, darunter Fuzzing des Parsers, Ressourcenbomben (Speicher, Rekursion, Verschachtelung), NaN-Winkel, Kreuzprüfungen der drei Engines, Chi-Quadrat-Tests der Stichproben, Reproduzierbarkeit per Seed und feindliche REST-Antworten. Das Audit fand 7 Fehler und behob sie (siehe Änderungsprotokoll).",
    h_dl="Download",
    dl="Programme für Windows, macOS (Apple Silicon und Intel) und Linux hängen an jedem Release.",
    dl_btn="Neuestes Release",
    h_lic="Lizenz",
    lic="Es wurde noch keine Lizenz festgelegt (Entscheidung des Autors).",
    foot="Mit Claude gebaut. Nur Simulation: Es wird kein Quantenvorteil behauptet.",
    cl_title="Änderungsprotokoll",
    cl_added="Hinzugefügt",
    cl_fixed="Behoben",
    cl_notes="Hinweise",
    added=[
        "Oberfläche in 8 Sprachen (GUI mit Sprachauswahl, CLI-Hilfe und Fehlermeldungen).",
        "Adversariale Testsuite: Parser-Fuzzing, Ressourcenbomben, Engine-Kreuzprüfungen, Statistik, feindliche REST-Antworten.",
        "IBM-Quantum-Integration: Transpiler in native Gatter mit Routing, Runtime-REST-Client, Composer-Export.",
        "Präsentationswebsite und Dokumentation in 8 Sprachen.",
    ],
    fixed=[
        "Die exponentielle Expansion verschachtelter benutzerdefinierter Gatter konnte Gigabytes belegen und den Prozess abbrechen; die Expansion ist jetzt begrenzt.",
        "Tief verschachtelte Ausdrücke verursachten einen Stacküberlauf und brachen den Prozess ab; die Tiefe ist auf 128 begrenzt.",
        "Stichprobenergebnisse hingen von der Threadzahl ab; sie hängen jetzt nur noch vom Seed ab.",
        "Die GUI startete bei schnellen Änderungen unbegrenzt viele Analyse-Threads (Speicher erschöpft bei 4096 Qubits); jetzt gibt es einen Arbeits-Thread und eine wartende Anfrage.",
        "Ungültige Rauschwahrscheinlichkeiten (über 1, negativ, NaN) wurden stillschweigend akzeptiert; sie werden jetzt abgelehnt.",
        "Die CLI ignorierte ungültige numerische Optionen wie --shots abc und unbekannte Engines stillschweigend; das sind jetzt Fehler.",
        "Mehrere Symbole fehlten in der Standardschrift und erschienen als leere Kästchen; sie wurden durch Textbeschriftungen ersetzt.",
    ],
    notes=["Auf echter IBM-Hardware nicht verifiziert.", "Dynamische Schaltungen lassen sich weder im Editor zeichnen noch an IBM-Hardware senden.", "Die Titel der Beispiele bleiben rumänisch."],
)

T["es"] = dict(
    tagline="Un simulador de computador cuántico para tu escritorio: de 5 a 1000 cúbits, con una interfaz al estilo de IBM Quantum Composer.",
    honest="Es una simulación exacta que se ejecuta en tu propio ordenador, no hardware cuántico.",
    nav=["Funciones", "Límites", "Inicio rápido", "IBM Quantum", "Calidad", "Descarga"],
    h_feat="Qué hace",
    feat=[
        "Editor al estilo Composer: paleta de puertas de colores, hilos q[0..n], diales de medición por cúbit, histograma de probabilidades, esfera Q con anillo de fase y código OpenQASM 3.0 / 2.0 / Qiskit en vivo.",
        "Tres motores exactos: vector de estado (hasta 28 cúbits), estabilizador para circuitos Clifford (hasta 4096 cúbits) y amplitudes dispersas para circuitos con pocas ramas.",
        "Circuitos dinámicos (medición intermedia, reset, if), puertas definidas por el usuario, ruido de despolarización y de lectura, semillas reproducibles.",
        "Importación de OpenQASM 2.0 y 3.0, exportación a OpenQASM 3.0 y Qiskit.",
        "Enlace con IBM Quantum: abre los circuitos en IBM Composer o ejecútalos en hardware real de IBM mediante la API REST de Runtime, con tu propia clave.",
        "Interfaz en 8 idiomas: rumano, inglés, francés, alemán, español, italiano, portugués, húngaro.",
        "Windows, macOS y Linux; un único espacio de trabajo Rust: núcleo, GUI, CLI, cliente de IBM.",
    ],
    h_lim="Límites físicos",
    lim="Un estado general de n cúbits necesita 2^n amplitudes: 24 cúbits ocupan unos 256 MiB y 28 cúbits unos 4 GiB. Más allá solo funcionan los circuitos con estructura (Clifford, estados tipo GHZ, pocas ramas de superposición). Un GHZ de 1000 cúbits se ejecuta al instante; un circuito arbitrario de 1000 cúbits es matemáticamente inalcanzable para cualquier máquina clásica.",
    h_start="Inicio rápido",
    start="Compila desde el código fuente con una cadena de herramientas de Rust actual:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: el menú Archivo → «Abrir en IBM Composer» guarda un archivo .qasm; en Composer usa File → Upload .qasm file.",
        "Hardware real: define localmente IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o ~/.config/cuantor/ibm.json) y ejecuta `cuantor ibm run circuit.qasm --backend NOMBRE`. La clave nunca se muestra, se registra ni se envía a otro sitio que no sea IBM.",
    ],
    ibm_warn="No verificado en hardware real: el transpilador (puertas nativas rz/sx/x + cx/cz/ecr, enrutamiento SWAP) se verifica con el simulador integrado, y el cliente REST solo contra un servidor simulado. Los detalles de las solicitudes pueden requerir ajustes; los circuitos dinámicos no se admiten por esta vía.",
    h_q="Calidad y auditoría",
    q=f"La versión {VER} pasó por una auditoría adversarial: {TESTS} pruebas automáticas, con fuzzing del analizador, bombas de recursos (memoria, recursión, anidamiento), ángulos NaN, contrastes entre los tres motores, pruebas de chi cuadrado del muestreo, reproducibilidad por semilla y respuestas REST hostiles. La auditoría encontró y corrigió 7 defectos (véase el registro de cambios).",
    h_dl="Descarga",
    dl="Cada versión incluye binarios para Windows, macOS (Apple silicon e Intel) y Linux.",
    dl_btn="Última versión",
    h_lic="Licencia",
    lic="Todavía no se ha definido una licencia (decisión del autor).",
    foot="Creado con Claude. Solo simulación: no se afirma ninguna ventaja cuántica.",
    cl_title="Registro de cambios",
    cl_added="Añadido",
    cl_fixed="Corregido",
    cl_notes="Notas",
    added=[
        "Interfaz en 8 idiomas (GUI con selector de idioma, ayuda y errores de la CLI).",
        "Batería de pruebas adversariales: fuzzing del analizador, bombas de recursos, contrastes entre motores, estadística, respuestas REST hostiles.",
        "Integración con IBM Quantum: transpilador a puertas nativas con enrutamiento, cliente REST de Runtime, exportación a Composer.",
        "Sitio web de presentación y documentación en 8 idiomas.",
    ],
    fixed=[
        "La expansión exponencial de puertas definidas por el usuario podía reservar gigabytes y abortar el proceso; la expansión ahora está acotada.",
        "Las expresiones muy anidadas desbordaban la pila y abortaban el proceso; la profundidad se limita a 128.",
        "Los resultados del muestreo dependían del número de hilos; ahora dependen solo de la semilla.",
        "La GUI lanzaba un número ilimitado de hilos de análisis con ediciones rápidas (memoria agotada con 4096 cúbits); ahora hay un hilo de trabajo y una solicitud en cola.",
        "Las probabilidades de ruido no válidas (mayores que 1, negativas, NaN) se aceptaban en silencio; ahora se rechazan.",
        "La CLI ignoraba en silencio opciones numéricas no válidas como --shots abc y motores desconocidos; ahora son errores.",
        "Varios símbolos faltaban en la fuente predeterminada y aparecían como recuadros vacíos; se sustituyeron por etiquetas de texto.",
    ],
    notes=["No verificado en hardware real de IBM.", "Los circuitos dinámicos no se pueden dibujar en el editor ni enviar al hardware de IBM.", "Los títulos de los ejemplos siguen en rumano."],
)

T["it"] = dict(
    tagline="Un simulatore di computer quantistico per il desktop: da 5 a 1000 qubit, con un'interfaccia in stile IBM Quantum Composer.",
    honest="È una simulazione esatta eseguita sul tuo computer, non hardware quantistico.",
    nav=["Funzioni", "Limiti", "Avvio rapido", "IBM Quantum", "Qualità", "Download"],
    h_feat="Cosa fa",
    feat=[
        "Editor in stile Composer: tavolozza di porte colorate, fili q[0..n], quadranti di misura per ogni qubit, istogramma delle probabilità, sfera Q con anello di fase e codice OpenQASM 3.0 / 2.0 / Qiskit in tempo reale.",
        "Tre motori esatti: vettore di stato (fino a 28 qubit), stabilizzatore per circuiti Clifford (fino a 4096 qubit) e ampiezze sparse per circuiti con pochi rami.",
        "Circuiti dinamici (misura a metà circuito, reset, if), porte definite dall'utente, rumore di depolarizzazione e di lettura, semi riproducibili.",
        "Importazione di OpenQASM 2.0 e 3.0, esportazione in OpenQASM 3.0 e Qiskit.",
        "Collegamento a IBM Quantum: apri i circuiti in IBM Composer oppure eseguili su hardware IBM reale tramite l'API REST Runtime, con la tua chiave.",
        "Interfaccia in 8 lingue: rumeno, inglese, francese, tedesco, spagnolo, italiano, portoghese, ungherese.",
        "Windows, macOS e Linux; un unico workspace Rust: nucleo, GUI, CLI, client IBM.",
    ],
    h_lim="Limiti fisici",
    lim="Uno stato generale di n qubit richiede 2^n ampiezze: 24 qubit occupano circa 256 MiB, 28 qubit circa 4 GiB. Oltre funzionano solo circuiti strutturati (Clifford, stati di tipo GHZ, pochi rami di sovrapposizione). Un GHZ da 1000 qubit gira all'istante; un circuito arbitrario da 1000 qubit è matematicamente fuori portata per qualsiasi macchina classica.",
    h_start="Avvio rapido",
    start="Compila dai sorgenti con una toolchain Rust aggiornata:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: il menu File → «Apri in IBM Composer» salva un file .qasm; in Composer usa File → Upload .qasm file.",
        "Hardware reale: imposta in locale IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (o ~/.config/cuantor/ibm.json), poi esegui `cuantor ibm run circuit.qasm --backend NOME`. La chiave non viene mai mostrata, registrata o inviata ad altri che a IBM.",
    ],
    ibm_warn="Non verificato su hardware reale: il transpilatore (porte native rz/sx/x + cx/cz/ecr, instradamento SWAP) è verificato con il simulatore integrato, il client REST solo con un server simulato. I dettagli delle richieste possono richiedere aggiustamenti; i circuiti dinamici non sono supportati su questo percorso.",
    h_q="Qualità e audit",
    q=f"La versione {VER} ha superato un audit avversariale: {TESTS} test automatici, tra cui fuzzing del parser, bombe di risorse (memoria, ricorsione, annidamento), angoli NaN, verifiche incrociate tra i tre motori, test chi quadrato del campionamento, riproducibilità tramite seme e risposte REST ostili. L'audit ha trovato e corretto 7 difetti (vedi il registro delle modifiche).",
    h_dl="Download",
    dl="Ogni versione include i binari per Windows, macOS (Apple silicon e Intel) e Linux.",
    dl_btn="Ultima versione",
    h_lic="Licenza",
    lic="Non è ancora stata impostata una licenza (scelta dell'autore).",
    foot="Realizzato con Claude. Solo simulazione: non si rivendica alcun vantaggio quantistico.",
    cl_title="Registro delle modifiche",
    cl_added="Aggiunto",
    cl_fixed="Corretto",
    cl_notes="Note",
    added=[
        "Interfaccia in 8 lingue (GUI con selettore di lingua, guida ed errori della CLI).",
        "Suite di test avversariali: fuzzing del parser, bombe di risorse, verifiche incrociate tra motori, statistica, risposte REST ostili.",
        "Integrazione con IBM Quantum: transpilatore verso porte native con instradamento, client REST Runtime, esportazione in Composer.",
        "Sito di presentazione e documentazione in 8 lingue.",
    ],
    fixed=[
        "L'espansione esponenziale di porte definite dall'utente poteva allocare gigabyte e interrompere il processo; l'espansione ora è limitata.",
        "Le espressioni molto annidate causavano overflow dello stack e l'interruzione del processo; la profondità è limitata a 128.",
        "I risultati del campionamento dipendevano dal numero di thread; ora dipendono solo dal seme.",
        "La GUI avviava un numero illimitato di thread di analisi con modifiche rapide (memoria esaurita a 4096 qubit); ora c'è un thread di lavoro e una richiesta in coda.",
        "Le probabilità di rumore non valide (superiori a 1, negative, NaN) venivano accettate in silenzio; ora vengono rifiutate.",
        "La CLI ignorava in silenzio opzioni numeriche non valide come --shots abc e motori sconosciuti; ora sono errori.",
        "Diversi simboli mancavano nel carattere predefinito e comparivano come riquadri vuoti; sono stati sostituiti da etichette di testo.",
    ],
    notes=["Non verificato su hardware IBM reale.", "I circuiti dinamici non possono essere disegnati nell'editor né inviati all'hardware IBM.", "I titoli degli esempi restano in rumeno."],
)

T["pt"] = dict(
    tagline="Um simulador de computador quântico para o seu ambiente de trabalho: de 5 a 1000 qubits, com uma interface ao estilo do IBM Quantum Composer.",
    honest="É uma simulação exata que corre no seu próprio computador, não hardware quântico.",
    nav=["Funções", "Limites", "Início rápido", "IBM Quantum", "Qualidade", "Transferência"],
    h_feat="O que faz",
    feat=[
        "Editor ao estilo Composer: paleta de portas coloridas, fios q[0..n], mostradores de medição por qubit, histograma de probabilidades, esfera Q com anel de fase e código OpenQASM 3.0 / 2.0 / Qiskit em tempo real.",
        "Três motores exatos: vetor de estado (até 28 qubits), estabilizador para circuitos Clifford (até 4096 qubits) e amplitudes esparsas para circuitos com poucos ramos.",
        "Circuitos dinâmicos (medição a meio do circuito, reset, if), portas definidas pelo utilizador, ruído de despolarização e de leitura, sementes reproduzíveis.",
        "Importação de OpenQASM 2.0 e 3.0, exportação para OpenQASM 3.0 e Qiskit.",
        "Ligação ao IBM Quantum: abra circuitos no IBM Composer ou execute-os em hardware IBM real através da API REST Runtime, com a sua própria chave.",
        "Interface em 8 idiomas: romeno, inglês, francês, alemão, espanhol, italiano, português, húngaro.",
        "Windows, macOS e Linux; um único workspace Rust: núcleo, GUI, CLI, cliente IBM.",
    ],
    h_lim="Limites físicos",
    lim="Um estado geral de n qubits exige 2^n amplitudes: 24 qubits ocupam cerca de 256 MiB e 28 qubits cerca de 4 GiB. Além disso só funcionam circuitos com estrutura (Clifford, estados do tipo GHZ, poucos ramos de sobreposição). Um GHZ de 1000 qubits executa-se instantaneamente; um circuito arbitrário de 1000 qubits é matematicamente inacessível a qualquer máquina clássica.",
    h_start="Início rápido",
    start="Compile a partir do código-fonte com uma cadeia de ferramentas Rust atual:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: o menu Ficheiro → «Abrir no IBM Composer» guarda um ficheiro .qasm; no Composer use File → Upload .qasm file.",
        "Hardware real: defina localmente IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN e IBM_QUANTUM_BACKEND (ou ~/.config/cuantor/ibm.json) e execute `cuantor ibm run circuit.qasm --backend NOME`. A chave nunca é mostrada, registada nem enviada para outro lado que não a IBM.",
    ],
    ibm_warn="Não verificado em hardware real: o transpilador (portas nativas rz/sx/x + cx/cz/ecr, encaminhamento SWAP) é verificado com o simulador integrado, e o cliente REST apenas contra um servidor simulado. Os detalhes dos pedidos podem exigir ajustes; os circuitos dinâmicos não são suportados por este caminho.",
    h_q="Qualidade e auditoria",
    q=f"A versão {VER} passou por uma auditoria adversarial: {TESTS} testes automáticos, incluindo fuzzing do analisador, bombas de recursos (memória, recursão, aninhamento), ângulos NaN, verificações cruzadas entre os três motores, testes de qui-quadrado da amostragem, reprodutibilidade por semente e respostas REST hostis. A auditoria encontrou e corrigiu 7 defeitos (veja o registo de alterações).",
    h_dl="Transferência",
    dl="Cada versão inclui binários para Windows, macOS (Apple silicon e Intel) e Linux.",
    dl_btn="Última versão",
    h_lic="Licença",
    lic="Ainda não foi definida uma licença (escolha do autor).",
    foot="Criado com Claude. Apenas simulação: não se reivindica qualquer vantagem quântica.",
    cl_title="Registo de alterações",
    cl_added="Adicionado",
    cl_fixed="Corrigido",
    cl_notes="Notas",
    added=[
        "Interface em 8 idiomas (GUI com seletor de idioma, ajuda e erros da CLI).",
        "Bateria de testes adversariais: fuzzing do analisador, bombas de recursos, verificações cruzadas entre motores, estatística, respostas REST hostis.",
        "Integração com o IBM Quantum: transpilador para portas nativas com encaminhamento, cliente REST Runtime, exportação para o Composer.",
        "Site de apresentação e documentação em 8 idiomas.",
    ],
    fixed=[
        "A expansão exponencial de portas definidas pelo utilizador podia reservar gigabytes e abortar o processo; a expansão passou a ser limitada.",
        "Expressões muito aninhadas provocavam um estouro da pilha e abortavam o processo; a profundidade está limitada a 128.",
        "Os resultados da amostragem dependiam do número de threads; passam a depender apenas da semente.",
        "A GUI iniciava um número ilimitado de threads de análise com edições rápidas (memória esgotada com 4096 qubits); agora há um thread de trabalho e um pedido em fila.",
        "Probabilidades de ruído inválidas (acima de 1, negativas, NaN) eram aceites em silêncio; agora são rejeitadas.",
        "A CLI ignorava em silêncio opções numéricas inválidas como --shots abc e motores desconhecidos; agora são erros.",
        "Vários símbolos faltavam no tipo de letra predefinido e surgiam como quadrados vazios; foram substituídos por etiquetas de texto.",
    ],
    notes=["Não verificado em hardware IBM real.", "Os circuitos dinâmicos não podem ser desenhados no editor nem enviados para hardware IBM.", "Os títulos dos exemplos continuam em romeno."],
)

T["hu"] = dict(
    tagline="Kvantumszámítógép-szimulátor az asztali gépedre: 5-től 1000 qubitig, IBM Quantum Composer stílusú felülettel.",
    honest="Pontos szimuláció, amely a saját számítógépeden fut; nem kvantumhardver.",
    nav=["Funkciók", "Korlátok", "Gyors kezdés", "IBM Quantum", "Minőség", "Letöltés"],
    h_feat="Mit tud",
    feat=[
        "Composer stílusú szerkesztő: színes kapupaletta, q[0..n] vezetékek, qubitenkénti mérőkorongok, valószínűségi hisztogram, Q-gömb fázisgyűrűvel, élő OpenQASM 3.0 / 2.0 / Qiskit kód.",
        "Három pontos motor: állapotvektor (legfeljebb 28 qubit), stabilizátor Clifford-áramkörökhöz (legfeljebb 4096 qubit), ritka amplitúdók kevés ággal rendelkező áramkörökhöz.",
        "Dinamikus áramkörök (közbenső mérés, reset, if), felhasználói kapuk, depolarizációs és kiolvasási zaj, reprodukálható magértékek.",
        "OpenQASM 2.0 és 3.0 import, OpenQASM 3.0 és Qiskit export.",
        "IBM Quantum kapcsolat: az áramkörök megnyithatók az IBM Composerben, vagy futtathatók valódi IBM hardveren a Runtime REST API-n keresztül, a saját kulcsoddal.",
        "8 nyelvű felület: román, angol, francia, német, spanyol, olasz, portugál, magyar.",
        "Windows, macOS és Linux; egyetlen Rust munkaterület: mag, grafikus felület, CLI, IBM kliens.",
    ],
    h_lim="Fizikai korlátok",
    lim="Egy általános n qubites állapot 2^n amplitúdót igényel: 24 qubit körülbelül 256 MiB, 28 qubit körülbelül 4 GiB. Efelett csak szerkezettel rendelkező áramkörök működnek (Clifford, GHZ-szerű állapotok, kevés szuperpozíciós ág). Egy 1000 qubites GHZ-állapot azonnal lefut; egy tetszőleges 1000 qubites áramkör matematikailag elérhetetlen bármely klasszikus gép számára.",
    h_start="Gyors kezdés",
    start="Fordítás forrásból, naprakész Rust eszközlánccal:",
    h_ibm="IBM Quantum (quantum.cloud.ibm.com)",
    ibm=[
        "Composer: a Fájl menü → „Megnyitás az IBM Composerben” .qasm fájlt ment; a Composerben használd a File → Upload .qasm file parancsot.",
        "Valódi hardver: állítsd be helyben az IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN és IBM_QUANTUM_BACKEND változót (vagy a ~/.config/cuantor/ibm.json fájlt), majd futtasd: `cuantor ibm run circuit.qasm --backend NÉV`. A kulcsot a program soha nem írja ki, nem naplózza, és csak az IBM-nek küldi el.",
    ],
    ibm_warn="Valódi hardveren nem ellenőrzött: a transzpiler (natív rz/sx/x + cx/cz/ecr kapuk, SWAP-útválasztás) a beépített szimulátorral van ellenőrizve, a REST kliens pedig csak egy mesterséges szerverrel. A kérések részletei módosításra szorulhatnak; dinamikus áramkörök ezen az úton nem támogatottak.",
    h_q="Minőség és audit",
    q=f"A(z) {VER} verzió ellenséges auditon esett át: {TESTS} automatikus teszt, köztük az elemző fuzzolása, erőforrás-bombák (memória, rekurzió, egymásba ágyazás), NaN szögek, a három motor kereszt-ellenőrzése, a mintavétel khí-négyzet tesztjei, magérték-alapú reprodukálhatóság és ellenséges REST-válaszok. Az audit 7 hibát talált és javított ki (lásd a változásnaplót).",
    h_dl="Letöltés",
    dl="Minden kiadáshoz tartoznak futtatható fájlok Windowsra, macOS-re (Apple silicon és Intel) és Linuxra.",
    dl_btn="Legújabb kiadás",
    h_lic="Licenc",
    lic="Licenc még nincs megadva (a szerző döntése).",
    foot="Claude segítségével készült. Csak szimuláció: kvantumelőnyt nem állít.",
    cl_title="Változásnapló",
    cl_added="Hozzáadva",
    cl_fixed="Javítva",
    cl_notes="Megjegyzések",
    added=[
        "8 nyelvű felület (grafikus felület nyelvválasztóval, CLI súgó és hibaüzenetek).",
        "Ellenséges tesztcsomag: elemző-fuzzolás, erőforrás-bombák, motorok közti ellenőrzések, statisztika, ellenséges REST-válaszok.",
        "IBM Quantum integráció: transzpiler natív kapukra útválasztással, Runtime REST kliens, Composer export.",
        "Bemutatkozó weboldal és dokumentáció 8 nyelven.",
    ],
    fixed=[
        "A felhasználói kapuk exponenciális kibontása gigabájtokat foglalhatott le, és leállíthatta a folyamatot; a kibontás most korlátozott.",
        "A mélyen egymásba ágyazott kifejezések veremtúlcsordulást okoztak és leállították a folyamatot; a mélység 128-ra korlátozott.",
        "A mintavétel eredménye függött a szálak számától; mostantól csak a magértéktől függ.",
        "A grafikus felület gyors szerkesztéskor korlátlan számú elemző szálat indított (4096 qubitnél elfogyott a memória); most egy munkaszál és egy sorban álló kérés van.",
        "Az érvénytelen zajvalószínűségeket (1 felett, negatív, NaN) csendben elfogadta; most elutasítja.",
        "A CLI csendben figyelmen kívül hagyta az érvénytelen számopciókat, például a --shots abc értéket, és az ismeretlen motorokat; ezek most hibák.",
        "Több szimbólum hiányzott az alapértelmezett betűtípusból, és üres négyzetként jelent meg; szöveges címkékre cseréltük őket.",
    ],
    notes=["Valódi IBM hardveren nem ellenőrzött.", "Dinamikus áramkörök nem rajzolhatók meg a szerkesztőben és nem küldhetők IBM hardverre.", "A példák címei románul maradnak."],
)

CODE = """git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release"""

HOWTO_FILES = {
    "ro": "Pentru Windows/macOS/Linux, binarele gata făcute apar la «Releases».",
}


def md_readme(l):
    t = T[l]
    out = [f"# CUANTOR", "", f"> {t['tagline']}", "", f"**{t['honest']}**", ""]
    out.append("[" + " · ".join(f"[{NAMES[x]}]({'README.md' if l == 'en' and x == 'en' else ('docs/README.%s.md' % x if l == 'en' else ('../README.md' if x == 'en' else 'README.%s.md' % x))})" for x in LANGS) + "]")
    out[-1] = " · ".join(f"[{NAMES[x]}]({('README.md' if x == 'en' else 'docs/README.%s.md' % x) if l == 'en' else ('../README.md' if x == 'en' else 'README.%s.md' % x)})" for x in LANGS)
    out.append("")
    out += [f"## {t['h_feat']}", ""] + [f"- {x}" for x in t["feat"]] + ["", f"## {t['h_lim']}", "", t["lim"], "", f"## {t['h_start']}", "", t["start"], "", "```", CODE, "```", "", f"## {t['h_ibm']}", ""]
    out += [f"- {x}" for x in t["ibm"]] + ["", f"> {t['ibm_warn']}", "", f"## {t['h_q']}", "", t["q"], ""]
    cl = "CHANGELOG.md" if l == "en" else f"docs/CHANGELOG.{l}.md"
    cl = "CHANGELOG.md" if l == "en" else f"CHANGELOG.{l}.md"
    out += [f"[{t['cl_title']}]({cl})", "", f"## {t['h_dl']}", "", t["dl"], "", f"## {t['h_lic']}", "", t["lic"], "", "---", t["foot"], ""]
    return "\n".join(out)


def md_changelog(l):
    t = T[l]
    out = [f"# {t['cl_title']}", "", f"## {VER}", "", f"### {t['cl_added']}", ""] + [f"- {x}" for x in t["added"]]
    out += ["", f"### {t['cl_fixed']}", ""] + [f"- {x}" for x in t["fixed"]]
    out += ["", f"### {t['cl_notes']}", ""] + [f"- {x}" for x in t["notes"]] + [""]
    return "\n".join(out)


CSS = """
:root{--bg:#16161d;--panel:#1e1e28;--ink:#e8e8f0;--mute:#9a9ab0;--acc:#8ab4ff;--pur:#8a3ffc;--red:#e84a27;--blue:#4d8dff;--crim:#b82a2a}
*{box-sizing:border-box}html{scroll-behavior:smooth}
body{margin:0;background:var(--bg);color:var(--ink);font:17px/1.6 system-ui,-apple-system,Segoe UI,Roboto,sans-serif}
a{color:var(--acc)}
header{position:sticky;top:0;background:rgba(22,22,29,.92);backdrop-filter:blur(8px);border-bottom:1px solid #2c2c3a;z-index:5}
.bar{max-width:1040px;margin:0 auto;padding:10px 20px;display:flex;gap:18px;align-items:center;flex-wrap:wrap}
.logo{font-weight:700;letter-spacing:.08em;color:var(--acc);text-decoration:none}
nav a{color:var(--mute);text-decoration:none;margin-right:14px;font-size:15px}nav a:hover{color:var(--ink)}
.langs{margin-left:auto;display:flex;gap:6px;flex-wrap:wrap}
.langs a{font-size:13px;padding:2px 8px;border:1px solid #33334a;border-radius:12px;color:var(--mute);text-decoration:none}
.langs a.on{background:var(--pur);border-color:var(--pur);color:#fff}
main{max-width:1040px;margin:0 auto;padding:0 20px 60px}
.hero{padding:56px 0 24px;display:grid;grid-template-columns:1.05fr 1fr;gap:36px;align-items:center}
@media(max-width:820px){.hero{grid-template-columns:1fr}}
h1{font-size:56px;line-height:1.05;margin:0 0 12px;letter-spacing:.04em}
.lead{font-size:20px;color:#cfd3ea;margin:0 0 14px}.honest{color:var(--mute);margin:0 0 22px}
.btn{display:inline-block;background:var(--pur);color:#fff;padding:12px 22px;border-radius:8px;text-decoration:none;font-weight:600}
h2{font-size:28px;margin:56px 0 14px;border-bottom:1px solid #2c2c3a;padding-bottom:8px}
ul{padding-left:20px}li{margin:8px 0}
pre{background:var(--panel);border:1px solid #2c2c3a;border-radius:8px;padding:14px 16px;overflow:auto;font-size:14px}
.warn{border-left:4px solid var(--red);background:var(--panel);padding:12px 16px;border-radius:0 8px 8px 0;color:#dcdcee}
footer{border-top:1px solid #2c2c3a;color:var(--mute);text-align:center;padding:26px 20px;font-size:14px}
svg{width:100%;height:auto;background:var(--panel);border:1px solid #2c2c3a;border-radius:12px}
"""

HERO_SVG = """<svg viewBox="0 0 520 330" role="img" aria-label="CUANTOR Composer">
<g font-family="system-ui,sans-serif" font-size="13" fill="#bfc3dc">
<text x="14" y="52">q[0]</text><text x="14" y="94">q[1]</text><text x="14" y="136">q[2]</text><text x="14" y="178">q[3]</text></g>
<g stroke="#5f6378" stroke-width="1.6"><line x1="52" x2="450" y1="48" y2="48"/><line x1="52" x2="450" y1="90" y2="90"/><line x1="52" x2="450" y1="132" y2="132"/><line x1="52" x2="450" y1="174" y2="174"/></g>
<rect x="80" y="30" width="36" height="36" rx="4" fill="#e84a27"/><text x="98" y="54" fill="#fff" font-size="16" text-anchor="middle" font-family="system-ui">H</text>
<line x1="190" x2="190" y1="48" y2="90" stroke="#9c8cff" stroke-width="2"/><circle cx="190" cy="48" r="6" fill="#9c8cff"/>
<circle cx="190" cy="90" r="13" fill="#8a3ffc"/><path d="M183 90h14M190 83v14" stroke="#fff" stroke-width="2"/>
<rect x="262" y="114" width="36" height="36" rx="4" fill="#4d8dff"/><text x="280" y="138" fill="#fff" font-size="15" text-anchor="middle" font-family="system-ui">T</text>
<rect x="334" y="156" width="36" height="36" rx="4" fill="#b82a2a"/><text x="352" y="180" fill="#fff" font-size="13" text-anchor="middle" font-family="system-ui">√X</text>
<g fill="#16161d" stroke="#d0d0e0" stroke-width="1.5"><circle cx="486" cy="48" r="13"/><circle cx="486" cy="90" r="13"/><circle cx="486" cy="132" r="13"/><circle cx="486" cy="174" r="13"/></g>
<path d="M473 48a13 13 0 0 0 26 0z" fill="#4d8dff" transform="translate(0 0)"/><path d="M473 90a13 13 0 0 0 26 0z" fill="#4d8dff"/>
<g stroke="#3c3c50"><line x1="40" x2="500" y1="230" y2="230"/></g>
<rect x="150" y="238" width="30" height="64" fill="#339aff"/><rect x="330" y="238" width="30" height="64" fill="#339aff"/>
<g font-family="ui-monospace,monospace" font-size="12" fill="#9a9ab0" text-anchor="middle"><text x="165" y="318">00</text><text x="345" y="318">11</text></g>
<g font-family="system-ui" font-size="12" fill="#d8dcf2" text-anchor="middle"><text x="165" y="234">50</text><text x="345" y="234">50</text></g>
<circle cx="430" cy="270" r="30" fill="none" stroke="#6a6a88"/>
<g stroke-width="5" fill="none"><path d="M430 240a30 30 0 0 1 30 30" stroke="#d94bd6"/><path d="M460 270a30 30 0 0 1-30 30" stroke="#f0c24b"/><path d="M430 300a30 30 0 0 1-30-30" stroke="#5be08a"/><path d="M400 270a30 30 0 0 1 30-30" stroke="#6a6cff"/></g>
</svg>"""


def page(l):
    t = T[l]
    e = html.escape
    nav_ids = ["features", "limits", "start", "ibm", "quality", "download"]
    nav = "".join(f'<a href="#{i}">{e(n)}</a>' for i, n in zip(nav_ids, t["nav"]))
    langs = "".join(f'<a href="../{x}/" hreflang="{x}" lang="{x}"{" class=on aria-current=true" if x == l else ""}>{NAMES[x]}</a>' for x in LANGS)
    alts = "".join(f'<link rel="alternate" hreflang="{x}" href="../{x}/">' for x in LANGS)
    return f"""<!doctype html>
<html lang="{l}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>CUANTOR — {e(t['tagline'][:70])}</title>
<meta name="description" content="{e(t['tagline'])}">
{alts}<link rel="alternate" hreflang="x-default" href="../en/">
<style>{CSS}</style></head><body>
<header><div class="bar"><a class="logo" href="#top">CUANTOR</a><nav>{nav}</nav><div class="langs">{langs}</div></div></header>
<main id="top">
<section class="hero"><div><h1>CUANTOR</h1><p class="lead">{e(t['tagline'])}</p><p class="honest">{e(t['honest'])}</p><a class="btn dl" href="#download">{e(t['dl_btn'])}</a></div>{HERO_SVG}</section>
<h2 id="features">{e(t['h_feat'])}</h2><ul>{"".join(f"<li>{e(x)}</li>" for x in t['feat'])}</ul>
<h2 id="limits">{e(t['h_lim'])}</h2><p>{e(t['lim'])}</p>
<h2 id="start">{e(t['h_start'])}</h2><p>{e(t['start'])}</p><pre>{e(CODE)}</pre>
<h2 id="ibm">{e(t['h_ibm'])}</h2><ul>{"".join(f"<li>{e(x)}</li>" for x in t['ibm'])}</ul><p class="warn">{e(t['ibm_warn'])}</p>
<h2 id="quality">{e(t['h_q'])}</h2><p>{e(t['q'])}</p>
<h3>{e(t['cl_title'])} — {VER}</h3><p><b>{e(t['cl_added'])}</b></p><ul>{"".join(f"<li>{e(x)}</li>" for x in t['added'])}</ul><p><b>{e(t['cl_fixed'])}</b></p><ul>{"".join(f"<li>{e(x)}</li>" for x in t['fixed'])}</ul>
<h2 id="download">{e(t['h_dl'])}</h2><p>{e(t['dl'])}</p><p><a class="btn dl" id="rel" href="#download">{e(t['dl_btn'])}</a></p>
<h2>{e(t['h_lic'])}</h2><p>{e(t['lic'])}</p>
</main><footer>{e(t['foot'])}</footer>
<script>
(function(){{var h=location.hostname,p=location.pathname.split('/').filter(Boolean);
if(h.endsWith('.github.io')&&p.length){{var u='https://github.com/'+h.split('.')[0]+'/'+p[0]+'/releases/latest';
document.querySelectorAll('a.dl').forEach(function(a){{if(a.id==='rel'||a.classList.contains('btn')){{a.href=u}}}});}}}})();
</script></body></html>
"""


def root_index():
    links = "".join(f'<li><a href="{x}/" hreflang="{x}">{NAMES[x]}</a></li>' for x in LANGS)
    codes = "[" + ",".join(f'"{x}"' for x in LANGS) + "]"
    return f"""<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>CUANTOR</title>
<style>body{{background:#16161d;color:#e8e8f0;font:18px system-ui;max-width:520px;margin:12vh auto;padding:0 20px}}a{{color:#8ab4ff}}</style>
<script>(function(){{var c={codes},n=(navigator.language||'en').slice(0,2).toLowerCase();location.replace((c.indexOf(n)>=0?n:'en')+'/');}})();</script></head>
<body><h1>CUANTOR</h1><ul>{links}</ul></body></html>
"""


def write(path, text):
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def main():
    for l in LANGS:
        write("README.md" if l == "en" else f"docs/README.{l}.md", md_readme(l))
        write("CHANGELOG.md" if l == "en" else f"docs/CHANGELOG.{l}.md", md_changelog(l))
        write(f"site/{l}/index.html", page(l))
    write("site/index.html", root_index())
    write("site/.nojekyll", "")
    print("ok", len(LANGS), "limbi, teste:", TESTS)


main()
