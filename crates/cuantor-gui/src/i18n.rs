//! Traduceri ale interfeței în 8 limbi. Cheia este textul românesc; ordinea limbilor: EN FR DE ES IT PT HU.

use std::sync::atomic::{AtomicU8, Ordering};

static CURRENT: AtomicU8 = AtomicU8::new(0);

/// Limba activă a interfeței (globală, ca să poată fi folosită și în funcții libere).
pub fn set(l: Lang) {
    CURRENT.store(l as u8, Ordering::Relaxed);
}
pub fn get() -> Lang {
    Lang::ALL[CURRENT.load(Ordering::Relaxed) as usize % 8]
}
pub fn tr(key: &'static str) -> &'static str {
    get().t(key)
}
pub fn trf(key: &'static str, args: &[&str]) -> String {
    get().tf(key, args)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Ro,
    En,
    Fr,
    De,
    Es,
    It,
    Pt,
    Hu,
}

impl Lang {
    pub const ALL: [Lang; 8] = [Lang::Ro, Lang::En, Lang::Fr, Lang::De, Lang::Es, Lang::It, Lang::Pt, Lang::Hu];
    pub fn code(self) -> &'static str {
        ["ro", "en", "fr", "de", "es", "it", "pt", "hu"][self as usize]
    }
    pub fn name(self) -> &'static str {
        ["Română", "English", "Français", "Deutsch", "Español", "Italiano", "Português", "Magyar"][self as usize]
    }
    pub fn from_code(c: &str) -> Option<Lang> {
        let c = c.trim().to_lowercase();
        Lang::ALL.into_iter().find(|l| c.starts_with(l.code()))
    }
    /// Limba salvată local, apoi cea din mediu (LANGUAGE/LC_ALL/LANG), altfel engleză.
    pub fn detect() -> Lang {
        if let Some(l) = config_path().and_then(|p| std::fs::read_to_string(p).ok()).and_then(|t| Lang::from_code(&t)) {
            return l;
        }
        for v in ["LANGUAGE", "LC_ALL", "LANG"] {
            if let Some(l) = std::env::var(v).ok().and_then(|x| Lang::from_code(&x)) {
                return l;
            }
        }
        Lang::En
    }
    pub fn save(self) {
        if let Some(p) = config_path() {
            if let Some(d) = p.parent() {
                let _ = std::fs::create_dir_all(d);
            }
            let _ = std::fs::write(p, self.code());
        }
    }
    /// Traduce o cheie (text românesc). Chei necunoscute se întorc neschimbate.
    pub fn t(self, key: &'static str) -> &'static str {
        if self == Lang::Ro {
            return key;
        }
        TABLE.iter().find(|(k, _)| *k == key).map(|(_, v)| v[self as usize - 1]).unwrap_or(key)
    }
    /// Ca `t`, dar înlocuiește pe rând fiecare `{}` cu argumentele date.
    pub fn tf(self, key: &'static str, args: &[&str]) -> String {
        let mut out = String::new();
        let mut it = args.iter();
        let mut rest = self.t(key);
        while let Some(i) = rest.find("{}") {
            out.push_str(&rest[..i]);
            out.push_str(it.next().copied().unwrap_or(""));
            rest = &rest[i + 2..];
        }
        out.push_str(rest);
        out
    }
}

fn config_path() -> Option<std::path::PathBuf> {
    let h = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
    Some(std::path::Path::new(&h).join(".config/cuantor/lang"))
}

type Row = (&'static str, [&'static str; 7]);

