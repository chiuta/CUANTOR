//! CUANTOR în linia de comandă: rulează circuite OpenQASM pe mașina virtuală cuantică.

use cuantor_core::examples::EXAMPLES;
use cuantor_core::{Backend, NoiseModel, analyze, bench_h_all, choose_backend, qasm, sample_with};

const LANGS: [&str; 8] = ["ro", "en", "fr", "de", "es", "it", "pt", "hu"];

fn lang() -> usize {
    let pick = |v: &str| LANGS.iter().position(|l| v.to_lowercase().starts_with(l));
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|x| x == "--lang") {
        if let Some(l) = args.get(i + 1).and_then(|v| pick(v)) {
            return l;
        }
    }
    ["CUANTOR_LANG", "LANGUAGE", "LC_ALL", "LANG"].iter().find_map(|v| std::env::var(v).ok().and_then(|x| pick(&x))).unwrap_or(1)
}

/// (comandă, descrieri în ordinea RO EN FR DE ES IT PT HU)
const HELP_LINES: &[(&str, [&str; 8])] = &[
    ("run <file.qasm> [--shots N] [--seed S] [--backend auto|sv|stab|sparse] [--depol1 P] [--depol2 P] [--readout P] [--top K]", ["rulează circuitul și afișează histograma", "run the circuit and print the histogram", "exécute le circuit et affiche l'histogramme", "Schaltung ausführen und Histogramm ausgeben", "ejecuta el circuito e imprime el histograma", "esegue il circuito e stampa l'istogramma", "executa o circuito e imprime o histograma", "futtatja az áramkört és kiírja a hisztogramot"]),
    ("state <file.qasm> [--backend ...] [--top K]", ["amplitudini / stabilizatori", "amplitudes / stabilizers", "amplitudes / stabilisateurs", "Amplituden / Stabilisatoren", "amplitudes / estabilizadores", "ampiezze / stabilizzatori", "amplitudes / estabilizadores", "amplitúdók / stabilizátorok"]),
    ("convert <file.qasm> --to qasm2|qasm3|qiskit", ["conversie de format", "format conversion", "conversion de format", "Formatkonvertierung", "conversión de formato", "conversione di formato", "conversão de formato", "formátumkonverzió"]),
    ("examples", ["lista exemplelor", "list the examples", "liste des exemples", "Beispiele auflisten", "lista de ejemplos", "elenco degli esempi", "lista de exemplos", "példák listája"]),
    ("example <n|title> [--shots N]", ["rulează un exemplu", "run an example", "exécute un exemple", "ein Beispiel ausführen", "ejecuta un ejemplo", "esegue un esempio", "executa um exemplo", "példa futtatása"]),
    ("bench [N]", ["test de viteză (H pe toți qubiții)", "speed test (H on every qubit)", "test de vitesse (H sur chaque qubit)", "Geschwindigkeitstest (H auf jedem Qubit)", "prueba de velocidad (H en cada cúbit)", "test di velocità (H su ogni qubit)", "teste de velocidade (H em cada qubit)", "sebességteszt (H minden qubiten)"]),
    ("ghz <N> [--shots N]", ["GHZ pe N qubiți (până la 4096)", "GHZ on N qubits (up to 4096)", "GHZ sur N qubits (jusqu'à 4096)", "GHZ auf N Qubits (bis 4096)", "GHZ en N cúbits (hasta 4096)", "GHZ su N qubit (fino a 4096)", "GHZ em N qubits (até 4096)", "GHZ N qubiten (legfeljebb 4096)"]),
    ("ibm backends", ["backend-uri IBM disponibile", "available IBM backends", "backends IBM disponibles", "verfügbare IBM-Backends", "backends de IBM disponibles", "backend IBM disponibili", "backends IBM disponíveis", "elérhető IBM backendek"]),
    ("ibm run <file.qasm> [--backend NAME] [--shots N] [--dry]", ["trimite pe hardware IBM real (--dry: doar circuitul ISA)", "run on real IBM hardware (--dry: only print the ISA circuit)", "exécute sur du matériel IBM réel (--dry : circuit ISA seulement)", "auf echter IBM-Hardware ausführen (--dry: nur ISA-Schaltung)", "ejecuta en hardware real de IBM (--dry: solo el circuito ISA)", "esegue su hardware IBM reale (--dry: solo il circuito ISA)", "executa em hardware IBM real (--dry: só o circuito ISA)", "valódi IBM hardveren futtat (--dry: csak az ISA áramkör)"]),
    ("ibm open <file.qasm>", ["salvează .qasm și deschide IBM Composer", "save .qasm and open IBM Composer", "enregistre .qasm et ouvre IBM Composer", ".qasm speichern und IBM Composer öffnen", "guarda .qasm y abre IBM Composer", "salva .qasm e apre IBM Composer", "guarda .qasm e abre o IBM Composer", ".qasm mentése és az IBM Composer megnyitása"]),
];

