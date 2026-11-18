use cuantor_core::{Program, Rng, State, qasm};
use cuantor_ibm::transpile::{Basis, Target, ecr_cx_identity, native_to_program, transpile};
use cuantor_ibm::{BackendConfig, Client, Config};
use serde_json::json;
use std::io::{Read, Write};
use std::net::TcpListener;

fn line(n: usize) -> Vec<(usize, usize)> {
    (0..n - 1).map(|i| (i, i + 1)).collect()
}

/// Probabilități logice din circuitul original vs. cel transpilat (după permutarea finală).
fn check(src: &str, basis: Basis, n_phys: usize, coupling: &[(usize, usize)]) {
    let p = qasm::parse(src).unwrap();
    let t = transpile(&p, &Target { n_qubits: n_phys, coupling, basis }).unwrap();
    let mut a = State::new(p.n_qubits).unwrap();
    for g in p.gates() {
        a.apply(g).unwrap();
    }
    let np = native_to_program(&t, n_phys).unwrap();
    let mut b = State::new(n_phys).unwrap();
    for g in np.gates() {
        b.apply(g).unwrap();
    }
    let pa = a.probabilities();
    let pb = b.probabilities();
    for (i, want) in pa.iter().enumerate() {
        // index logic i → index fizic
        let mut j = 0usize;
        for l in 0..p.n_qubits {
            if (i >> l) & 1 == 1 {
                j |= 1 << t.final_map[l];
            }
        }
        assert!((pb[j] - want).abs() < 1e-9, "stare {i:b}: {} vs {} ({src})", pb[j], want);
    }
}

const HDR: &str = "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[4];\n";

#[test]
fn transpile_preserves_distribution_cx_and_cz() {
    let circuits = [
        "h q[0];\ncx q[0],q[3];\nry(0.7) q[3];\ncx q[3],q[1];\nt q[1];\nh q[1];\n",
        "h q[0];\nh q[1];\nccx q[0],q[1],q[3];\ncp(1.1) q[3],q[0];\nswap q[0],q[2];\n",
        "rx(0.3) q[0];\nry(1.9) q[1];\nrz(0.4) q[2];\ncswap q[0],q[1],q[3];\nch q[2],q[0];\ncy q[1],q[3];\ncrz(0.8) q[3],q[2];\n",
        "h q[2];\ncz q[2],q[0];\ns q[0];\nsx q[1];\ncx q[1],q[2];\n",
    ];
    for basis in [Basis::Cx, Basis::Cz] {
        for c in circuits {
            check(&format!("{HDR}{c}"), basis, 4, &line(4));
        }
    }
}

#[test]
fn transpile_random_circuits_on_a_line() {
    let mut rng = Rng::from_seed(7);
    for round in 0..30 {
        let mut p = Program::new(5);
        for _ in 0..14 {
            let a = (rng.next_u64() % 5) as usize;
            let mut b = (rng.next_u64() % 5) as usize;
            if b == a {
                b = (a + 1) % 5;
            }
            match rng.next_u64() % 5 {
                0 => {
                    p.h(a);
                }
                1 => {
                    p.rz(a, (rng.next_u64() % 628) as f64 / 100.0);
                }
                2 => {
                    p.ry(a, (rng.next_u64() % 628) as f64 / 100.0);
                }
                3 => {
                    p.cx(a, b);
                }
                _ => {
                    p.cp(a, b, (rng.next_u64() % 628) as f64 / 100.0);
                }
            }
        }
        let src = p.to_qasm();
        check(&src, if round % 2 == 0 { Basis::Cx } else { Basis::Cz }, 5, &line(5));
    }
}

#[test]
fn ecr_identity_exists_and_output_is_native() {
    assert!(ecr_cx_identity().is_some());
    let p = qasm::parse(&format!("{HDR}h q[0];\ncx q[0],q[3];\n")).unwrap();
    let t = transpile(&p, &Target { n_qubits: 4, coupling: &line(4), basis: Basis::Ecr }).unwrap();
    let q = t.to_qasm3();
    assert!(q.contains("ecr $") && !q.contains("cx $") && !q.contains("\nh "));
    assert!(t.swaps >= 1);
}

#[test]
fn rejects_dynamic_and_oversized() {
    let p = qasm::parse("OPENQASM 2.0;\nqreg q[2];\ncreg c[2];\nmeasure q[0] -> c[0];\nif(c==1) x q[1];\n").unwrap();
    assert!(transpile(&p, &Target { n_qubits: 2, coupling: &line(2), basis: Basis::Cx }).is_err());
    let big = Program::new(10);
    assert!(transpile(&big, &Target { n_qubits: 5, coupling: &line(5), basis: Basis::Cx }).is_err());
}

