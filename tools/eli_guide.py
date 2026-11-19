# -*- coding: utf-8 -*-
"""Conținutul ghidului de utilizare: ELI10 (explicat pentru un copil de 10 ani) + rezumat ELI5 după fiecare capitol.
Blocuri: ("p", text) · ("ul", [..]) · ("ol", [..]) · ("code", text) · ("note", text)."""

QASM = """OPENQASM 3.0;
include "stdgates.inc";
qubit[2] q;
bit[2] c;
h q[0];
cx q[0], q[1];
c = measure q;"""

CLI = """cuantor examples
cuantor example Bell --shots 1000
cuantor run circuit.qasm --shots 1000 --seed 7
cuantor state circuit.qasm
cuantor convert circuit.qasm --to qiskit
cuantor ghz 1000 --shots 8"""

IBM = """export IBM_QUANTUM_API_KEY="..."
export IBM_QUANTUM_CRN="..."
export IBM_QUANTUM_BACKEND="..."
cuantor ibm backends
cuantor ibm run circuit.qasm --backend NAME --dry
cuantor ibm run circuit.qasm --backend NAME --shots 1000"""

LABELS = {
    "en": dict(title="CUANTOR — User guide", sub="Explained like you are 10 (ELI10), with a tiny summary for a 5-year-old (ELI5) after every chapter", eli5="ELI5 — in one breath", toc="Contents", foot="Built with Claude · Simulation only: no quantum advantage is claimed."),
    "ro": dict(title="CUANTOR — Ghid de utilizare", sub="Explicat ca pentru un copil de 10 ani (ELI10), cu un rezumat pentru un copil de 5 ani (ELI5) după fiecare capitol", eli5="ELI5 — pe scurt", toc="Cuprins", foot="Realizat cu Claude · Doar simulare: nu se pretinde niciun avantaj cuantic."),
    "fr": dict(title="CUANTOR — Guide d’utilisation", sub="Expliqué comme à un enfant de 10 ans (ELI10), avec un mini-résumé pour un enfant de 5 ans (ELI5) après chaque chapitre", eli5="ELI5 — en une phrase", toc="Sommaire", foot="Réalisé avec Claude · Simulation uniquement : aucun avantage quantique n’est revendiqué."),
    "de": dict(title="CUANTOR — Benutzerhandbuch", sub="Erklärt wie für ein 10-jähriges Kind (ELI10), mit einer Mini-Zusammenfassung für ein 5-jähriges Kind (ELI5) nach jedem Kapitel", eli5="ELI5 — in einem Atemzug", toc="Inhalt", foot="Mit Claude gebaut · Nur Simulation: Es wird kein Quantenvorteil behauptet."),
    "es": dict(title="CUANTOR — Guía de uso", sub="Explicado como para un niño de 10 años (ELI10), con un mini resumen para un niño de 5 años (ELI5) después de cada capítulo", eli5="ELI5 — en una frase", toc="Contenido", foot="Hecho con Claude · Solo simulación: no se afirma ninguna ventaja cuántica."),
    "it": dict(title="CUANTOR — Guida all’uso", sub="Spiegato come a un bambino di 10 anni (ELI10), con un mini riassunto per un bambino di 5 anni (ELI5) dopo ogni capitolo", eli5="ELI5 — in una frase", toc="Indice", foot="Realizzato con Claude · Solo simulazione: nessun vantaggio quantistico dichiarato."),
    "pt": dict(title="CUANTOR — Guia de utilização", sub="Explicado como para uma criança de 10 anos (ELI10), com um mini resumo para uma criança de 5 anos (ELI5) depois de cada capítulo", eli5="ELI5 — numa frase", toc="Índice", foot="Feito com Claude · Apenas simulação: nenhuma vantagem quântica é alegada."),
    "hu": dict(title="CUANTOR — Felhasználói útmutató", sub="Úgy elmagyarázva, mintha 10 évesnek szólna (ELI10), minden fejezet végén egy 5 éves szintű összefoglalóval (ELI5)", eli5="ELI5 — egy szuszra", toc="Tartalom", foot="Claude-dal készült · Csak szimuláció: kvantumelőnyt nem állítunk."),
}

G = {}

G["en"] = [
 ("What is CUANTOR?", [
  ("p", "A normal computer thinks with bits. A bit is like a light switch: off (0) or on (1). A quantum bit, or qubit, is more like a coin spinning in the air: until it lands, it is a little bit heads and a little bit tails at the same time."),
  ("p", "CUANTOR is a pretend quantum computer that lives inside your computer. It does the quantum maths exactly right, but it is a simulation, not a real quantum machine. That is perfect for learning, playing and checking ideas."),
 ], "CUANTOR is a toy quantum computer on your screen. Instead of switches, it has spinning coins."),
 ("Install it", [
  ("ol", ["Open the Releases page (the Download button on the website).", "Download the file for your computer: Windows .zip, Linux .tar.gz, Mac with an Apple chip: macos-arm64, older Mac with an Intel chip: macos-x86_64.", "Unpack it anywhere you like. There is nothing to install.", "Double-click cuantor-gui. That is the window program. The file called cuantor is the text-mode tool (chapter 9).", "Optional: the .sha256 file lets you check that the download is not damaged."]),
  ("note", "The programs are not digitally signed, so Windows or macOS may show a warning the first time. This is normal for a free hobby project. Only open files you downloaded from the official release page."),
 ], "Download, unpack, double-click cuantor-gui."),
 ("A tour of the window", [
  ("ul", ["Left: the palette. Colored blocks, called gates. They are your tools.", "Middle: the circuit. Each horizontal line is one qubit (q[0], q[1], …). Time goes from left to right, like reading a sentence.", "Right: the answers, drawn as pictures. Probabilities (bars), the Q-sphere (a ball; the colors tell the “phase”, a hidden twist) and the state vector (a table of numbers).", "Code panel: the same circuit written as text, live. You can switch between OpenQASM 3.0, OpenQASM 2.0 and Qiskit.", "Top: menus File, Edit, View and a language box with 8 languages."]),
 ], "Gates on the left, your circuit in the middle, answers on the right."),
 ("Your first circuit: two magic coins", [
  ("ol", ["Set Qubits to 2.", "Pick H (Hadamard) in the palette and tap the first wire q[0], in the first column.", "Pick CNOT. Tap q[0] first (the control), then q[1] (the target).", "Look at Probabilities: two bars, 00 and 11, about 50% each.", "Press Run. CUANTOR repeats the experiment 1000 times. You only ever see 00 or 11, never 01 or 10."]),
  ("p", "Why? H makes the first coin spin. CNOT ties the second coin to the first, so they always land the same way. Scientists call that entanglement. You can also load it from File → Examples (the example titles are still in Romanian: “Încâlcire: starea Bell”)."),
  ("p", "The same circuit as text, if you want to type or save it:"),
  ("code", QASM),
 ], "Two magic coins that always land the same."),
 ("The gates, one by one", [
  ("ul", ["H: starts the coin spinning (half heads, half tails).", "X (NOT): flips heads and tails. Y and Z: other flips. Z changes only the hidden twist (phase).", "S, T and their daggers (S†, T†), √X: small, careful twists.", "P(φ), Rx, Ry, Rz: turn by an angle that you choose. Type it in radians. You may write expressions such as pi/4 or 2*pi/3. To change it, select the gate (Sel) and edit “Angle (rad)”.", "CNOT, CZ, CY, CH: “if the first qubit is 1, do the trick on the second”. CP and controlled Rz: the same with an angle.", "SWAP: swaps two qubits. Toffoli: two controls and one target. Fredkin: one control, swaps two.", "Measure: you look at the coin. It stops spinning and becomes 0 or 1.", "Eraser: removes a gate. Edit → Undo / Redo fix mistakes. Inspect: hover over a gate to see its matrix."]),
 ], "Gates are tiny tricks you play on the coins."),
 ("Running it, and making mistakes on purpose", [
  ("p", "Shots is how many times the experiment is repeated (1 to 1,000,000). Quantum answers are chances, so you repeat many times and count, like throwing a die 1000 times."),
  ("p", "Real quantum computers are clumsy. The sliders add pretend clumsiness (noise): 1-qubit depolarizing, 2-qubit depolarizing and readout error. Leave them at 0 for a perfect machine. The seed makes the “random” results repeatable: same seed, same answers (in the command line, --seed)."),
 ], "Run = flip many times and count. Noise = clumsy fingers."),
 ("How big can it be?", [
  ("p", "Every extra qubit doubles the work, like a notebook where each new qubit doubles the number of pages. 24 qubits need about 256 MiB of memory; 28 qubits need about 4 GiB; 40 would need a whole data center."),
  ("p", "That is why CUANTOR has three engines. State vector: keeps every number, exact, up to 28 qubits. Stabilizer: only for special gates (H, S, CNOT, …), but super fast, up to 4096 qubits. Sparse: for circuits with only a few branches. “Auto” picks one for you."),
  ("p", "Example: a 1000-qubit GHZ state (a big group of coins that all agree) runs instantly. A random 1000-qubit circuit is impossible for any normal computer, and no program can change that."),
 ], "More qubits means much, much more work. Simple patterns can still be huge."),
 ("The code panel and files", [
  ("ul", ["The code panel always shows your circuit as text. Edit the text, then press “Apply to circuit” and the picture changes.", "File → Open .qasm and Save .qasm (OpenQASM 3) load and save circuits. OpenQASM 2.0 and 3.0 can be imported.", "Some tricks cannot be drawn in the editor: reset and if (circuits that decide while running, “dynamic circuits”). Use the command line for those."]),
 ], "A circuit is a picture and a text at the same time."),
 ("The command line", [
  ("p", "The text-mode program cuantor does the same maths without a window. Handy for scripts and for very big circuits."),
  ("code", CLI),
  ("p", "Choose the language with --lang xx or the variable CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Add --backend sv|stab|sparse to choose the engine, and --depol1, --depol2, --readout for noise."),
 ], "Same power, no window."),
 ("Talking to IBM Quantum", [
  ("p", "Open in IBM Composer: the button saves your circuit as a .qasm file. In IBM’s Composer (quantum.cloud.ibm.com) choose File → Upload .qasm file."),
  ("p", "Run on a real IBM machine: you need your own IBM Quantum account. Give CUANTOR your key and instance (CRN) through variables, or through the file ~/.config/cuantor/ibm.json (with api_key, crn and backend). The key stays on your computer and is only sent to IBM."),
  ("code", IBM),
  ("note", "Honest warning: this link has been tested only against a pretend server, never on a real IBM machine. It may need small fixes. Real machines have queues and plan limits, and circuits with reset or if cannot be sent."),
 ], "You can send circuits to a real IBM quantum computer if you have an account. We have not tried it on the real thing yet."),
 ("If something goes wrong", [
  ("ul", ["Warning when opening: see chapter 2. The programs are not signed.", "Slow or out of memory: use fewer qubits, or an engine that fits (stabilizer for simple gates). CUANTOR refuses sizes that would crash your computer.", "“Invalid angle”: write a number such as 0.785 or an expression such as pi/4.", "Wrong language: use the language box, or set CUANTOR_LANG.", "IBM errors: check the key, the CRN and the backend name (cuantor ibm backends). Nothing is printed about the key, on purpose."]),
 ], "Stuck? Use fewer qubits and simpler numbers."),
]