pub const TABLE: &[Row] = &[
    ("Fișier", ["File", "Fichier", "Datei", "Archivo", "File", "Ficheiro", "Fájl"]),
    ("Editare", ["Edit", "Édition", "Bearbeiten", "Edición", "Modifica", "Editar", "Szerkesztés"]),
    ("Vizualizare", ["View", "Affichage", "Ansicht", "Ver", "Visualizza", "Ver", "Nézet"]),
    ("Nou", ["New", "Nouveau", "Neu", "Nuevo", "Nuovo", "Novo", "Új"]),
    ("Cale:", ["Path:", "Chemin :", "Pfad:", "Ruta:", "Percorso:", "Caminho:", "Útvonal:"]),
    ("Deschide .qasm", ["Open .qasm", "Ouvrir .qasm", ".qasm öffnen", "Abrir .qasm", "Apri .qasm", "Abrir .qasm", ".qasm megnyitása"]),
    ("Salvează .qasm (OpenQASM 3)", ["Save .qasm (OpenQASM 3)", "Enregistrer .qasm (OpenQASM 3)", ".qasm speichern (OpenQASM 3)", "Guardar .qasm (OpenQASM 3)", "Salva .qasm (OpenQASM 3)", "Guardar .qasm (OpenQASM 3)", ".qasm mentése (OpenQASM 3)"]),
    ("Exemple", ["Examples", "Exemples", "Beispiele", "Ejemplos", "Esempi", "Exemplos", "Példák"]),
    ("Anulează", ["Undo", "Annuler", "Rückgängig", "Deshacer", "Annulla", "Anular", "Visszavonás"]),
    ("Refă", ["Redo", "Rétablir", "Wiederholen", "Rehacer", "Ripeti", "Refazer", "Ismétlés"]),
    ("Golește circuitul", ["Clear circuit", "Vider le circuit", "Schaltung leeren", "Vaciar el circuito", "Svuota il circuito", "Limpar o circuito", "Áramkör törlése"]),
    ("Inspectare (matricea porții la hover)", ["Inspect (gate matrix on hover)", "Inspecter (matrice de la porte au survol)", "Prüfen (Gattermatrix beim Überfahren)", "Inspeccionar (matriz de la puerta al pasar el cursor)", "Ispeziona (matrice della porta al passaggio)", "Inspecionar (matriz da porta ao passar o cursor)", "Vizsgálat (kapumátrix rámutatáskor)"]),
    ("Qubiți:", ["Qubits:", "Qubits :", "Qubits:", "Cúbits:", "Qubit:", "Qubits:", "Qubitek:"]),
    ("Motor:", ["Engine:", "Moteur :", "Motor:", "Motor:", "Motore:", "Motor:", "Motor:"]),
    ("Rulează", ["Run", "Exécuter", "Ausführen", "Ejecutar", "Esegui", "Executar", "Futtatás"]),
    ("Motor activ:", ["Active engine:", "Moteur actif :", "Aktiver Motor:", "Motor activo:", "Motore attivo:", "Motor ativo:", "Aktív motor:"]),
    ("memorie", ["memory", "mémoire", "Speicher", "memoria", "memoria", "memória", "memória"]),
    ("amplitudini nenule", ["non-zero amplitudes", "amplitudes non nulles", "Amplituden ungleich null", "amplitudes no nulas", "ampiezze non nulle", "amplitudes não nulas", "nem nulla amplitúdó"]),
    ("calculez…", ["computing…", "calcul…", "Berechnung…", "calculando…", "calcolo…", "a calcular…", "számítás…"]),
    ("se calculează…", ["computing…", "calcul en cours…", "wird berechnet…", "calculando…", "calcolo in corso…", "a calcular…", "számítás folyamatban…"]),
    ("Operații", ["Operations", "Opérations", "Operationen", "Operaciones", "Operazioni", "Operações", "Műveletek"]),
    ("Unghi (rad):", ["Angle (rad):", "Angle (rad) :", "Winkel (rad):", "Ángulo (rad):", "Angolo (rad):", "Ângulo (rad):", "Szög (rad):"]),
    ("expresie invalidă", ["invalid expression", "expression invalide", "ungültiger Ausdruck", "expresión no válida", "espressione non valida", "expressão inválida", "érvénytelen kifejezés"]),
    ("Execuție și zgomot", ["Execution and noise", "Exécution et bruit", "Ausführung und Rauschen", "Ejecución y ruido", "Esecuzione e rumore", "Execução e ruído", "Futtatás és zaj"]),
    ("Măsurători:", ["Shots:", "Mesures :", "Messungen:", "Mediciones:", "Misure:", "Medições:", "Mérések:"]),
    ("depolarizare 1q", ["1q depolarizing", "dépolarisation 1q", "Depolarisation 1q", "despolarización 1q", "depolarizzazione 1q", "despolarização 1q", "1q depolarizáció"]),
    ("depolarizare 2q", ["2q depolarizing", "dépolarisation 2q", "Depolarisation 2q", "despolarización 2q", "depolarizzazione 2q", "despolarização 2q", "2q depolarizáció"]),
    ("eroare citire", ["readout error", "erreur de lecture", "Auslesefehler", "error de lectura", "errore di lettura", "erro de leitura", "kiolvasási hiba"]),
    ("IBM Quantum", ["IBM Quantum", "IBM Quantum", "IBM Quantum", "IBM Quantum", "IBM Quantum", "IBM Quantum", "IBM Quantum"]),
    ("Cheia API și CRN se citesc local din variabile de mediu sau ~/.config/cuantor/ibm.json; nu se salvează în aplicație.", [
        "The API key and CRN are read locally from environment variables or ~/.config/cuantor/ibm.json; they are never stored in the app.",
        "La clé API et le CRN sont lus localement depuis les variables d'environnement ou ~/.config/cuantor/ibm.json ; ils ne sont jamais enregistrés dans l'application.",
        "API-Schlüssel und CRN werden lokal aus Umgebungsvariablen oder ~/.config/cuantor/ibm.json gelesen; sie werden nie in der App gespeichert.",
        "La clave API y el CRN se leen localmente de variables de entorno o de ~/.config/cuantor/ibm.json; nunca se guardan en la aplicación.",
        "La chiave API e il CRN vengono letti localmente da variabili d'ambiente o da ~/.config/cuantor/ibm.json; non vengono mai salvati nell'app.",
        "A chave API e o CRN são lidos localmente a partir de variáveis de ambiente ou de ~/.config/cuantor/ibm.json; nunca são guardados na aplicação.",
        "Az API-kulcsot és a CRN-t a program helyben, környezeti változókból vagy a ~/.config/cuantor/ibm.json fájlból olvassa; soha nem tárolja őket.",
    ]),
    ("Backend:", ["Backend:", "Backend :", "Backend:", "Backend:", "Backend:", "Backend:", "Backend:"]),
    ("Trimite pe hardware IBM", ["Run on IBM hardware", "Envoyer sur le matériel IBM", "Auf IBM-Hardware senden", "Enviar al hardware de IBM", "Invia all'hardware IBM", "Enviar para o hardware IBM", "Küldés IBM hardverre"]),
    ("Transpilează în porți native și trimite un job Sampler (neverificat pe hardware real)", [
        "Transpiles to native gates and submits a Sampler job (untested on real hardware)",
        "Transpile en portes natives et envoie un job Sampler (non testé sur du matériel réel)",
        "Übersetzt in native Gatter und sendet einen Sampler-Job (auf echter Hardware ungetestet)",
        "Transpila a puertas nativas y envía un trabajo Sampler (sin probar en hardware real)",
        "Traspila in porte native e invia un job Sampler (non verificato su hardware reale)",
        "Transpila para portas nativas e envia um job Sampler (não testado em hardware real)",
        "Natív kapukra fordít és Sampler jobot küld (valódi hardveren nem tesztelt)",
    ]),
    ("Deschide în IBM Composer", ["Open in IBM Composer", "Ouvrir dans IBM Composer", "In IBM Composer öffnen", "Abrir en IBM Composer", "Apri in IBM Composer", "Abrir no IBM Composer", "Megnyitás az IBM Composerben"]),
    ("Salvează circuitul ca .qasm și deschide Composer; apoi Fișier → Upload .qasm file", [
        "Saves the circuit as .qasm and opens Composer; then File → Upload .qasm file",
        "Enregistre le circuit en .qasm et ouvre Composer ; puis Fichier → Upload .qasm file",
        "Speichert die Schaltung als .qasm und öffnet Composer; dann Datei → Upload .qasm file",
        "Guarda el circuito como .qasm y abre Composer; luego Archivo → Upload .qasm file",
        "Salva il circuito come .qasm e apre Composer; poi File → Upload .qasm file",
        "Guarda o circuito como .qasm e abre o Composer; depois Ficheiro → Upload .qasm file",
        "A kapcsolást .qasm fájlba menti és megnyitja a Composert; utána Fájl → Upload .qasm file",
    ]),
    ("Copiază", ["Copy", "Copier", "Kopieren", "Copiar", "Copia", "Copiar", "Másolás"]),
    ("Aplică în circuit", ["Apply to circuit", "Appliquer au circuit", "Auf Schaltung anwenden", "Aplicar al circuito", "Applica al circuito", "Aplicar ao circuito", "Alkalmazás az áramkörre"]),
    ("Probabilități", ["Probabilities", "Probabilités", "Wahrscheinlichkeiten", "Probabilidades", "Probabilità", "Probabilidades", "Valószínűségek"]),
    ("Rezultate măsurare", ["Measurement results", "Résultats de mesure", "Messergebnisse", "Resultados de medición", "Risultati delle misure", "Resultados das medições", "Mérési eredmények"]),
    ("Q-sferă", ["Q-sphere", "Sphère Q", "Q-Sphäre", "Esfera Q", "Sfera Q", "Esfera Q", "Q-gömb"]),
    ("Vector de stare", ["State vector", "Vecteur d'état", "Zustandsvektor", "Vector de estado", "Vettore di stato", "Vetor de estado", "Állapotvektor"]),
    ("Stare", ["State", "État", "Zustand", "Estado", "Stato", "Estado", "Állapot"]),
    ("Unghi de fază", ["Phase angle", "Angle de phase", "Phasenwinkel", "Ángulo de fase", "Angolo di fase", "Ângulo de fase", "Fázisszög"]),
    ("Fază", ["Phase", "Phase", "Phase", "Fase", "Fase", "Fase", "Fázis"]),
    ("Starea după pasul:", ["State after step:", "État après l'étape :", "Zustand nach Schritt:", "Estado tras el paso:", "Stato dopo il passo:", "Estado após o passo:", "Állapot a lépés után:"]),
    ("Apasă «Rulează» pentru măsurători", ["Press “Run” to take measurements", "Appuyez sur « Exécuter » pour mesurer", "„Ausführen“ drücken, um zu messen", "Pulsa «Ejecutar» para medir", "Premi «Esegui» per misurare", "Prima «Executar» para medir", "Nyomja meg a „Futtatás” gombot a méréshez"]),
    ("Limbă:", ["Language:", "Langue :", "Sprache:", "Idioma:", "Lingua:", "Idioma:", "Nyelv:"]),
    ("Șt", ["Del", "Eff", "Lö", "Bor", "Can", "Apg", "Tö"]),
    ("Sel", ["Sel", "Sél", "Wäh", "Sel", "Sel", "Sel", "Kij"]),
    ("Gumă", ["Eraser", "Gomme", "Radierer", "Borrador", "Gomma", "Borracha", "Radír"]),
    ("Hadamard: superpoziție egală", ["Hadamard: equal superposition", "Hadamard : superposition égale", "Hadamard: gleiche Überlagerung", "Hadamard: superposición igual", "Hadamard: sovrapposizione uniforme", "Hadamard: sobreposição igual", "Hadamard: egyenlő szuperpozíció"]),
    ("NOT (Pauli-X)", ["NOT (Pauli-X)", "NON (Pauli-X)", "NOT (Pauli-X)", "NOT (Pauli-X)", "NOT (Pauli-X)", "NOT (Pauli-X)", "NOT (Pauli-X)"]),
    ("Pauli-Y", ["Pauli-Y"; 7]),
    ("Pauli-Z", ["Pauli-Z"; 7]),
    ("S (fază π/2)", ["S (phase π/2)", "S (phase π/2)", "S (Phase π/2)", "S (fase π/2)", "S (fase π/2)", "S (fase π/2)", "S (fázis π/2)"]),
    ("S† (fază −π/2)", ["S† (phase −π/2)", "S† (phase −π/2)", "S† (Phase −π/2)", "S† (fase −π/2)", "S† (fase −π/2)", "S† (fase −π/2)", "S† (fázis −π/2)"]),
    ("T (fază π/4)", ["T (phase π/4)", "T (phase π/4)", "T (Phase π/4)", "T (fase π/4)", "T (fase π/4)", "T (fase π/4)", "T (fázis π/4)"]),
    ("T† (fază −π/4)", ["T† (phase −π/4)", "T† (phase −π/4)", "T† (Phase −π/4)", "T† (fase −π/4)", "T† (fase −π/4)", "T† (fase −π/4)", "T† (fázis −π/4)"]),
    ("Rădăcina pătrată din X", ["Square root of X", "Racine carrée de X", "Quadratwurzel von X", "Raíz cuadrada de X", "Radice quadrata di X", "Raiz quadrada de X", "X négyzetgyöke"]),
    ("Poartă de fază P(φ)", ["Phase gate P(φ)", "Porte de phase P(φ)", "Phasengatter P(φ)", "Puerta de fase P(φ)", "Porta di fase P(φ)", "Porta de fase P(φ)", "Fáziskapu P(φ)"]),
    ("Rotație în jurul axei X", ["Rotation about the X axis", "Rotation autour de l'axe X", "Drehung um die X-Achse", "Rotación alrededor del eje X", "Rotazione attorno all'asse X", "Rotação em torno do eixo X", "Forgatás az X tengely körül"]),
    ("Rotație în jurul axei Y", ["Rotation about the Y axis", "Rotation autour de l'axe Y", "Drehung um die Y-Achse", "Rotación alrededor del eje Y", "Rotazione attorno all'asse Y", "Rotação em torno do eixo Y", "Forgatás az Y tengely körül"]),
    ("Rotație în jurul axei Z (doar fază)", ["Rotation about the Z axis (phase only)", "Rotation autour de l'axe Z (phase seulement)", "Drehung um die Z-Achse (nur Phase)", "Rotación alrededor del eje Z (solo fase)", "Rotazione attorno all'asse Z (solo fase)", "Rotação em torno do eixo Z (só fase)", "Forgatás a Z tengely körül (csak fázis)"]),
    ("CNOT: control apoi țintă", ["CNOT: control then target", "CNOT : contrôle puis cible", "CNOT: erst Steuer-, dann Zielqubit", "CNOT: control y luego objetivo", "CNOT: controllo poi bersaglio", "CNOT: controlo e depois alvo", "CNOT: vezérlő, majd cél"]),
    ("CZ: control apoi țintă", ["CZ: control then target", "CZ : contrôle puis cible", "CZ: erst Steuer-, dann Zielqubit", "CZ: control y luego objetivo", "CZ: controllo poi bersaglio", "CZ: controlo e depois alvo", "CZ: vezérlő, majd cél"]),
    ("CY: control apoi țintă", ["CY: control then target", "CY : contrôle puis cible", "CY: erst Steuer-, dann Zielqubit", "CY: control y luego objetivo", "CY: controllo poi bersaglio", "CY: controlo e depois alvo", "CY: vezérlő, majd cél"]),
    ("CH: control apoi țintă", ["CH: control then target", "CH : contrôle puis cible", "CH: erst Steuer-, dann Zielqubit", "CH: control y luego objetivo", "CH: controllo poi bersaglio", "CH: controlo e depois alvo", "CH: vezérlő, majd cél"]),
    ("Fază controlată CP(φ)", ["Controlled phase CP(φ)", "Phase contrôlée CP(φ)", "Gesteuerte Phase CP(φ)", "Fase controlada CP(φ)", "Fase controllata CP(φ)", "Fase controlada CP(φ)", "Vezérelt fázis CP(φ)"]),
    ("Rz controlat", ["Controlled Rz", "Rz contrôlée", "Gesteuertes Rz", "Rz controlada", "Rz controllata", "Rz controlada", "Vezérelt Rz"]),
    ("SWAP: doi qubiți", ["SWAP: two qubits", "SWAP : deux qubits", "SWAP: zwei Qubits", "SWAP: dos cúbits", "SWAP: due qubit", "SWAP: dois qubits", "SWAP: két qubit"]),
    ("Toffoli: 2 controale, țintă", ["Toffoli: 2 controls, target", "Toffoli : 2 contrôles, cible", "Toffoli: 2 Steuerqubits, Ziel", "Toffoli: 2 controles, objetivo", "Toffoli: 2 controlli, bersaglio", "Toffoli: 2 controlos, alvo", "Toffoli: 2 vezérlő, cél"]),
    ("Fredkin: control, 2 qubiți", ["Fredkin: control, 2 qubits", "Fredkin : contrôle, 2 qubits", "Fredkin: Steuerqubit, 2 Qubits", "Fredkin: control, 2 cúbits", "Fredkin: controllo, 2 qubit", "Fredkin: controlo, 2 qubits", "Fredkin: vezérlő, 2 qubit"]),
    ("Măsurare în baza Z", ["Measure in the Z basis", "Mesure dans la base Z", "Messung in der Z-Basis", "Medición en la base Z", "Misura nella base Z", "Medição na base Z", "Mérés a Z bázisban"]),
    ("Selectează o poartă (pentru unghi)", ["Select a gate (to edit its angle)", "Sélectionner une porte (pour son angle)", "Gatter auswählen (für den Winkel)", "Seleccionar una puerta (para su ángulo)", "Seleziona una porta (per l'angolo)", "Selecionar uma porta (para o ângulo)", "Kapu kijelölése (a szöghöz)"]),
    ("controlul", ["the control", "le contrôle", "das Steuerqubit", "el control", "il controllo", "o controlo", "a vezérlőt"]),
    ("ținta", ["the target", "la cible", "das Zielqubit", "el objetivo", "il bersaglio", "o alvo", "a célt"]),
    ("qubitul", ["the qubit", "le qubit", "das Qubit", "el cúbit", "il qubit", "o qubit", "a qubitet"]),
    ("{}: alege {} (atinge un fir; toți qubiții pe aceeași coloană).", [
        "{}: pick {} (tap a wire; all qubits in the same column).",
        "{} : choisissez {} (touchez un fil ; tous les qubits dans la même colonne).",
        "{}: {} wählen (Leitung antippen; alle Qubits in derselben Spalte).",
        "{}: elige {} (toca un hilo; todos los cúbits en la misma columna).",
        "{}: scegli {} (tocca un filo; tutti i qubit nella stessa colonna).",
        "{}: escolha {} (toque num fio; todos os qubits na mesma coluna).",
        "{}: válassza ki {} (érintsen meg egy vezetéket; minden qubit ugyanabban az oszlopban).",
    ]),
    ("Celulă ocupată: alege alt pas sau șterge poarta.", ["Cell occupied: pick another step or delete the gate.", "Cellule occupée : choisissez une autre étape ou supprimez la porte.", "Zelle belegt: anderen Schritt wählen oder das Gatter löschen.", "Celda ocupada: elige otro paso o elimina la puerta.", "Cella occupata: scegli un altro passo o elimina la porta.", "Célula ocupada: escolha outro passo ou apague a porta.", "A cella foglalt: válasszon másik lépést, vagy törölje a kaput."]),
    ("Unghi invalid", ["Invalid angle", "Angle invalide", "Ungültiger Winkel", "Ángulo no válido", "Angolo non valido", "Ângulo inválido", "Érvénytelen szög"]),
    ("Rezultate IBM primite", ["IBM results received", "Résultats IBM reçus", "IBM-Ergebnisse empfangen", "Resultados de IBM recibidos", "Risultati IBM ricevuti", "Resultados da IBM recebidos", "IBM-eredmények megérkeztek"]),
    ("Eroare la măsurare: {}", ["Measurement error: {}", "Erreur de mesure : {}", "Messfehler: {}", "Error de medición: {}", "Errore di misura: {}", "Erro de medição: {}", "Mérési hiba: {}"]),
    ("Deschis {}", ["Opened {}", "Ouvert : {}", "Geöffnet: {}", "Abierto: {}", "Aperto: {}", "Aberto: {}", "Megnyitva: {}"]),
    ("Salvat {}", ["Saved {}", "Enregistré : {}", "Gespeichert: {}", "Guardado: {}", "Salvato: {}", "Guardado: {}", "Mentve: {}"]),
    ("Nu pot citi {}: {}", ["Cannot read {}: {}", "Lecture impossible de {} : {}", "{} kann nicht gelesen werden: {}", "No se puede leer {}: {}", "Impossibile leggere {}: {}", "Não é possível ler {}: {}", "Nem olvasható: {}: {}"]),
    ("Nu pot scrie {}: {}", ["Cannot write {}: {}", "Écriture impossible de {} : {}", "{} kann nicht geschrieben werden: {}", "No se puede escribir {}: {}", "Impossibile scrivere {}: {}", "Não é possível escrever {}: {}", "Nem írható: {}: {}"]),
    ("Salvat {} — încarcă-l în Composer (File → Upload .qasm file)", ["Saved {} — upload it in Composer (File → Upload .qasm file)", "Enregistré : {} — importez-le dans Composer (File → Upload .qasm file)", "Gespeichert: {} — in Composer hochladen (File → Upload .qasm file)", "Guardado: {} — súbelo en Composer (File → Upload .qasm file)", "Salvato: {} — caricalo in Composer (File → Upload .qasm file)", "Guardado: {} — carregue-o no Composer (File → Upload .qasm file)", "Mentve: {} — töltse fel a Composerben (File → Upload .qasm file)"]),
    ("Nu pot scrie fișierul: {}", ["Cannot write the file: {}", "Impossible d'écrire le fichier : {}", "Datei kann nicht geschrieben werden: {}", "No se puede escribir el archivo: {}", "Impossibile scrivere il file: {}", "Não é possível escrever o ficheiro: {}", "A fájl nem írható: {}"]),
    ("eșantion din {} rulări", ["sample of {} runs", "échantillon de {} exécutions", "Stichprobe aus {} Läufen", "muestra de {} ejecuciones", "campione di {} esecuzioni", "amostra de {} execuções", "{} futtatás mintája"]),
    ("reset nu poate fi desenat în editor (folosește CLI-ul pentru circuite dinamice)", ["reset cannot be drawn in the editor (use the CLI for dynamic circuits)", "reset ne peut pas être dessiné dans l'éditeur (utilisez la CLI pour les circuits dynamiques)", "reset kann im Editor nicht gezeichnet werden (für dynamische Schaltungen die CLI verwenden)", "reset no se puede dibujar en el editor (usa la CLI para circuitos dinámicos)", "reset non può essere disegnato nell'editor (usa la CLI per i circuiti dinamici)", "reset não pode ser desenhado no editor (use a CLI para circuitos dinâmicos)", "a reset nem rajzolható a szerkesztőben (dinamikus áramkörökhöz használja a CLI-t)"]),
    ("instrucțiunile if nu pot fi desenate în editor (folosește CLI-ul)", ["if instructions cannot be drawn in the editor (use the CLI)", "les instructions if ne peuvent pas être dessinées dans l'éditeur (utilisez la CLI)", "if-Anweisungen können im Editor nicht gezeichnet werden (die CLI verwenden)", "las instrucciones if no se pueden dibujar en el editor (usa la CLI)", "le istruzioni if non possono essere disegnate nell'editor (usa la CLI)", "as instruções if não podem ser desenhadas no editor (use a CLI)", "az if utasítások nem rajzolhatók a szerkesztőben (használja a CLI-t)"]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_entry_is_complete() {
        for (k, v) in TABLE {
            for (i, t) in v.iter().enumerate() {
                assert!(!t.trim().is_empty(), "traducere goală pentru «{k}» în {}", Lang::ALL[i + 1].code());
            }
            let ph = k.matches("{}").count();
            for (i, t) in v.iter().enumerate() {
                assert_eq!(t.matches("{}").count(), ph, "număr diferit de {{}} pentru «{k}» în {}", Lang::ALL[i + 1].code());
            }
        }
        let mut keys: Vec<_> = TABLE.iter().map(|r| r.0).collect();
        keys.sort();
        let n = keys.len();
        keys.dedup();
        assert_eq!(n, keys.len(), "chei duplicate în tabel");
    }

    /// Fiecare `t("...")`/`tf("...")` din sursa interfeței trebuie să aibă intrare în tabel.
    #[test]
    fn every_key_used_in_source_is_translated() {
        let src = include_str!("main.rs");
        let mut found = 0;
        for pat in ["tr(\"", "trf(\""] {
            let mut rest = src;
            while let Some(i) = rest.find(pat) {
                rest = &rest[i + pat.len()..];
                let end = rest.find('"').unwrap();
                let key = &rest[..end];
                assert!(TABLE.iter().any(|r| r.0 == key), "cheie netradusă: «{key}»");
                found += 1;
            }
        }
        assert!(found > 40, "prea puține chei găsite ({found}) — căutarea e stricată?");
    }

    #[test]
    fn tf_substitutes_in_order() {
        assert_eq!(Lang::En.tf("Nu pot citi {}: {}", &["a.qasm", "denied"]), "Cannot read a.qasm: denied");
        assert_eq!(Lang::Hu.t("Rulează"), "Futtatás");
        assert_eq!(Lang::Ro.t("Rulează"), "Rulează");
        assert_eq!(Lang::from_code("de_DE.UTF-8"), Some(Lang::De));
    }
}