/// Server HTTP minimal pentru simularea IAM + Runtime.
fn mock() -> (String, std::thread::JoinHandle<Vec<String>>) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    let h = std::thread::spawn(move || {
        let mut log = vec![];
        for _ in 0..6 {
            let (mut s, _) = l.accept().unwrap();
            let mut buf = vec![0u8; 65536];
            let mut n = 0;
            loop {
                n += s.read(&mut buf[n..]).unwrap();
                let txt = String::from_utf8_lossy(&buf[..n]).to_string();
                if let Some(i) = txt.find("\r\n\r\n") {
                    let cl = txt.to_lowercase().split("content-length:").nth(1).and_then(|r| r.trim_start().split("\r\n").next().and_then(|v| v.trim().parse::<usize>().ok())).unwrap_or(0);
                    if n >= i + 4 + cl {
                        break;
                    }
                }
            }
            let req = String::from_utf8_lossy(&buf[..n]).to_string();
            let first = req.lines().next().unwrap_or("").to_string();
            let hdrs = req.to_lowercase();
            let body = if first.contains("/identity/token") {
                assert!(req.contains("grant_type=urn"));
                json!({"access_token":"T0K","expires_in":3600})
            } else {
                assert!(hdrs.contains("authorization: bearer t0k"), "lipsește Bearer: {first}");
                assert!(hdrs.contains("service-crn: crn:v1:test"), "lipsește Service-CRN");
                if first.contains("/configuration") {
                    json!({"n_qubits":5,"basis_gates":["ecr","rz","sx","x"],"coupling_map":[[0,1],[1,2],[2,3],[3,4]]})
                } else if first.starts_with("POST /api/v1/jobs") {
                    assert!(req.contains("\"program_id\":\"sampler\"") && req.contains("ecr $"));
                    json!({"id":"job-1"})
                } else if first.contains("/results") {
                    json!({"results":[{"data":{"c":{"samples":["0x0","0x3","0x3","0x0","0x3"],"num_bits":2}}}]})
                } else if first.starts_with("GET /api/v1/jobs/job-1 ") {
                    json!({"state":{"status":"Completed"}})
                } else {
                    json!({"devices":[{"name":"ibm_fake"}]})
                }
            };
            log.push(first);
            let b = body.to_string();
            write!(s, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", b.len(), b).unwrap();
        }
        log
    });
    (format!("http://127.0.0.1:{port}"), h)
}

#[test]
fn rest_flow_against_mock_server() {
    let (url, h) = mock();
    let cfg = Config { api_key: "SECRET".into(), crn: "crn:v1:test".into(), backend: "ibm_fake".into(), iam_url: url.clone(), base_url: format!("{url}/api/v1"), api_version: String::new() };
    assert!(!format!("{cfg:?}").contains("SECRET"));
    let mut c = Client::new(cfg);
    assert_eq!(c.list_backends().unwrap(), vec!["ibm_fake"]);
    let bc: BackendConfig = c.backend_config("ibm_fake").unwrap();
    assert_eq!(bc.n_qubits, 5);
    let p = qasm::parse("OPENQASM 2.0;\nqreg q[2];\nh q[0];\ncx q[0],q[1];\n").unwrap();
    let (id, t) = c.submit(&p, 100, &bc).unwrap();
    assert_eq!(id, "job-1");
    assert_eq!(t.n_clbits, 2);
    c.wait(&id, std::time::Duration::from_secs(10), |_| {}).unwrap();
    let r = c.results(&id).unwrap();
    assert_eq!(r[0], ("11".to_string(), 3));
    assert_eq!(r[1], ("00".to_string(), 2));
    let log = h.join().unwrap();
    assert_eq!(log.iter().filter(|l| l.contains("identity/token")).count(), 1, "tokenul trebuie reutilizat");
}

// ---------------------------------------------------------------- audit adversarial
#[test]
fn reversed_only_coupling_edges_still_correct() {
    let rev: Vec<(usize, usize)> = (0..3).map(|i| (i + 1, i)).collect();
    let src = format!("{HDR}h q[0];\ncx q[0],q[1];\ncx q[1],q[2];\ncx q[2],q[3];\nry(0.6) q[3];\ncx q[3],q[0];\n");
    check(&src, Basis::Cx, 4, &rev);
    // în ieșire, fiecare cx trebuie să respecte direcția permisă
    let p = qasm::parse(&src).unwrap();
    let t = transpile(&p, &Target { n_qubits: 4, coupling: &rev, basis: Basis::Cx }).unwrap();
    for g in &t.gates {
        if let cuantor_ibm::Native::Cx(a, b) = g {
            assert!(rev.contains(&(*a, *b)), "cx {a}->{b} împotriva direcției hărții");
        }
    }
}