G["ro"] = [
 ("Ce este CUANTOR?", [
  ("p", "Un calculator obișnuit gândește cu biți. Un bit e ca un întrerupător: stins (0) sau aprins (1). Un bit cuantic, sau qubit, seamănă mai mult cu o monedă care se învârte în aer: până aterizează, e puțin cap și puțin pajură în același timp."),
  ("p", "CUANTOR este un calculator cuantic de jucărie, care trăiește în calculatorul tău. Face matematica cuantică exact, dar este o simulare, nu o mașină cuantică reală. E perfect pentru a învăța, a te juca și a verifica idei."),
 ], "CUANTOR e un calculator cuantic de jucărie pe ecran. În loc de întrerupătoare are monede care se învârt."),
 ("Instalare", [
  ("ol", ["Deschide pagina Releases (butonul Descarcă de pe site).", "Descarcă fișierul pentru calculatorul tău: Windows .zip, Linux .tar.gz, Mac cu cip Apple: macos-arm64, Mac mai vechi cu procesor Intel: macos-x86_64.", "Dezarhivează-l oriunde. Nu e nimic de instalat.", "Dă dublu clic pe cuantor-gui. Este programul cu fereastră. Fișierul numit cuantor este instrumentul în mod text (capitolul 9).", "Opțional: fișierul .sha256 te lasă să verifici că descărcarea nu e stricată."]),
  ("note", "Programele nu sunt semnate digital, așa că Windows sau macOS pot afișa un avertisment prima dată. E normal pentru un proiect gratuit, de pasiune. Deschide doar fișiere descărcate de pe pagina oficială de release."),
 ], "Descarci, dezarhivezi, dai dublu clic pe cuantor-gui."),
 ("Plimbare prin fereastră", [
  ("ul", ["Stânga: paleta. Blocuri colorate, numite porți. Sunt uneltele tale.", "Mijloc: circuitul. Fiecare linie orizontală e un qubit (q[0], q[1], …). Timpul merge de la stânga la dreapta, ca la citit.", "Dreapta: răspunsurile, desenate. Probabilități (bare), Q-sfera (o minge; culorile arată „faza”, o răsucire ascunsă) și vectorul de stare (un tabel de numere).", "Panoul de cod: același circuit scris ca text, în direct. Poți alege OpenQASM 3.0, OpenQASM 2.0 sau Qiskit.", "Sus: meniurile Fișier, Editare, Vizualizare și o casetă de limbă cu 8 limbi."]),
 ], "Porțile în stânga, circuitul la mijloc, răspunsurile în dreapta."),
 ("Primul tău circuit: două monede magice", [
  ("ol", ["Setează Qubiți la 2.", "Alege H (Hadamard) din paletă și atinge primul fir q[0], în prima coloană.", "Alege CNOT. Atinge întâi q[0] (controlul), apoi q[1] (ținta).", "Uită-te la Probabilități: două bare, 00 și 11, cam 50% fiecare.", "Apasă Rulează. CUANTOR repetă experimentul de 1000 de ori. Vezi mereu doar 00 sau 11, niciodată 01 sau 10."]),
  ("p", "De ce? H pune prima monedă să se învârtă. CNOT leagă a doua monedă de prima, deci aterizează mereu la fel. Oamenii de știință numesc asta încâlcire (entanglement). Îl poți încărca și din Fișier → Exemple („Încâlcire: starea Bell”)."),
  ("p", "Același circuit ca text, dacă vrei să-l scrii sau să-l salvezi:"),
  ("code", QASM),
 ], "Două monede magice care aterizează mereu la fel."),
 ("Porțile, pe rând", [
  ("ul", ["H: pune moneda să se învârtă (jumătate cap, jumătate pajură).", "X (NOT): schimbă capul cu pajura. Y și Z: alte întoarceri. Z schimbă doar răsucirea ascunsă (faza).", "S, T și variantele lor cu pumnal (S†, T†), √X: răsuciri mici și atente.", "P(φ), Rx, Ry, Rz: se rotesc cu un unghi ales de tine. Scrie-l în radiani. Poți folosi expresii ca pi/4 sau 2*pi/3. Ca să-l schimbi, selectează poarta (Sel) și editează „Unghi (rad)”.", "CNOT, CZ, CY, CH: „dacă primul qubit e 1, fă trucul pe al doilea”. CP și Rz controlat: la fel, cu unghi.", "SWAP: schimbă două qubiți între ei. Toffoli: două controale și o țintă. Fredkin: un control, schimbă doi.", "Măsurare: te uiți la monedă. Se oprește din învârtit și devine 0 sau 1.", "Radiera (Gumă): șterge o poartă. Editare → Anulează / Refă repară greșelile. Inspectare: ține cursorul pe o poartă să-i vezi matricea."]),
 ], "Porțile sunt trucuri mici pe care le faci monedelor."),
 ("Rularea și greșelile făcute intenționat", [
  ("p", "Măsurători este de câte ori se repetă experimentul (de la 1 la 1.000.000). Răspunsurile cuantice sunt șanse, așa că repeți de multe ori și numeri, ca la aruncatul unui zar de 1000 de ori."),
  ("p", "Calculatoarele cuantice reale sunt stângace. Cursoarele adaugă stângăcie prefăcută (zgomot): depolarizare 1q, depolarizare 2q și eroare de citire. Lasă-le la 0 pentru o mașină perfectă. Sămânța (seed) face rezultatele „aleatoare” repetabile: aceeași sămânță, aceleași răspunsuri (în linia de comandă, --seed)."),
 ], "Rulează = arunci de multe ori și numeri. Zgomot = degete stângace."),
 ("Cât de mare poate fi?", [
  ("p", "Fiecare qubit în plus dublează munca, ca un caiet în care fiecare qubit nou dublează numărul de pagini. 24 de qubiți cer cam 256 MiB de memorie; 28 cer cam 4 GiB; 40 ar cere un centru de date întreg."),
  ("p", "De aceea CUANTOR are trei motoare. Vector de stare: ține minte fiecare număr, exact, până la 28 de qubiți. Stabilizator: doar pentru porți speciale (H, S, CNOT, …), dar foarte rapid, până la 4096 de qubiți. Rar (sparse): pentru circuite cu doar câteva ramuri. „Auto” alege unul pentru tine."),
  ("p", "Exemplu: o stare GHZ pe 1000 de qubiți (un grup mare de monede care se pun de acord) merge instant. Un circuit oarecare pe 1000 de qubiți e imposibil pentru orice calculator obișnuit și niciun program nu poate schimba asta."),
 ], "Mai mulți qubiți înseamnă mult, mult mai multă muncă. Tiparele simple pot rămâne uriașe."),
 ("Panoul de cod și fișierele", [
  ("ul", ["Panoul de cod arată mereu circuitul ca text. Editezi textul, apeși „Aplică în circuit” și imaginea se schimbă.", "Fișier → Deschide .qasm și Salvează .qasm (OpenQASM 3) încarcă și salvează circuite. Se pot importa OpenQASM 2.0 și 3.0.", "Unele trucuri nu pot fi desenate în editor: reset și if (circuite care decid din mers, „circuite dinamice”). Pentru ele folosește linia de comandă."]),
 ], "Un circuit e și imagine, și text, în același timp."),
 ("Linia de comandă", [
  ("p", "Programul în mod text cuantor face aceeași matematică fără fereastră. E util pentru scripturi și pentru circuite foarte mari."),
  ("code", CLI),
  ("p", "Alege limba cu --lang xx sau cu variabila CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Adaugă --backend sv|stab|sparse pentru motor și --depol1, --depol2, --readout pentru zgomot."),
 ], "Aceeași putere, fără fereastră."),
 ("Legătura cu IBM Quantum", [
  ("p", "Deschide în IBM Composer: butonul salvează circuitul ca fișier .qasm. În Composer-ul IBM (quantum.cloud.ibm.com) alege File → Upload .qasm file."),
  ("p", "Rulare pe o mașină IBM reală: ai nevoie de propriul cont IBM Quantum. Dă-i lui CUANTOR cheia și instanța (CRN) prin variabile sau prin fișierul ~/.config/cuantor/ibm.json (cu api_key, crn și backend). Cheia rămâne pe calculatorul tău și se trimite doar la IBM."),
  ("code", IBM),
  ("note", "Avertisment sincer: această legătură a fost testată doar cu un server simulat, niciodată pe o mașină IBM reală. Poate avea nevoie de mici corecturi. Mașinile reale au cozi și limite de plan, iar circuitele cu reset sau if nu pot fi trimise."),
 ], "Poți trimite circuite la un calculator cuantic IBM real dacă ai cont. Încă nu am încercat pe cel real."),
 ("Dacă ceva nu merge", [
  ("ul", ["Avertisment la deschidere: vezi capitolul 2. Programele nu sunt semnate.", "Lent sau fără memorie: folosește mai puțini qubiți sau un motor potrivit (stabilizator pentru porți simple). CUANTOR refuză dimensiunile care ți-ar bloca calculatorul.", "„Unghi invalid”: scrie un număr ca 0,785 (cu punct: 0.785) sau o expresie ca pi/4.", "Limbă greșită: folosește caseta de limbă sau setează CUANTOR_LANG.", "Erori IBM: verifică cheia, CRN-ul și numele backend-ului (cuantor ibm backends). Despre cheie nu se afișează nimic, intenționat."]),
 ], "Te-ai blocat? Mai puțini qubiți și numere mai simple."),
]