const TITLE: [&str; 8] = [
    "CUANTOR — computer cuantic virtual (simulator)",
    "CUANTOR — virtual quantum computer (simulator)",
    "CUANTOR — ordinateur quantique virtuel (simulateur)",
    "CUANTOR — virtueller Quantencomputer (Simulator)",
    "CUANTOR — computador cuántico virtual (simulador)",
    "CUANTOR — computer quantistico virtuale (simulatore)",
    "CUANTOR — computador quântico virtual (simulador)",
    "CUANTOR — virtuális kvantumszámítógép (szimulátor)",
];
const USAGE: [&str; 8] = ["UTILIZARE", "USAGE", "UTILISATION", "VERWENDUNG", "USO", "USO", "UTILIZAÇÃO", "HASZNÁLAT"];
const ERR: [&str; 8] = ["eroare", "error", "erreur", "Fehler", "error", "errore", "erro", "hiba"];
const ENVS: [&str; 8] = [
    "variabile IBM: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; limbă: --lang xx sau CUANTOR_LANG",
    "IBM variables: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; language: --lang xx or CUANTOR_LANG",
    "variables IBM : IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND ; langue : --lang xx ou CUANTOR_LANG",
    "IBM-Variablen: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; Sprache: --lang xx oder CUANTOR_LANG",
    "variables de IBM: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; idioma: --lang xx o CUANTOR_LANG",
    "variabili IBM: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; lingua: --lang xx o CUANTOR_LANG",
    "variáveis IBM: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; idioma: --lang xx ou CUANTOR_LANG",
    "IBM változók: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND; nyelv: --lang xx vagy CUANTOR_LANG",
];

fn help() -> String {
    let l = lang();
    let mut s = format!("{}\n\n{}:\n", TITLE[l], USAGE[l]);
    for (cmd, d) in HELP_LINES {
        s.push_str(&format!("  cuantor {cmd}\n      {}\n", d[l]));
    }
    s.push_str(&format!("\n{}\n", ENVS[l]));
    s
}

fn flag<'a>(a: &'a [String], name: &str) -> Option<&'a str> {
    a.iter().position(|x| x == name).and_then(|i| a.get(i + 1)).map(|s| s.as_str())
}
fn num<T: std::str::FromStr>(a: &[String], name: &str, d: T) -> T {
    match flag(a, name) {
        None => d,
        Some(s) => s.parse().unwrap_or_else(|_| {
            eprintln!("eroare: valoare invalidă pentru {name}: «{s}»");
            std::process::exit(2)
        }),
    }
}
fn backend(a: &[String]) -> Backend {
    match flag(a, "--backend").unwrap_or("auto") {
        "sv" | "statevector" => Backend::StateVector,
        "stab" | "stabilizer" => Backend::Stabilizer,
        "sparse" => Backend::Sparse,
        "auto" => Backend::Auto,
        other => {
            eprintln!("eroare: motor necunoscut «{other}» (auto|sv|stab|sparse)");
            std::process::exit(2)
        }
    }
}
fn noise(a: &[String]) -> NoiseModel {
    NoiseModel { depol1: num(a, "--depol1", 0.0), depol2: num(a, "--depol2", 0.0), readout: num(a, "--readout", 0.0) }
}