#[test]
fn disconnected_coupling_is_an_error_not_a_panic() {
    let cm = vec![(0, 1), (2, 3)];
    let p = qasm::parse(&format!("{HDR}cx q[0],q[3];\n")).unwrap();
    assert!(transpile(&p, &Target { n_qubits: 4, coupling: &cm, basis: Basis::Cx }).is_err());
    let p = qasm::parse(&format!("{HDR}cx q[0],q[3];\n")).unwrap();
    assert!(transpile(&p, &Target { n_qubits: 4, coupling: &[], basis: Basis::Cx }).is_err());
}

#[test]
fn measurements_follow_swapped_qubits() {
    // X pe q0, apoi CX între q0 și q3 (rutat): măsurătorile trebuie mapate pe pozițiile finale
    let src = "OPENQASM 2.0;\nqreg q[4];\ncreg c[4];\nx q[0];\ncx q[0],q[3];\nmeasure q[0] -> c[0];\nmeasure q[3] -> c[3];\n";
    let p = qasm::parse(src).unwrap();
    let t = transpile(&p, &Target { n_qubits: 4, coupling: &line(4), basis: Basis::Cx }).unwrap();
    let np = native_to_program(&t, 4).unwrap();
    let mut st = State::new(4).unwrap();
    for g in np.gates() {
        st.apply(g).unwrap();
    }
    for (phys, clbit) in &t.measures {
        assert!(st.prob_one(*phys) > 0.999, "bit clasic {clbit} (fizic {phys}) ar trebui să fie 1");
    }
    assert_eq!(t.measures.len(), 2);
}

#[test]
fn hostile_rest_responses_do_not_panic() {
    use cuantor_ibm::client::decode_results;
    for v in [
        json!({}),
        json!({"results":[]}),
        json!({"results":[{"data":{}}]}),
        json!({"results":[{"data":{"c":{"samples":"x"}}}]}),
        json!({"results":[{"data":{"c":{"samples":[1,2],"num_bits":2}}}]}),
        json!({"results":[{"data":{"c":{"samples":["0xZZ"],"num_bits":2}}}]}),
        json!({"results":[{"data":{"c":{"samples":[],"num_bits":2}}}]}),
        json!(null),
    ] {
        let _ = std::panic::catch_unwind(|| decode_results(&v)).expect("panic la răspuns ostil");
    }
    // eșantion gol → zero rânduri, nu eroare/panic
    let ok = decode_results(&json!({"results":[{"data":{"c":{"samples":["0x1"],"num_bits":70}}}]})).unwrap();
    assert_eq!(ok[0].0.len(), 70, "padding la num_bits");
    assert!(BackendConfig::from_json("x", &json!({"n_qubits": "5"})).is_err());
    assert!(BackendConfig::from_json("x", &json!({"n_qubits": 5, "coupling_map": [[0], "a", [1,2]]})).is_ok());
}

#[test]
fn api_key_never_appears_in_errors() {
    // server care răspunde 401 cu un corp care NU conține cheia; clientul nu are voie să o adauge
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = l.local_addr().unwrap().port();
    std::thread::spawn(move || {
        if let Ok((mut s, _)) = l.accept() {
            let mut b = [0u8; 4096];
            let _ = s.read(&mut b);
            let body = "{\"error\":\"bad key\"}";
            let _ = write!(s, "HTTP/1.1 401 Unauthorized\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
        }
    });
    let url = format!("http://127.0.0.1:{port}");
    let cfg = Config { api_key: "SUPERSECRETKEY123".into(), crn: "crn:x".into(), backend: "b".into(), iam_url: url.clone(), base_url: url, api_version: String::new() };
    let mut c = Client::new(cfg);
    let e = c.list_backends().unwrap_err();
    assert!(!e.contains("SUPERSECRETKEY123"), "cheia apare în mesajul de eroare: {e}");
    assert!(e.contains("401"));
}

#[test]
fn large_circuits_transpile_in_reasonable_time() {
    let mut s = String::from("OPENQASM 2.0;\nqreg q[60];\nh q[0];\n");
    for i in 0..59 {
        s.push_str(&format!("cx q[{i}],q[{}];\nrz(0.1) q[{i}];\n", i + 1));
    }
    s.push_str("cx q[0],q[59];\ncx q[10],q[50];\n");
    let p = qasm::parse(&s).unwrap();
    let t0 = std::time::Instant::now();
    let t = transpile(&p, &Target { n_qubits: 127, coupling: &line(127), basis: Basis::Ecr }).unwrap();
    assert!(t0.elapsed().as_secs() < 10, "transpilare lentă: {:?}", t0.elapsed());
    assert!(t.swaps > 50);
}