G["fr"] = [
 ("Qu’est-ce que CUANTOR ?", [
  ("p", "Un ordinateur normal pense avec des bits. Un bit, c’est comme un interrupteur : éteint (0) ou allumé (1). Un bit quantique, ou qubit, ressemble plutôt à une pièce qui tourne en l’air : tant qu’elle n’est pas retombée, elle est un peu pile et un peu face en même temps."),
  ("p", "CUANTOR est un ordinateur quantique pour de faux, qui vit dans ton ordinateur. Il fait les calculs quantiques exactement, mais c’est une simulation, pas une vraie machine quantique. Parfait pour apprendre, jouer et vérifier des idées."),
 ], "CUANTOR est un ordinateur quantique jouet sur ton écran. Au lieu d’interrupteurs, il a des pièces qui tournent."),
 ("Installation", [
  ("ol", ["Ouvre la page Releases (le bouton Télécharger du site).", "Télécharge le fichier de ton ordinateur : Windows .zip, Linux .tar.gz, Mac à puce Apple : macos-arm64, Mac plus ancien à processeur Intel : macos-x86_64.", "Décompresse-le où tu veux. Rien à installer.", "Double-clique sur cuantor-gui. C’est le programme à fenêtre. Le fichier nommé cuantor est l’outil en mode texte (chapitre 9).", "Facultatif : le fichier .sha256 permet de vérifier que le téléchargement n’est pas abîmé."]),
  ("note", "Les programmes ne sont pas signés numériquement ; Windows ou macOS peuvent donc afficher un avertissement la première fois. C’est normal pour un projet libre fait par passion. N’ouvre que des fichiers téléchargés depuis la page officielle des releases."),
 ], "Télécharge, décompresse, double-clique sur cuantor-gui."),
 ("Visite de la fenêtre", [
  ("ul", ["À gauche : la palette. Des blocs colorés, appelés portes. Ce sont tes outils.", "Au milieu : le circuit. Chaque ligne horizontale est un qubit (q[0], q[1], …). Le temps va de gauche à droite, comme quand on lit.", "À droite : les réponses, en images. Probabilités (barres), sphère de Q (une boule ; les couleurs indiquent la « phase », une torsion cachée) et vecteur d’état (un tableau de nombres).", "Panneau de code : le même circuit écrit en texte, en direct. Au choix : OpenQASM 3.0, OpenQASM 2.0 ou Qiskit.", "En haut : les menus Fichier, Édition, Affichage et une liste de 8 langues."]),
 ], "Les portes à gauche, ton circuit au milieu, les réponses à droite."),
 ("Ton premier circuit : deux pièces magiques", [
  ("ol", ["Mets Qubits à 2.", "Choisis H (Hadamard) dans la palette et touche le premier fil q[0], dans la première colonne.", "Choisis CNOT. Touche d’abord q[0] (le contrôle), puis q[1] (la cible).", "Regarde Probabilités : deux barres, 00 et 11, environ 50 % chacune.", "Appuie sur Exécuter. CUANTOR répète l’expérience 1000 fois. Tu ne vois jamais que 00 ou 11, jamais 01 ni 10."]),
  ("p", "Pourquoi ? H fait tourner la première pièce. CNOT attache la deuxième à la première : elles retombent toujours pareil. Les scientifiques appellent ça l’intrication. Tu peux aussi la charger par Fichier → Exemples (les titres des exemples sont encore en roumain : « Încâlcire: starea Bell »)."),
  ("p", "Le même circuit en texte, si tu veux le taper ou l’enregistrer :"),
  ("code", QASM),
 ], "Deux pièces magiques qui retombent toujours pareil."),
 ("Les portes, une par une", [
  ("ul", ["H : fait tourner la pièce (moitié pile, moitié face).", "X (NOT) : échange pile et face. Y et Z : d’autres retournements. Z ne change que la torsion cachée (la phase).", "S, T et leurs versions « dague » (S†, T†), √X : de petites torsions soigneuses.", "P(φ), Rx, Ry, Rz : tournent d’un angle que tu choisis, en radians. Tu peux écrire des expressions comme pi/4 ou 2*pi/3. Pour le changer, sélectionne la porte (Sél) et modifie « Angle (rad) ».", "CNOT, CZ, CY, CH : « si le premier qubit vaut 1, fais le tour sur le second ». CP et Rz contrôlé : pareil, avec un angle.", "SWAP : échange deux qubits. Toffoli : deux contrôles et une cible. Fredkin : un contrôle, échange deux qubits.", "Mesure : tu regardes la pièce. Elle s’arrête et devient 0 ou 1.", "Gomme : retire une porte. Édition → Annuler / Rétablir réparent les erreurs. Inspecter : survole une porte pour voir sa matrice."]),
 ], "Les portes sont de petits tours que tu joues aux pièces."),
 ("Exécuter, et se tromper exprès", [
  ("p", "Mesures, c’est le nombre de répétitions de l’expérience (de 1 à 1 000 000). Les réponses quantiques sont des chances ; on répète donc beaucoup et on compte, comme lancer un dé 1000 fois."),
  ("p", "Les vrais ordinateurs quantiques sont maladroits. Les curseurs ajoutent une maladresse pour de faux (bruit) : dépolarisation 1q, dépolarisation 2q et erreur de lecture. Laisse-les à 0 pour une machine parfaite. La graine (seed) rend le « hasard » reproductible : même graine, mêmes réponses (en ligne de commande, --seed)."),
 ], "Exécuter = lancer plein de fois et compter. Bruit = doigts maladroits."),
 ("Jusqu’où peut-on aller ?", [
  ("p", "Chaque qubit en plus double le travail, comme un cahier où chaque nouveau qubit double le nombre de pages. 24 qubits demandent environ 256 Mio de mémoire ; 28 qubits environ 4 Gio ; 40 demanderaient un centre de données entier."),
  ("p", "C’est pourquoi CUANTOR a trois moteurs. Vecteur d’état : garde tous les nombres, exact, jusqu’à 28 qubits. Stabilisateur : seulement pour des portes spéciales (H, S, CNOT, …), mais ultra rapide, jusqu’à 4096 qubits. Creux (sparse) : pour les circuits à peu de branches. « Auto » choisit pour toi."),
  ("p", "Exemple : un état GHZ de 1000 qubits (un grand groupe de pièces toutes d’accord) tourne instantanément. Un circuit quelconque de 1000 qubits est impossible pour tout ordinateur normal, et aucun programme n’y changera rien."),
 ], "Plus de qubits, beaucoup plus de travail. Les motifs simples peuvent rester énormes."),
 ("Le panneau de code et les fichiers", [
  ("ul", ["Le panneau de code montre toujours ton circuit en texte. Modifie le texte, appuie sur « Appliquer au circuit » et l’image change.", "Fichier → Ouvrir .qasm et Enregistrer .qasm (OpenQASM 3) chargent et enregistrent des circuits. OpenQASM 2.0 et 3.0 s’importent.", "Certains tours ne se dessinent pas dans l’éditeur : reset et if (circuits qui décident en route, « circuits dynamiques »). Utilise la ligne de commande pour eux."]),
 ], "Un circuit est à la fois une image et un texte."),
 ("La ligne de commande", [
  ("p", "Le programme en mode texte cuantor fait les mêmes calculs sans fenêtre. Pratique pour les scripts et les très grands circuits."),
  ("code", CLI),
  ("p", "Choisis la langue avec --lang xx ou la variable CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Ajoute --backend sv|stab|sparse pour le moteur et --depol1, --depol2, --readout pour le bruit."),
 ], "Même puissance, sans fenêtre."),
 ("Parler à IBM Quantum", [
  ("p", "Ouvrir dans IBM Composer : le bouton enregistre ton circuit en fichier .qasm. Dans le Composer d’IBM (quantum.cloud.ibm.com), choisis File → Upload .qasm file."),
  ("p", "Exécuter sur une vraie machine IBM : il faut ton propre compte IBM Quantum. Donne à CUANTOR ta clé et ton instance (CRN) par des variables, ou par le fichier ~/.config/cuantor/ibm.json (avec api_key, crn et backend). La clé reste sur ton ordinateur et n’est envoyée qu’à IBM."),
  ("code", IBM),
  ("note", "Avertissement honnête : ce lien n’a été testé qu’avec un faux serveur, jamais sur une vraie machine IBM. Il peut demander de petites corrections. Les vraies machines ont des files d’attente et des limites de forfait, et les circuits avec reset ou if ne peuvent pas être envoyés."),
 ], "Tu peux envoyer des circuits à un vrai ordinateur quantique IBM si tu as un compte. On ne l’a pas encore essayé en vrai."),
 ("Si quelque chose ne va pas", [
  ("ul", ["Avertissement à l’ouverture : voir le chapitre 2. Les programmes ne sont pas signés.", "Lent ou manque de mémoire : moins de qubits, ou un moteur adapté (stabilisateur pour des portes simples). CUANTOR refuse les tailles qui feraient planter ton ordinateur.", "« Angle invalide » : écris un nombre comme 0.785 ou une expression comme pi/4.", "Mauvaise langue : utilise la liste de langues, ou définis CUANTOR_LANG.", "Erreurs IBM : vérifie la clé, le CRN et le nom du backend (cuantor ibm backends). Rien n’est affiché sur la clé, exprès."]),
 ], "Bloqué ? Moins de qubits et des nombres plus simples."),
]