fn run_text(text: &str, a: &[String]) -> Result<(), String> {
    let p = qasm::parse(text).map_err(|e| e.0)?;
    let req = backend(a);
    let be = choose_backend(&p, req).map_err(|e| e.0)?;
    let shots: usize = num(a, "--shots", 1024);
    let top: usize = num(a, "--top", 16);
    let seed: Option<u64> = flag(a, "--seed").and_then(|s| s.parse().ok());
    let t0 = std::time::Instant::now();
    let c = sample_with(&p, shots, &noise(a), seed, req).map_err(|e| e.0)?;
    println!("qubiți: {} · motor: {} · măsurători: {} · {:.3}s", p.n_qubits, be.name(), shots, t0.elapsed().as_secs_f64());
    for (k, n) in c.sorted().into_iter().take(top) {
        let f = n as f64 / shots as f64;
        println!("{}  {:>7}  {:6.2}%  {}", c.bitstring(&k), n, f * 100.0, "█".repeat((f * 40.0).round() as usize));
    }
    if c.map.len() > top {
        println!("… încă {} rezultate distincte", c.map.len() - top);
    }
    Ok(())
}

fn state_text(text: &str, a: &[String]) -> Result<(), String> {
    let p = qasm::parse(text).map_err(|e| e.0)?;
    let r = analyze(&p, backend(a), num(a, "--top", 16), 2048, flag(a, "--seed").and_then(|s| s.parse().ok())).map_err(|e| e.0)?;
    println!("motor: {} · qubiți: {} · memorie ≈ {} B", r.backend.name(), r.n, r.memory_bytes);
    for s in r.stabilizers.iter().take(20) {
        println!("  {s}");
    }
    for t in &r.top {
        match t.amp {
            Some(z) => println!("|{}>  {:+.5}{:+.5}i  {:7.3}%", t.bits, z.re, z.im, t.prob * 100.0),
            None => println!("|{}>  ≈ {:7.3}%", t.bits, t.prob * 100.0),
        }
    }
    Ok(())
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let cmd = a.first().map(|s| s.as_str()).unwrap_or("help");
    let file_text = |i: usize| -> Result<String, String> {
        let p = a.get(i).ok_or("lipsește fișierul")?;
        std::fs::read_to_string(p).map_err(|e| format!("nu pot citi {p}: {e}"))
    };
    let res: Result<(), String> = match cmd {
        "run" => file_text(1).and_then(|t| run_text(&t, &a)),
        "state" => file_text(1).and_then(|t| state_text(&t, &a)),
        "convert" => file_text(1).and_then(|t| {
            let p = qasm::parse(&t).map_err(|e| e.0)?;
            match flag(&a, "--to").unwrap_or("qasm3") {
                "qasm2" => print!("{}", p.to_qasm()),
                "qiskit" => print!("{}", p.to_qiskit()),
                _ => print!("{}", p.to_qasm3()),
            }
            Ok(())
        }),
        "examples" => {
            for (i, e) in EXAMPLES.iter().enumerate() {
                println!("{:>2}. [{}] {} — {}", i + 1, e.group, e.title, e.desc);
            }
            Ok(())
        }
        "example" => a
            .get(1)
            .and_then(|k| k.parse::<usize>().ok().and_then(|i| EXAMPLES.get(i.wrapping_sub(1))).or_else(|| EXAMPLES.iter().find(|e| e.title.to_lowercase().contains(&k.to_lowercase()))))
            .ok_or_else(|| "exemplu necunoscut (vezi `cuantor examples`)".to_string())
            .and_then(|e| {
                println!("{}\n{}\n", e.title, e.qasm);
                run_text(e.qasm, &a)
            }),
        "bench" => {
            let n: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(24);
            bench_h_all(n).map(|s| println!("H pe {n} qubiți: {s:.3}s ({} fire)", cuantor_core::threads())).map_err(|e| e.0)
        }
        "ghz" => {
            let n: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(100);
            let mut t = format!("OPENQASM 2.0;\nqreg q[{n}];\nh q[0];\n");
            for i in 0..n.saturating_sub(1) {
                t.push_str(&format!("cx q[{i}],q[{}];\n", i + 1));
            }
            run_text(&t, &a)
        }
        "ibm" => ibm(&a),
        _ => {
            print!("{}", help());
            Ok(())
        }
    };
    if let Err(e) = res {
        eprintln!("{}: {e}", ERR[lang()]);
        std::process::exit(1);
    }
}