G["de"] = [
 ("Was ist CUANTOR?", [
  ("p", "Ein normaler Computer denkt mit Bits. Ein Bit ist wie ein Lichtschalter: aus (0) oder an (1). Ein Quantenbit, kurz Qubit, ist eher wie eine Münze, die sich in der Luft dreht: Bis sie landet, ist sie ein bisschen Kopf und ein bisschen Zahl zugleich."),
  ("p", "CUANTOR ist ein Quantencomputer zum So-tun-als-ob, der in deinem Computer lebt. Er rechnet die Quantenmathematik genau richtig, ist aber eine Simulation und keine echte Quantenmaschine. Das ist ideal zum Lernen, Spielen und Ausprobieren von Ideen."),
 ], "CUANTOR ist ein Spielzeug-Quantencomputer auf deinem Bildschirm. Statt Schaltern hat er sich drehende Münzen."),
 ("Installieren", [
  ("ol", ["Öffne die Seite Releases (der Download-Knopf auf der Webseite).", "Lade die Datei für deinen Computer herunter: Windows .zip, Linux .tar.gz, Mac mit Apple-Chip: macos-arm64, älterer Mac mit Intel-Prozessor: macos-x86_64.", "Entpacke sie, wohin du willst. Es gibt nichts zu installieren.", "Doppelklicke auf cuantor-gui. Das ist das Programm mit Fenster. Die Datei namens cuantor ist das Textprogramm (Kapitel 9).", "Optional: Mit der .sha256-Datei prüfst du, ob der Download unbeschädigt ist."]),
  ("note", "Die Programme sind nicht digital signiert, daher zeigen Windows oder macOS beim ersten Start vielleicht eine Warnung. Das ist bei einem kostenlosen Hobbyprojekt normal. Öffne nur Dateien von der offiziellen Release-Seite."),
 ], "Herunterladen, entpacken, cuantor-gui doppelklicken."),
 ("Rundgang durchs Fenster", [
  ("ul", ["Links: die Palette. Bunte Bausteine, Gatter genannt. Sie sind dein Werkzeug.", "Mitte: der Schaltkreis. Jede waagerechte Linie ist ein Qubit (q[0], q[1], …). Die Zeit läuft von links nach rechts, wie beim Lesen.", "Rechts: die Antworten als Bilder. Wahrscheinlichkeiten (Balken), die Q-Sphäre (eine Kugel; die Farben zeigen die „Phase“, eine versteckte Drehung) und der Zustandsvektor (eine Zahlentabelle).", "Code-Fenster: derselbe Schaltkreis als Text, live. Du kannst zwischen OpenQASM 3.0, OpenQASM 2.0 und Qiskit wählen.", "Oben: die Menüs Datei, Bearbeiten, Ansicht und ein Sprachfeld mit 8 Sprachen."]),
 ], "Gatter links, dein Schaltkreis in der Mitte, Antworten rechts."),
 ("Dein erster Schaltkreis: zwei Zaubermünzen", [
  ("ol", ["Stelle Qubits auf 2.", "Wähle H (Hadamard) in der Palette und tippe auf den ersten Draht q[0] in der ersten Spalte.", "Wähle CNOT. Tippe zuerst auf q[0] (die Steuerung), dann auf q[1] (das Ziel).", "Sieh bei Wahrscheinlichkeiten nach: zwei Balken, 00 und 11, je etwa 50 %.", "Drücke Ausführen. CUANTOR wiederholt das Experiment 1000-mal. Du siehst immer nur 00 oder 11, nie 01 oder 10."]),
  ("p", "Warum? H lässt die erste Münze kreiseln. CNOT bindet die zweite an die erste, sodass beide immer gleich landen. Forscher nennen das Verschränkung. Du kannst es auch über Datei → Beispiele laden (die Beispieltitel sind noch rumänisch: „Încâlcire: starea Bell“)."),
  ("p", "Derselbe Schaltkreis als Text, falls du ihn tippen oder speichern willst:"),
  ("code", QASM),
 ], "Zwei Zaubermünzen, die immer gleich landen."),
 ("Die Gatter einzeln", [
  ("ul", ["H: bringt die Münze zum Kreiseln (halb Kopf, halb Zahl).", "X (NOT): tauscht Kopf und Zahl. Y und Z: andere Umdrehungen. Z ändert nur die versteckte Drehung (Phase).", "S, T und ihre Dolch-Versionen (S†, T†), √X: kleine, sorgfältige Drehungen.", "P(φ), Rx, Ry, Rz: drehen um einen Winkel, den du wählst, in Radiant. Du kannst Ausdrücke wie pi/4 oder 2*pi/3 schreiben. Zum Ändern wähle das Gatter (Ausw.) und bearbeite „Winkel (rad)“.", "CNOT, CZ, CY, CH: „Wenn das erste Qubit 1 ist, mach den Trick beim zweiten.“ CP und gesteuertes Rz: genauso, mit Winkel.", "SWAP: tauscht zwei Qubits. Toffoli: zwei Steuerungen und ein Ziel. Fredkin: eine Steuerung, tauscht zwei.", "Messung: Du schaust auf die Münze. Sie hört auf zu kreiseln und wird 0 oder 1.", "Radierer: entfernt ein Gatter. Bearbeiten → Rückgängig / Wiederholen beheben Fehler. Untersuchen: Zeiger über ein Gatter zeigt seine Matrix."]),
 ], "Gatter sind kleine Tricks, die du mit den Münzen machst."),
 ("Ausführen und absichtlich Fehler machen", [
  ("p", "Messungen ist, wie oft das Experiment wiederholt wird (1 bis 1.000.000). Quantenantworten sind Chancen, also wiederholt man oft und zählt, wie wenn man 1000-mal würfelt."),
  ("p", "Echte Quantencomputer sind tollpatschig. Die Regler fügen gespielte Tollpatschigkeit (Rauschen) hinzu: 1-Qubit-Depolarisation, 2-Qubit-Depolarisation und Auslesefehler. Bei 0 hast du eine perfekte Maschine. Der Seed macht den „Zufall“ wiederholbar: gleicher Seed, gleiche Antworten (in der Kommandozeile --seed)."),
 ], "Ausführen = oft werfen und zählen. Rauschen = ungeschickte Finger."),
 ("Wie groß darf es werden?", [
  ("p", "Jedes zusätzliche Qubit verdoppelt die Arbeit, wie ein Heft, in dem jedes neue Qubit die Seitenzahl verdoppelt. 24 Qubits brauchen etwa 256 MiB Speicher, 28 Qubits etwa 4 GiB, 40 bräuchten ein ganzes Rechenzentrum."),
  ("p", "Darum hat CUANTOR drei Motoren. Zustandsvektor: merkt sich jede Zahl, exakt, bis 28 Qubits. Stabilisator: nur für besondere Gatter (H, S, CNOT, …), dafür blitzschnell, bis 4096 Qubits. Sparse: für Schaltkreise mit nur wenigen Zweigen. „Auto“ wählt für dich."),
  ("p", "Beispiel: Ein GHZ-Zustand mit 1000 Qubits (eine große Gruppe Münzen, die sich alle einig sind) läuft sofort. Ein beliebiger Schaltkreis mit 1000 Qubits ist für jeden normalen Computer unmöglich, und kein Programm kann das ändern."),
 ], "Mehr Qubits heißt viel, viel mehr Arbeit. Einfache Muster können trotzdem riesig sein."),
 ("Code-Fenster und Dateien", [
  ("ul", ["Das Code-Fenster zeigt deinen Schaltkreis immer als Text. Ändere den Text, drücke „Auf Schaltkreis anwenden“, und das Bild ändert sich.", "Datei → .qasm öffnen und .qasm speichern (OpenQASM 3) laden und sichern Schaltkreise. OpenQASM 2.0 und 3.0 lassen sich importieren.", "Manche Tricks lassen sich im Editor nicht zeichnen: reset und if (Schaltkreise, die unterwegs entscheiden, „dynamische Schaltkreise“). Dafür nimmst du die Kommandozeile."]),
 ], "Ein Schaltkreis ist Bild und Text zugleich."),
 ("Die Kommandozeile", [
  ("p", "Das Textprogramm cuantor rechnet dasselbe ohne Fenster. Praktisch für Skripte und sehr große Schaltkreise."),
  ("code", CLI),
  ("p", "Wähle die Sprache mit --lang xx oder der Variable CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Mit --backend sv|stab|sparse wählst du den Motor, mit --depol1, --depol2, --readout das Rauschen."),
 ], "Gleiche Kraft, kein Fenster."),
 ("Verbindung zu IBM Quantum", [
  ("p", "In IBM Composer öffnen: Der Knopf speichert deinen Schaltkreis als .qasm-Datei. Im Composer von IBM (quantum.cloud.ibm.com) wähle File → Upload .qasm file."),
  ("p", "Auf einer echten IBM-Maschine ausführen: Du brauchst ein eigenes IBM-Quantum-Konto. Gib CUANTOR deinen Schlüssel und die Instanz (CRN) über Variablen oder die Datei ~/.config/cuantor/ibm.json (mit api_key, crn und backend). Der Schlüssel bleibt auf deinem Computer und geht nur an IBM."),
  ("code", IBM),
  ("note", "Ehrliche Warnung: Diese Verbindung wurde nur gegen einen Schein-Server getestet, nie auf einer echten IBM-Maschine. Sie braucht vielleicht kleine Korrekturen. Echte Maschinen haben Warteschlangen und Tariflimits, und Schaltkreise mit reset oder if lassen sich nicht senden."),
 ], "Mit einem Konto kannst du Schaltkreise an einen echten IBM-Quantencomputer schicken. Am echten haben wir es noch nicht probiert."),
 ("Wenn etwas schiefgeht", [
  ("ul", ["Warnung beim Öffnen: siehe Kapitel 2. Die Programme sind nicht signiert.", "Langsam oder zu wenig Speicher: weniger Qubits oder ein passender Motor (Stabilisator für einfache Gatter). CUANTOR lehnt Größen ab, die deinen Computer abstürzen ließen.", "„Ungültiger Winkel“: schreibe eine Zahl wie 0.785 oder einen Ausdruck wie pi/4.", "Falsche Sprache: nimm das Sprachfeld oder setze CUANTOR_LANG.", "IBM-Fehler: prüfe Schlüssel, CRN und Backend-Namen (cuantor ibm backends). Zum Schlüssel wird absichtlich nichts ausgegeben."]),
 ], "Festgefahren? Weniger Qubits und einfachere Zahlen."),
]

G["es"] = [
 ("¿Qué es CUANTOR?", [
  ("p", "Un ordenador normal piensa con bits. Un bit es como un interruptor: apagado (0) o encendido (1). Un bit cuántico, o qubit, se parece más a una moneda girando en el aire: hasta que cae, es un poco cara y un poco cruz a la vez."),
  ("p", "CUANTOR es un ordenador cuántico de mentira que vive dentro de tu ordenador. Hace las matemáticas cuánticas exactamente, pero es una simulación, no una máquina cuántica real. Es perfecto para aprender, jugar y comprobar ideas."),
 ], "CUANTOR es un ordenador cuántico de juguete en tu pantalla. En vez de interruptores, tiene monedas que giran."),
 ("Instalación", [
  ("ol", ["Abre la página Releases (el botón Descargar de la web).", "Descarga el archivo de tu ordenador: Windows .zip, Linux .tar.gz, Mac con chip Apple: macos-arm64, Mac antiguo con procesador Intel: macos-x86_64.", "Descomprímelo donde quieras. No hay nada que instalar.", "Haz doble clic en cuantor-gui. Es el programa con ventana. El archivo llamado cuantor es la herramienta en modo texto (capítulo 9).", "Opcional: el archivo .sha256 permite comprobar que la descarga no está dañada."]),
  ("note", "Los programas no están firmados digitalmente, así que Windows o macOS pueden mostrar un aviso la primera vez. Es normal en un proyecto gratuito y de afición. Abre solo archivos descargados de la página oficial de releases."),
 ], "Descarga, descomprime, doble clic en cuantor-gui."),
 ("Paseo por la ventana", [
  ("ul", ["Izquierda: la paleta. Bloques de colores, llamados puertas. Son tus herramientas.", "Centro: el circuito. Cada línea horizontal es un qubit (q[0], q[1], …). El tiempo va de izquierda a derecha, como al leer.", "Derecha: las respuestas, dibujadas. Probabilidades (barras), la esfera Q (una bola; los colores indican la «fase», un giro escondido) y el vector de estado (una tabla de números).", "Panel de código: el mismo circuito escrito como texto, en directo. Puedes elegir OpenQASM 3.0, OpenQASM 2.0 o Qiskit.", "Arriba: los menús Archivo, Edición, Ver y una caja de idioma con 8 idiomas."]),
 ], "Puertas a la izquierda, tu circuito en el centro, respuestas a la derecha."),
 ("Tu primer circuito: dos monedas mágicas", [
  ("ol", ["Pon Qubits en 2.", "Elige H (Hadamard) en la paleta y toca el primer hilo q[0], en la primera columna.", "Elige CNOT. Toca primero q[0] (el control) y luego q[1] (el objetivo).", "Mira Probabilidades: dos barras, 00 y 11, de casi 50 % cada una.", "Pulsa Ejecutar. CUANTOR repite el experimento 1000 veces. Solo ves 00 o 11, nunca 01 ni 10."]),
  ("p", "¿Por qué? H pone a girar la primera moneda. CNOT ata la segunda a la primera, así que caen siempre igual. Los científicos lo llaman entrelazamiento. También puedes cargarlo desde Archivo → Ejemplos (los títulos de los ejemplos siguen en rumano: «Încâlcire: starea Bell»)."),
  ("p", "El mismo circuito como texto, si quieres escribirlo o guardarlo:"),
  ("code", QASM),
 ], "Dos monedas mágicas que siempre caen igual."),
 ("Las puertas, una a una", [
  ("ul", ["H: pone la moneda a girar (mitad cara, mitad cruz).", "X (NOT): cambia cara por cruz. Y y Z: otros giros. Z solo cambia el giro escondido (la fase).", "S, T y sus versiones con daga (S†, T†), √X: giros pequeños y cuidadosos.", "P(φ), Rx, Ry, Rz: giran un ángulo que eliges, en radianes. Puedes escribir expresiones como pi/4 o 2*pi/3. Para cambiarlo, selecciona la puerta (Sel) y edita «Ángulo (rad)».", "CNOT, CZ, CY, CH: «si el primer qubit es 1, haz el truco en el segundo». CP y Rz controlada: igual, con ángulo.", "SWAP: intercambia dos qubits. Toffoli: dos controles y un objetivo. Fredkin: un control, intercambia dos.", "Medición: miras la moneda. Deja de girar y se vuelve 0 o 1.", "Goma: borra una puerta. Edición → Deshacer / Rehacer arreglan errores. Inspeccionar: pasa el cursor sobre una puerta para ver su matriz."]),
 ], "Las puertas son truquitos que haces a las monedas."),
 ("Ejecutar y equivocarse a propósito", [
  ("p", "Mediciones es cuántas veces se repite el experimento (de 1 a 1.000.000). Las respuestas cuánticas son probabilidades, así que se repite muchas veces y se cuenta, como tirar un dado 1000 veces."),
  ("p", "Los ordenadores cuánticos reales son torpes. Los deslizadores añaden torpeza de mentira (ruido): despolarización 1q, despolarización 2q y error de lectura. Déjalos en 0 para una máquina perfecta. La semilla (seed) hace repetible el «azar»: misma semilla, mismas respuestas (en la línea de comandos, --seed)."),
 ], "Ejecutar = lanzar muchas veces y contar. Ruido = dedos torpes."),
 ("¿Qué tamaño puede tener?", [
  ("p", "Cada qubit extra duplica el trabajo, como un cuaderno donde cada qubit nuevo duplica las páginas. 24 qubits necesitan unos 256 MiB de memoria; 28 qubits, unos 4 GiB; 40 necesitarían un centro de datos entero."),
  ("p", "Por eso CUANTOR tiene tres motores. Vector de estado: guarda todos los números, exacto, hasta 28 qubits. Estabilizador: solo para puertas especiales (H, S, CNOT, …), pero rapidísimo, hasta 4096 qubits. Disperso (sparse): para circuitos con pocas ramas. «Auto» elige por ti."),
  ("p", "Ejemplo: un estado GHZ de 1000 qubits (un grupo enorme de monedas que se ponen de acuerdo) corre al instante. Un circuito cualquiera de 1000 qubits es imposible para cualquier ordenador normal, y ningún programa puede cambiarlo."),
 ], "Más qubits, mucho, muchísimo más trabajo. Los patrones simples aún pueden ser enormes."),
 ("El panel de código y los archivos", [
  ("ul", ["El panel de código siempre muestra tu circuito como texto. Edita el texto, pulsa «Aplicar al circuito» y el dibujo cambia.", "Archivo → Abrir .qasm y Guardar .qasm (OpenQASM 3) cargan y guardan circuitos. Se pueden importar OpenQASM 2.0 y 3.0.", "Algunos trucos no se pueden dibujar en el editor: reset e if (circuitos que deciden sobre la marcha, «circuitos dinámicos»). Para ellos usa la línea de comandos."]),
 ], "Un circuito es dibujo y texto a la vez."),
 ("La línea de comandos", [
  ("p", "El programa en modo texto cuantor hace las mismas matemáticas sin ventana. Útil para scripts y para circuitos enormes."),
  ("code", CLI),
  ("p", "Elige el idioma con --lang xx o la variable CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Añade --backend sv|stab|sparse para el motor y --depol1, --depol2, --readout para el ruido."),
 ], "El mismo poder, sin ventana."),
 ("Hablar con IBM Quantum", [
  ("p", "Abrir en IBM Composer: el botón guarda tu circuito como archivo .qasm. En el Composer de IBM (quantum.cloud.ibm.com) elige File → Upload .qasm file."),
  ("p", "Ejecutar en una máquina IBM real: necesitas tu propia cuenta de IBM Quantum. Dale a CUANTOR tu clave y tu instancia (CRN) con variables o con el archivo ~/.config/cuantor/ibm.json (con api_key, crn y backend). La clave se queda en tu ordenador y solo se envía a IBM."),
  ("code", IBM),
  ("note", "Aviso sincero: este enlace solo se ha probado contra un servidor de mentira, nunca en una máquina IBM real. Puede necesitar pequeños arreglos. Las máquinas reales tienen colas y límites de plan, y los circuitos con reset o if no se pueden enviar."),
 ], "Puedes enviar circuitos a un ordenador cuántico IBM real si tienes cuenta. Aún no lo hemos probado con el real."),
 ("Si algo sale mal", [
  ("ul", ["Aviso al abrir: mira el capítulo 2. Los programas no están firmados.", "Lento o sin memoria: usa menos qubits o un motor adecuado (estabilizador para puertas simples). CUANTOR rechaza tamaños que bloquearían tu ordenador.", "«Ángulo no válido»: escribe un número como 0.785 o una expresión como pi/4.", "Idioma equivocado: usa la caja de idioma o define CUANTOR_LANG.", "Errores de IBM: revisa la clave, el CRN y el nombre del backend (cuantor ibm backends). Sobre la clave no se muestra nada, a propósito."]),
 ], "¿Atascado? Menos qubits y números más simples."),
]