fn ibm(a: &[String]) -> Result<(), String> {
    use cuantor_ibm::{Client, Config};
    match a.get(1).map(|s| s.as_str()) {
        Some("open") => {
            let path = a.get(2).ok_or("lipsește fișierul")?;
            let p = qasm::parse(&std::fs::read_to_string(path).map_err(|e| e.to_string())?).map_err(|e| e.0)?;
            let out = std::env::temp_dir().join("cuantor_composer.qasm");
            std::fs::write(&out, p.to_qasm3()).map_err(|e| e.to_string())?;
            println!("Salvat {} — în Composer: File → Upload .qasm file", out.display());
            println!("https://quantum.cloud.ibm.com/composer");
            Ok(())
        }
        Some("backends") => {
            let mut c = Client::new(Config::load()?);
            for b in c.list_backends()? {
                println!("{b}");
            }
            Ok(())
        }
        Some("run") => {
            let path = a.get(2).ok_or("lipsește fișierul")?;
            let p = qasm::parse(&std::fs::read_to_string(path).map_err(|e| e.to_string())?).map_err(|e| e.0)?;
            let shots: usize = num(a, "--shots", 1024);
            let cfg = Config::load()?;
            let name = flag(a, "--backend").map(String::from).filter(|s| !s.is_empty()).or_else(|| Some(cfg.backend.clone()).filter(|s| !s.is_empty())).ok_or("alege backend: --backend NUME sau IBM_QUANTUM_BACKEND")?;
            let mut c = Client::new(cfg);
            let bc = c.backend_config(&name)?;
            if a.iter().any(|x| x == "--dry") {
                let t = cuantor_ibm::transpile(&p, &bc.target())?;
                println!("{}\n// porți 2q: {} · swap-uri: {}", t.to_qasm3(), t.two_qubit_count(), t.swaps);
                return Ok(());
            }
            let (id, t) = c.submit(&p, shots, &bc)?;
            println!("job {id} trimis pe {name} ({} porți 2q, {} swap-uri)", t.two_qubit_count(), t.swaps);
            c.wait(&id, std::time::Duration::from_secs(3600), |s| eprintln!("stare: {s}"))?;
            let r = c.results(&id)?;
            let sim = sample_with(&p, shots, &NoiseModel::default(), None, Backend::Auto).ok();
            for (k, n) in r.iter().take(num(a, "--top", 16)) {
                let f = *n as f64 / shots as f64;
                let ideal = sim.as_ref().map(|s| s.sorted().into_iter().find(|(b, _)| s.bitstring(b) == *k).map(|(_, n)| n as f64 / s.shots as f64).unwrap_or(0.0)).unwrap_or(f64::NAN);
                println!("{k}  {n:>7}  hardware {:6.2}%  simulare ideală {:6.2}%", f * 100.0, ideal * 100.0);
            }
            Ok(())
        }
        _ => Err("folosește: cuantor ibm backends | run <fisier> | open <fisier>".into()),
    }
}