G["it"] = [
 ("Che cos’è CUANTOR?", [
  ("p", "Un computer normale pensa con i bit. Un bit è come un interruttore: spento (0) o acceso (1). Un bit quantistico, o qubit, somiglia di più a una moneta che gira in aria: finché non atterra, è un po’ testa e un po’ croce nello stesso momento."),
  ("p", "CUANTOR è un computer quantistico per finta che vive dentro il tuo computer. Fa la matematica quantistica esattamente, ma è una simulazione, non una vera macchina quantistica. Perfetto per imparare, giocare e provare idee."),
 ], "CUANTOR è un computer quantistico giocattolo sullo schermo. Invece degli interruttori ha monete che girano."),
 ("Installazione", [
  ("ol", ["Apri la pagina Releases (il pulsante Scarica del sito).", "Scarica il file per il tuo computer: Windows .zip, Linux .tar.gz, Mac con chip Apple: macos-arm64, Mac più vecchio con processore Intel: macos-x86_64.", "Decomprimilo dove vuoi. Non c’è nulla da installare.", "Fai doppio clic su cuantor-gui. È il programma con finestra. Il file chiamato cuantor è lo strumento in modalità testo (capitolo 9).", "Facoltativo: il file .sha256 serve a controllare che il download non sia danneggiato."]),
  ("note", "I programmi non sono firmati digitalmente, quindi Windows o macOS possono mostrare un avviso la prima volta. È normale per un progetto gratuito e fatto per passione. Apri solo file scaricati dalla pagina ufficiale delle release."),
 ], "Scarica, decomprimi, doppio clic su cuantor-gui."),
 ("Giro della finestra", [
  ("ul", ["A sinistra: la tavolozza. Blocchi colorati, chiamati porte. Sono i tuoi attrezzi.", "Al centro: il circuito. Ogni linea orizzontale è un qubit (q[0], q[1], …). Il tempo va da sinistra a destra, come quando si legge.", "A destra: le risposte, disegnate. Probabilità (barre), la sfera Q (una palla; i colori indicano la «fase», una rotazione nascosta) e il vettore di stato (una tabella di numeri).", "Pannello del codice: lo stesso circuito scritto come testo, in tempo reale. Puoi scegliere OpenQASM 3.0, OpenQASM 2.0 o Qiskit.", "In alto: i menu File, Modifica, Vista e una casella della lingua con 8 lingue."]),
 ], "Porte a sinistra, circuito al centro, risposte a destra."),
 ("Il tuo primo circuito: due monete magiche", [
  ("ol", ["Imposta Qubit a 2.", "Scegli H (Hadamard) nella tavolozza e tocca il primo filo q[0], nella prima colonna.", "Scegli CNOT. Tocca prima q[0] (il controllo), poi q[1] (il bersaglio).", "Guarda Probabilità: due barre, 00 e 11, circa 50% ciascuna.", "Premi Esegui. CUANTOR ripete l’esperimento 1000 volte. Vedi sempre e solo 00 o 11, mai 01 o 10."]),
  ("p", "Perché? H fa girare la prima moneta. CNOT lega la seconda alla prima, così atterrano sempre uguali. Gli scienziati lo chiamano entanglement. Puoi caricarlo anche da File → Esempi (i titoli degli esempi sono ancora in rumeno: «Încâlcire: starea Bell»)."),
  ("p", "Lo stesso circuito come testo, se vuoi scriverlo o salvarlo:"),
  ("code", QASM),
 ], "Due monete magiche che atterrano sempre uguali."),
 ("Le porte, una per una", [
  ("ul", ["H: fa girare la moneta (metà testa, metà croce).", "X (NOT): scambia testa e croce. Y e Z: altri giri. Z cambia solo la rotazione nascosta (la fase).", "S, T e le loro versioni con pugnale (S†, T†), √X: piccole rotazioni attente.", "P(φ), Rx, Ry, Rz: ruotano di un angolo che scegli, in radianti. Puoi scrivere espressioni come pi/4 o 2*pi/3. Per cambiarlo, seleziona la porta (Sel) e modifica «Angolo (rad)».", "CNOT, CZ, CY, CH: «se il primo qubit è 1, fai il trucco sul secondo». CP e Rz controllata: uguale, con un angolo.", "SWAP: scambia due qubit. Toffoli: due controlli e un bersaglio. Fredkin: un controllo, scambia due qubit.", "Misura: guardi la moneta. Smette di girare e diventa 0 o 1.", "Gomma: toglie una porta. Modifica → Annulla / Ripeti sistemano gli errori. Ispeziona: passa il cursore su una porta per vederne la matrice."]),
 ], "Le porte sono piccoli trucchi che fai alle monete."),
 ("Eseguire, e sbagliare apposta", [
  ("p", "Misure è quante volte si ripete l’esperimento (da 1 a 1.000.000). Le risposte quantistiche sono probabilità, quindi si ripete tante volte e si conta, come tirare un dado 1000 volte."),
  ("p", "I veri computer quantistici sono maldestri. I cursori aggiungono una maldestraggine finta (rumore): depolarizzazione 1q, depolarizzazione 2q ed errore di lettura. Lasciali a 0 per una macchina perfetta. Il seed rende ripetibile il «caso»: stesso seed, stesse risposte (nella riga di comando, --seed)."),
 ], "Esegui = lancia tante volte e conta. Rumore = dita maldestre."),
 ("Quanto può diventare grande?", [
  ("p", "Ogni qubit in più raddoppia il lavoro, come un quaderno in cui ogni nuovo qubit raddoppia le pagine. 24 qubit richiedono circa 256 MiB di memoria; 28 qubit circa 4 GiB; 40 richiederebbero un intero data center."),
  ("p", "Per questo CUANTOR ha tre motori. Vettore di stato: ricorda ogni numero, esatto, fino a 28 qubit. Stabilizzatore: solo per porte speciali (H, S, CNOT, …), ma velocissimo, fino a 4096 qubit. Sparso (sparse): per circuiti con pochi rami. «Auto» sceglie per te."),
  ("p", "Esempio: uno stato GHZ da 1000 qubit (un grande gruppo di monete tutte d’accordo) gira all’istante. Un circuito qualsiasi da 1000 qubit è impossibile per qualunque computer normale, e nessun programma può cambiarlo."),
 ], "Più qubit significa molto, molto più lavoro. I disegni semplici possono restare enormi."),
 ("Il pannello del codice e i file", [
  ("ul", ["Il pannello del codice mostra sempre il circuito come testo. Modifica il testo, premi «Applica al circuito» e il disegno cambia.", "File → Apri .qasm e Salva .qasm (OpenQASM 3) caricano e salvano circuiti. Si possono importare OpenQASM 2.0 e 3.0.", "Alcuni trucchi non si possono disegnare nell’editor: reset e if (circuiti che decidono strada facendo, «circuiti dinamici»). Per loro usa la riga di comando."]),
 ], "Un circuito è disegno e testo insieme."),
 ("La riga di comando", [
  ("p", "Il programma in modalità testo cuantor fa la stessa matematica senza finestra. Comodo per script e circuiti molto grandi."),
  ("code", CLI),
  ("p", "Scegli la lingua con --lang xx o la variabile CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Aggiungi --backend sv|stab|sparse per il motore e --depol1, --depol2, --readout per il rumore."),
 ], "Stessa forza, niente finestra."),
 ("Parlare con IBM Quantum", [
  ("p", "Apri in IBM Composer: il pulsante salva il circuito come file .qasm. Nel Composer di IBM (quantum.cloud.ibm.com) scegli File → Upload .qasm file."),
  ("p", "Esecuzione su una vera macchina IBM: serve un tuo account IBM Quantum. Dai a CUANTOR la chiave e l’istanza (CRN) tramite variabili o con il file ~/.config/cuantor/ibm.json (con api_key, crn e backend). La chiave resta sul tuo computer e viene inviata solo a IBM."),
  ("code", IBM),
  ("note", "Avviso sincero: questo collegamento è stato provato solo con un server finto, mai su una vera macchina IBM. Potrebbe servire qualche piccola correzione. Le macchine vere hanno code e limiti di piano, e i circuiti con reset o if non si possono inviare."),
 ], "Puoi inviare circuiti a un vero computer quantistico IBM se hai un account. Con quello vero non l’abbiamo ancora provato."),
 ("Se qualcosa va storto", [
  ("ul", ["Avviso all’apertura: vedi il capitolo 2. I programmi non sono firmati.", "Lento o senza memoria: usa meno qubit o un motore adatto (stabilizzatore per porte semplici). CUANTOR rifiuta dimensioni che bloccherebbero il computer.", "«Angolo non valido»: scrivi un numero come 0.785 o un’espressione come pi/4.", "Lingua sbagliata: usa la casella della lingua o imposta CUANTOR_LANG.", "Errori IBM: controlla la chiave, il CRN e il nome del backend (cuantor ibm backends). Sulla chiave non viene stampato nulla, di proposito."]),
 ], "Bloccato? Meno qubit e numeri più semplici."),
]

G["pt"] = [
 ("O que é o CUANTOR?", [
  ("p", "Um computador normal pensa com bits. Um bit é como um interruptor: desligado (0) ou ligado (1). Um bit quântico, ou qubit, parece mais uma moeda a rodar no ar: até cair, é um pouco cara e um pouco coroa ao mesmo tempo."),
  ("p", "O CUANTOR é um computador quântico de faz de conta que vive dentro do teu computador. Faz a matemática quântica exatamente, mas é uma simulação, não uma máquina quântica real. É perfeito para aprender, brincar e testar ideias."),
 ], "O CUANTOR é um computador quântico de brincar no teu ecrã. Em vez de interruptores, tem moedas a rodar."),
 ("Instalação", [
  ("ol", ["Abre a página Releases (o botão Descarregar do site).", "Descarrega o ficheiro do teu computador: Windows .zip, Linux .tar.gz, Mac com chip Apple: macos-arm64, Mac mais antigo com processador Intel: macos-x86_64.", "Descompacta onde quiseres. Não há nada para instalar.", "Faz duplo clique em cuantor-gui. É o programa com janela. O ficheiro chamado cuantor é a ferramenta em modo texto (capítulo 9).", "Opcional: o ficheiro .sha256 serve para verificar que a transferência não está danificada."]),
  ("note", "Os programas não estão assinados digitalmente, por isso o Windows ou o macOS podem mostrar um aviso da primeira vez. É normal num projeto gratuito e de paixão. Abre apenas ficheiros descarregados da página oficial de releases."),
 ], "Descarrega, descompacta, duplo clique em cuantor-gui."),
 ("Passeio pela janela", [
  ("ul", ["Esquerda: a paleta. Blocos coloridos, chamados portas. São as tuas ferramentas.", "Meio: o circuito. Cada linha horizontal é um qubit (q[0], q[1], …). O tempo vai da esquerda para a direita, como ao ler.", "Direita: as respostas, desenhadas. Probabilidades (barras), a esfera Q (uma bola; as cores mostram a «fase», uma rotação escondida) e o vetor de estado (uma tabela de números).", "Painel de código: o mesmo circuito escrito como texto, em direto. Podes escolher OpenQASM 3.0, OpenQASM 2.0 ou Qiskit.", "Em cima: os menus Ficheiro, Editar, Ver e uma caixa de idioma com 8 idiomas."]),
 ], "Portas à esquerda, circuito ao meio, respostas à direita."),
 ("O teu primeiro circuito: duas moedas mágicas", [
  ("ol", ["Põe Qubits em 2.", "Escolhe H (Hadamard) na paleta e toca no primeiro fio q[0], na primeira coluna.", "Escolhe CNOT. Toca primeiro em q[0] (o controlo) e depois em q[1] (o alvo).", "Olha para Probabilidades: duas barras, 00 e 11, cerca de 50 % cada.", "Carrega em Executar. O CUANTOR repete a experiência 1000 vezes. Só vês 00 ou 11, nunca 01 nem 10."]),
  ("p", "Porquê? O H faz a primeira moeda rodar. O CNOT prende a segunda à primeira, por isso caem sempre iguais. Os cientistas chamam-lhe emaranhamento. Também o podes carregar em Ficheiro → Exemplos (os títulos dos exemplos ainda estão em romeno: «Încâlcire: starea Bell»)."),
  ("p", "O mesmo circuito como texto, se o quiseres escrever ou guardar:"),
  ("code", QASM),
 ], "Duas moedas mágicas que caem sempre iguais."),
 ("As portas, uma a uma", [
  ("ul", ["H: põe a moeda a rodar (metade cara, metade coroa).", "X (NOT): troca cara e coroa. Y e Z: outras viragens. O Z só muda a rotação escondida (a fase).", "S, T e as suas versões com punhal (S†, T†), √X: pequenas rotações cuidadosas.", "P(φ), Rx, Ry, Rz: rodam um ângulo que escolhes, em radianos. Podes escrever expressões como pi/4 ou 2*pi/3. Para o mudar, seleciona a porta (Sel) e edita «Ângulo (rad)».", "CNOT, CZ, CY, CH: «se o primeiro qubit for 1, faz o truque no segundo». CP e Rz controlada: igual, com ângulo.", "SWAP: troca dois qubits. Toffoli: dois controlos e um alvo. Fredkin: um controlo, troca dois.", "Medição: olhas para a moeda. Pára de rodar e fica 0 ou 1.", "Borracha: remove uma porta. Editar → Anular / Refazer corrigem erros. Inspecionar: passa o rato sobre uma porta para ver a matriz."]),
 ], "As portas são pequenos truques que fazes às moedas."),
 ("Executar, e errar de propósito", [
  ("p", "Medições é quantas vezes a experiência se repete (de 1 a 1.000.000). As respostas quânticas são probabilidades, por isso repete-se muitas vezes e conta-se, como lançar um dado 1000 vezes."),
  ("p", "Os computadores quânticos reais são desajeitados. Os cursores juntam desajeitamento de faz de conta (ruído): despolarização 1q, despolarização 2q e erro de leitura. Deixa-os em 0 para uma máquina perfeita. A semente (seed) torna o «acaso» repetível: mesma semente, mesmas respostas (na linha de comandos, --seed)."),
 ], "Executar = lançar muitas vezes e contar. Ruído = dedos desajeitados."),
 ("Até que tamanho pode ir?", [
  ("p", "Cada qubit a mais duplica o trabalho, como um caderno em que cada qubit novo duplica as páginas. 24 qubits precisam de cerca de 256 MiB de memória; 28 qubits, cerca de 4 GiB; 40 precisariam de um centro de dados inteiro."),
  ("p", "Por isso o CUANTOR tem três motores. Vetor de estado: guarda todos os números, exato, até 28 qubits. Estabilizador: só para portas especiais (H, S, CNOT, …), mas rapidíssimo, até 4096 qubits. Esparso (sparse): para circuitos com poucos ramos. O «Auto» escolhe por ti."),
  ("p", "Exemplo: um estado GHZ de 1000 qubits (um grande grupo de moedas todas de acordo) corre num instante. Um circuito qualquer de 1000 qubits é impossível para qualquer computador normal, e nenhum programa o pode mudar."),
 ], "Mais qubits, muito, muito mais trabalho. Padrões simples ainda podem ser enormes."),
 ("O painel de código e os ficheiros", [
  ("ul", ["O painel de código mostra sempre o circuito como texto. Edita o texto, carrega em «Aplicar ao circuito» e o desenho muda.", "Ficheiro → Abrir .qasm e Guardar .qasm (OpenQASM 3) carregam e guardam circuitos. Podem importar-se OpenQASM 2.0 e 3.0.", "Alguns truques não se desenham no editor: reset e if (circuitos que decidem pelo caminho, «circuitos dinâmicos»). Para esses usa a linha de comandos."]),
 ], "Um circuito é desenho e texto ao mesmo tempo."),
 ("A linha de comandos", [
  ("p", "O programa em modo texto cuantor faz a mesma matemática sem janela. Útil para scripts e circuitos muito grandes."),
  ("code", CLI),
  ("p", "Escolhe o idioma com --lang xx ou a variável CUANTOR_LANG (ro, en, fr, de, es, it, pt, hu). Junta --backend sv|stab|sparse para o motor e --depol1, --depol2, --readout para o ruído."),
 ], "A mesma força, sem janela."),
 ("Falar com a IBM Quantum", [
  ("p", "Abrir no IBM Composer: o botão guarda o circuito como ficheiro .qasm. No Composer da IBM (quantum.cloud.ibm.com) escolhe File → Upload .qasm file."),
  ("p", "Executar numa máquina IBM real: precisas de uma conta IBM Quantum tua. Dá ao CUANTOR a chave e a instância (CRN) por variáveis ou pelo ficheiro ~/.config/cuantor/ibm.json (com api_key, crn e backend). A chave fica no teu computador e só é enviada à IBM."),
  ("code", IBM),
  ("note", "Aviso sincero: esta ligação só foi testada contra um servidor de faz de conta, nunca numa máquina IBM real. Pode precisar de pequenas correções. As máquinas reais têm filas e limites de plano, e os circuitos com reset ou if não se podem enviar."),
 ], "Podes enviar circuitos para um computador quântico IBM real se tiveres conta. Ainda não o experimentámos no real."),
 ("Se algo correr mal", [
  ("ul", ["Aviso ao abrir: vê o capítulo 2. Os programas não estão assinados.", "Lento ou sem memória: usa menos qubits ou um motor adequado (estabilizador para portas simples). O CUANTOR recusa tamanhos que bloqueariam o teu computador.", "«Ângulo inválido»: escreve um número como 0.785 ou uma expressão como pi/4.", "Idioma errado: usa a caixa de idioma ou define CUANTOR_LANG.", "Erros da IBM: verifica a chave, o CRN e o nome do backend (cuantor ibm backends). Sobre a chave não se mostra nada, de propósito."]),
 ], "Preso? Menos qubits e números mais simples."),
]

G["hu"] = [
 ("Mi az a CUANTOR?", [
  ("p", "A hétköznapi számítógép bitekkel gondolkodik. A bit olyan, mint egy villanykapcsoló: kikapcsolt (0) vagy bekapcsolt (1). A kvantumbit, vagyis qubit inkább egy levegőben pörgő pénzérméhez hasonlít: amíg le nem esik, egyszerre egy kicsit fej és egy kicsit írás."),
  ("p", "A CUANTOR egy játék-kvantumszámítógép, amely a te számítógépedben él. A kvantummatematikát pontosan számolja, de szimuláció, nem valódi kvantumgép. Tanuláshoz, játékhoz és ötletek kipróbálásához tökéletes."),
 ], "A CUANTOR játék kvantumszámítógép a képernyődön. Kapcsolók helyett pörgő érméi vannak."),
 ("Telepítés", [
  ("ol", ["Nyisd meg a Releases oldalt (a weboldal Letöltés gombja).", "Töltsd le a géped fájlját: Windows .zip, Linux .tar.gz, Apple-chipes Mac: macos-arm64, régebbi, Intel processzoros Mac: macos-x86_64.", "Csomagold ki, ahová csak akarod. Nincs mit telepíteni.", "Kattints duplán a cuantor-gui fájlra. Ez az ablakos program. A cuantor nevű fájl a szöveges eszköz (9. fejezet).", "Választható: a .sha256 fájllal ellenőrizheted, hogy a letöltés nem sérült."]),
  ("note", "A programok nincsenek digitálisan aláírva, ezért a Windows vagy a macOS az első indításkor figyelmeztethet. Ez egy ingyenes hobbiprojektnél normális. Csak a hivatalos kiadási oldalról letöltött fájlokat nyisd meg."),
 ], "Letöltés, kicsomagolás, dupla kattintás a cuantor-gui-ra."),
 ("Körséta az ablakban", [
  ("ul", ["Balra: a paletta. Színes építőkockák, ezek a kapuk. Ezek az eszközeid.", "Középen: az áramkör. Minden vízszintes vonal egy qubit (q[0], q[1], …). Az idő balról jobbra telik, mint olvasáskor.", "Jobbra: a válaszok, lerajzolva. Valószínűségek (oszlopok), a Q-gömb (egy golyó; a színek a „fázist”, egy rejtett csavarást mutatnak) és az állapotvektor (számtáblázat).", "Kódpanel: ugyanaz az áramkör szövegként, élőben. Választhatsz OpenQASM 3.0, OpenQASM 2.0 vagy Qiskit között.", "Fent: a Fájl, Szerkesztés, Nézet menü és egy 8 nyelvű nyelvválasztó."]),
 ], "Kapuk balra, áramkör középen, válaszok jobbra."),
 ("Az első áramköröd: két varázsérme", [
  ("ol", ["Állítsd a Qubitek számát 2-re.", "Válaszd a H (Hadamard) kaput a palettáról, és érintsd meg az első vezetéket, q[0]-t, az első oszlopban.", "Válaszd a CNOT-ot. Előbb a q[0]-t érintsd meg (a vezérlő), aztán a q[1]-et (a cél).", "Nézd meg a Valószínűségeket: két oszlop, 00 és 11, mindkettő kb. 50%.", "Nyomd meg a Futtatás gombot. A CUANTOR 1000-szer megismétli a kísérletet. Mindig csak 00-t vagy 11-et látsz, soha 01-et vagy 10-et."]),
  ("p", "Miért? A H megpörgeti az első érmét. A CNOT a másodikat az elsőhöz köti, így mindig ugyanúgy esnek le. A tudósok ezt összefonódásnak hívják. Betöltheted a Fájl → Példák menüből is (a példák címe még románul van: „Încâlcire: starea Bell”)."),
  ("p", "Ugyanez az áramkör szövegként, ha begépelnéd vagy elmentenéd:"),
  ("code", QASM),
 ], "Két varázsérme, amely mindig ugyanúgy esik le."),
 ("A kapuk egyenként", [
  ("ul", ["H: megpörgeti az érmét (félig fej, félig írás).", "X (NOT): felcseréli a fejet és az írást. Y és Z: más fordítások. A Z csak a rejtett csavarást (a fázist) változtatja.", "S, T és tőrös párjuk (S†, T†), √X: apró, óvatos csavarások.", "P(φ), Rx, Ry, Rz: általad választott szöggel forgatnak, radiánban. Írhatsz kifejezést is, például pi/4 vagy 2*pi/3. A módosításhoz jelöld ki a kaput (Kij.), és szerkeszd a „Szög (rad)” mezőt.", "CNOT, CZ, CY, CH: „ha az első qubit 1, csináld meg a trükköt a másodikon”. CP és vezérelt Rz: ugyanez, szöggel.", "SWAP: két qubitet cserél meg. Toffoli: két vezérlő és egy cél. Fredkin: egy vezérlő, két qubitet cserél.", "Mérés: ránézel az érmére. Megáll, és 0 vagy 1 lesz.", "Radír: töröl egy kaput. Szerkesztés → Visszavonás / Újra javítja a hibákat. Vizsgálat: vidd az egeret egy kapu fölé, és látod a mátrixát."]),
 ], "A kapuk apró trükkök, amiket az érmékkel csinálsz."),
 ("Futtatás és szándékos hibák", [
  ("p", "A Mérések száma azt adja meg, hányszor ismétlődik a kísérlet (1-től 1 000 000-ig). A kvantumválaszok esélyek, ezért sokszor ismételünk és számolunk, mint amikor 1000-szer dobunk egy kockával."),
  ("p", "A valódi kvantumszámítógépek ügyetlenek. A csúszkák színlelt ügyetlenséget (zajt) adnak hozzá: 1 qubites depolarizáció, 2 qubites depolarizáció és kiolvasási hiba. Hagyd 0-n a tökéletes géphez. A seed megismételhetővé teszi a „véletlent”: ugyanaz a seed, ugyanazok a válaszok (parancssorban --seed)."),
 ], "Futtatás = sokszor dobunk és számolunk. Zaj = ügyetlen ujjak."),
 ("Mekkora lehet?", [
  ("p", "Minden további qubit megduplázza a munkát, mint egy füzet, ahol minden új qubit duplájára növeli az oldalak számát. 24 qubithez kb. 256 MiB memória kell, 28-hoz kb. 4 GiB, 40-hez egy egész adatközpont."),
  ("p", "Ezért van a CUANTOR-nak három motorja. Állapotvektor: minden számot megjegyez, pontos, 28 qubitig. Stabilizátor: csak különleges kapukhoz (H, S, CNOT, …), de villámgyors, 4096 qubitig. Ritka (sparse): kevés ágú áramkörökhöz. Az „Auto” kiválasztja neked."),
  ("p", "Példa: egy 1000 qubites GHZ-állapot (érmék nagy csoportja, amelyek mind egyetértenek) azonnal lefut. Egy tetszőleges 1000 qubites áramkör bármelyik hétköznapi számítógépnek lehetetlen, és ezen semmilyen program nem változtat."),
 ], "Több qubit sokkal-sokkal több munka. Az egyszerű minták azonban lehetnek hatalmasak."),
 ("A kódpanel és a fájlok", [
  ("ul", ["A kódpanel mindig szövegként mutatja az áramkört. Szerkeszd a szöveget, nyomd meg az „Alkalmazás az áramkörre” gombot, és a rajz megváltozik.", "Fájl → .qasm megnyitása és .qasm mentése (OpenQASM 3) tölt be és ment áramköröket. OpenQASM 2.0 és 3.0 importálható.", "Néhány trükköt nem lehet lerajzolni a szerkesztőben: reset és if (menet közben döntő áramkörök, „dinamikus áramkörök”). Ezekhez használd a parancssort."]),
 ], "Az áramkör egyszerre rajz és szöveg."),
 ("A parancssor", [
  ("p", "A szöveges cuantor program ugyanazt számolja ablak nélkül. Hasznos szkriptekhez és nagyon nagy áramkörökhöz."),
  ("code", CLI),
  ("p", "A nyelvet a --lang xx kapcsolóval vagy a CUANTOR_LANG változóval választhatod (ro, en, fr, de, es, it, pt, hu). A --backend sv|stab|sparse a motort, a --depol1, --depol2, --readout a zajt állítja."),
 ], "Ugyanaz az erő, ablak nélkül."),
 ("Kapcsolat az IBM Quantummal", [
  ("p", "Megnyitás az IBM Composerben: a gomb .qasm fájlként menti az áramkört. Az IBM Composerében (quantum.cloud.ibm.com) válaszd a File → Upload .qasm file parancsot."),
  ("p", "Futtatás valódi IBM-gépen: saját IBM Quantum-fiók kell hozzá. Add meg a CUANTOR-nak a kulcsodat és a példányodat (CRN) változókkal, vagy a ~/.config/cuantor/ibm.json fájlban (api_key, crn és backend mezőkkel). A kulcs a gépeden marad, és csak az IBM-nek megy el."),
  ("code", IBM),
  ("note", "Őszinte figyelmeztetés: ezt a kapcsolatot csak egy színlelt szerverrel próbáltuk ki, valódi IBM-gépen soha. Apró javításokra szorulhat. A valódi gépeken sorok és csomaghatárok vannak, a reset vagy if tartalmú áramkörök pedig nem küldhetők el."),
 ], "Fiókkal elküldheted az áramköreidet egy valódi IBM kvantumszámítógépnek. A valódin még nem próbáltuk."),
 ("Ha valami elromlik", [
  ("ul", ["Figyelmeztetés megnyitáskor: lásd a 2. fejezetet. A programok nincsenek aláírva.", "Lassú vagy elfogy a memória: használj kevesebb qubitet vagy megfelelő motort (stabilizátor egyszerű kapukhoz). A CUANTOR visszautasítja azokat a méreteket, amelyek lefagyasztanák a gépedet.", "„Érvénytelen szög”: írj számot, például 0.785, vagy kifejezést, például pi/4.", "Rossz nyelv: használd a nyelvválasztót, vagy állítsd be a CUANTOR_LANG-ot.", "IBM-hibák: ellenőrizd a kulcsot, a CRN-t és a backend nevét (cuantor ibm backends). A kulcsról szándékosan semmi sem íródik ki."]),
 ], "Elakadtál? Kevesebb qubit és egyszerűbb számok."),
]
