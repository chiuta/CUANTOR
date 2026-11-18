//! Audit adversarial: intrări ostile, diferențiale între motoare, statistici, reproductibilitate.
use cuantor_core::*;
use std::time::{Duration, Instant};

fn hdr(n: usize) -> String {
    format!("OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[{n}];\ncreg c[{n}];\n")
}

fn within<T>(secs: u64, f: impl FnOnce() -> T) -> T {
    let t0 = Instant::now();
    let r = f();
    assert!(t0.elapsed() < Duration::from_secs(secs), "prea lent: {:?}", t0.elapsed());
    r
}

// ---------------------------------------------------------------- parser ostil
#[test]
fn parser_never_panics_on_garbage() {
    let mut rng = Rng::from_seed(1);
    let toks = ["h", "cx", "q", "[", "]", "(", ")", ";", ",", "pi", "1e999", "-", "*", "/", "gate", "{", "}", "if", "==", "measure", "->", "qreg", "creg", "OPENQASM", "2.0", "3.0", "qubit", "bit", "c", "0", "99999999999999999999", "rz", "//", "/*", "*/", "\n", "\"", "include", "reset", "barrier", "swap", "u3", "ccx", "x", "=", "$0", "NaN", "inf", "ø", "😀"];
    for _ in 0..4000 {
        let len = (rng.next_u64() % 40) as usize;
        let mut s = String::new();
        for _ in 0..len {
            s.push_str(toks[(rng.next_u64() % toks.len() as u64) as usize]);
            s.push(' ');
        }
        let r = std::panic::catch_unwind(|| qasm::parse(&s));
        assert!(r.is_ok(), "PANIC la: {s:?}");
    }
}

#[test]
fn parser_mutations_of_valid_programs_never_panic() {
    let mut rng = Rng::from_seed(2);
    for ex in examples::EXAMPLES {
        for _ in 0..60 {
            let mut b = ex.qasm.as_bytes().to_vec();
            for _ in 0..1 + rng.next_u64() % 4 {
                if b.is_empty() {
                    break;
                }
                let i = (rng.next_u64() % b.len() as u64) as usize;
                match rng.next_u64() % 3 {
                    0 => b[i] = (rng.next_u64() % 128) as u8,
                    1 => {
                        b.remove(i);
                    }
                    _ => b.insert(i, b"[](){};,"[(rng.next_u64() % 8) as usize]),
                }
            }
            let s = String::from_utf8_lossy(&b).to_string();
            assert!(std::panic::catch_unwind(|| qasm::parse(&s)).is_ok(), "PANIC: {s:?}");
        }
    }
}

#[test]
fn bomb_huge_qreg() {
    within(5, || assert!(qasm::parse(&hdr(100_000_000)).is_err()));
}
#[test]
fn bomb_4096_qubits_dense_rejected() {
    let p = qasm::parse("OPENQASM 2.0;\nqreg q[4000];\nh q[0];\nt q[0];\n").unwrap();
    within(10, || {
        let r = sample_with(&p, 4, &NoiseModel::default(), Some(1), Backend::StateVector);
        assert!(r.is_err(), "4000 qubiți pe vector de stare trebuie refuzați, nu alocați");
    });
}
#[test]
fn bomb_state_new_40() {
    assert!(State::new(40).is_err());
    let _ = std::panic::catch_unwind(|| State::new(0)).expect("panic la State::new(0)");
}
#[test]
fn bomb_recursive_gate_defs() {
    let rec = "OPENQASM 2.0;\nqreg q[1];\ngate a x { b x; }\ngate b x { a x; }\na q[0];\n";
    within(5, || {
        let _ = std::panic::catch_unwind(|| qasm::parse(rec)).expect("panic/stack overflow la recursie");
    });
}
#[test]
fn bomb_exponential_gate_expansion() {
    let mut s = String::from("OPENQASM 2.0;\nqreg q[1];\ngate g0 a { h a; }\n");
    for i in 1..40 {
        s.push_str(&format!("gate g{i} a {{ g{} a; g{} a; }}\n", i - 1, i - 1));
    }
    s.push_str("g39 q[0];\n");
    within(10, || {
        let r = std::panic::catch_unwind(|| qasm::parse(&s)).expect("panic la expansiune exponențială");
        assert!(r.is_err(), "2^39 porți trebuie refuzate");
    });
}
#[test]
fn bomb_deep_expression_nesting() {
    let deep = format!("OPENQASM 2.0;\nqreg q[1];\nrz({}1{}) q[0];\n", "(".repeat(50_000), ")".repeat(50_000));
    within(10, || {
        let _ = std::panic::catch_unwind(|| qasm::parse(&deep)).expect("panic/stack overflow la expresie imbricată");
    });
}

#[test]
fn nan_and_inf_angles_do_not_poison_state() {
    for a in ["1e999", "-1e999", "0/0", "sqrt(-1)", "ln(0)", "1e308*10"] {
        let src = format!("{}h q[0];\nrz({a}) q[0];\n", hdr(2));
        if let Ok(p) = qasm::parse(&src) {
            let r = analyze(&p, Backend::Auto, 4, 64, Some(1));
            if let Ok(an) = r {
                for t in &an.top {
                    assert!(t.prob.is_finite(), "probabilitate non-finită pentru unghi {a}");
                }
            }
        }
    }
}

#[test]
fn out_of_range_and_duplicate_qubits_rejected() {
    for bad in ["cx q[0],q[0];", "cx q[0],q[5];", "h q[2];", "ccx q[0],q[1],q[1];", "swap q[0],q[0];", "measure q[0] -> c[9];"] {
        let src = format!("{}{bad}\n", hdr(2));
        match qasm::parse(&src) {
            Err(_) => {}
            Ok(p) => assert!(p.check().is_err(), "acceptat greșit: {bad}"),
        }
    }
}

// ---------------------------------------------------------------- diferențiale între motoare
fn random_circuit(rng: &mut Rng, n: usize, depth: usize, clifford: bool) -> Program {
    let mut p = Program::new(n);
    for _ in 0..depth {
        let a = (rng.next_u64() % n as u64) as usize;
        let mut b = (rng.next_u64() % n as u64) as usize;
        if b == a {
            b = (a + 1) % n;
        }
        let k = rng.next_u64() % if clifford { 6 } else { 10 };
        match k {
            0 => {
                p.h(a);
            }
            1 => {
                p.push(Op::single(Kind::S, a));
            }
            2 => {
                p.cx(a, b);
            }
            3 => {
                p.cz(a, b);
            }
            4 => {
                p.x(a);
            }
            5 => {
                p.swap(a, b);
            }
            6 => {
                p.push(Op::single(Kind::T, a));
            }
            7 => {
                p.rz(a, (rng.next_u64() % 1000) as f64 / 159.0);
            }
            8 => {
                p.ry(a, (rng.next_u64() % 1000) as f64 / 159.0);
            }
            _ => {
                p.cp(a, b, (rng.next_u64() % 1000) as f64 / 159.0);
            }
        }
    }
    p
}

fn dist(p: &Program, be: Backend, n: usize) -> Vec<f64> {
    let an = analyze(p, be, 1 << n, 4096, Some(3)).unwrap();
    let mut v = vec![0.0; 1 << n];
    for t in &an.top {
        v[usize::from_str_radix(&t.bits, 2).unwrap()] = t.prob;
    }
    v
}

#[test]
fn differential_statevector_vs_sparse_general_circuits() {
    let mut rng = Rng::from_seed(11);
    for _ in 0..60 {
        let n = 2 + (rng.next_u64() % 6) as usize;
        let p = random_circuit(&mut rng, n, 40, false);
        let (a, b) = (dist(&p, Backend::StateVector, n), dist(&p, Backend::Sparse, n));
        for i in 0..a.len() {
            assert!((a[i] - b[i]).abs() < 1e-9, "SV≠Sparse la {i:b}: {} vs {}", a[i], b[i]);
        }
        assert!((a.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }
}

#[test]
fn differential_statevector_vs_stabilizer_clifford() {
    let mut rng = Rng::from_seed(12);
    for _ in 0..80 {
        let n = 2 + (rng.next_u64() % 7) as usize;
        let p = random_circuit(&mut rng, n, 60, true);
        let a = dist(&p, Backend::StateVector, n);
        // stabilizatorul: compară marginalele p(1) exacte
        let st = analyze(&p, Backend::Stabilizer, 1, 20000, Some(5)).unwrap();
        for q in 0..n {
            let exact: f64 = a.iter().enumerate().filter(|(i, _)| (i >> q) & 1 == 1).map(|(_, x)| x).sum();
            assert!((st.p_one[q] - exact).abs() < 1e-9, "p1[{q}] stabilizator {} vs {}", st.p_one[q], exact);
        }
    }
}

#[test]
fn stabilizer_rejects_non_clifford_loudly() {
    let p = qasm::parse(&format!("{}t q[0];\n", hdr(2))).unwrap();
    assert!(analyze(&p, Backend::Stabilizer, 1, 8, None).is_err());
}

#[test]
fn state_stays_normalized_after_many_gates() {
    let mut rng = Rng::from_seed(13);
    let p = random_circuit(&mut rng, 8, 20_000, false);
    let mut st = State::new(8).unwrap();
    for g in p.gates() {
        st.apply(g).unwrap();
    }
    assert!((st.norm_sqr() - 1.0).abs() < 1e-8, "derivă de normă: {}", st.norm_sqr());
}

// ---------------------------------------------------------------- statistică și reproductibilitate
#[test]
fn sampling_matches_distribution_chi_square() {
    let p = qasm::parse(&format!("{}ry(1.0) q[0];\nh q[1];\ncx q[1],q[2];\nrx(0.4) q[0];\n", hdr(3))).unwrap();
    let shots = 200_000;
    let c = sample_with(&p, shots, &NoiseModel::default(), Some(9), Backend::StateVector).unwrap();
    let ex = dist(&p.with_measure_all(), Backend::StateVector, 3);
    let mut chi = 0.0;
    let mut dof = 0;
    for i in 0..8 {
        let obs = c.freq(i as u64) * shots as f64;
        let e = ex[i] * shots as f64;
        if e > 5.0 {
            chi += (obs - e).powi(2) / e;
            dof += 1;
        }
    }
    assert!(chi < 30.0 + 4.0 * dof as f64, "chi²={chi} (gdl {dof}): eșantionare părtinitoare");
}

#[test]
fn seed_reproducible_and_thread_independent() {
    let p = qasm::parse(&format!("{}h q[0];\nh q[1];\ncx q[1],q[2];\nt q[0];\nh q[0];\n", hdr(3))).unwrap();
    let noise = NoiseModel { depol1: 0.01, depol2: 0.02, readout: 0.01 };
    set_threads(1);
    let a = sample_with(&p, 3000, &noise, Some(42), Backend::Auto).unwrap();
    let b = sample_with(&p, 3000, &noise, Some(42), Backend::Auto).unwrap();
    set_threads(4);
    let c = sample_with(&p, 3000, &noise, Some(42), Backend::Auto).unwrap();
    assert_eq!(a.map, b.map, "aceeași sămânță dă rezultate diferite");
    assert_eq!(a.map, c.map, "rezultatul depinde de numărul de fire");
}

#[test]
fn edge_shots_and_empty_programs() {
    let p = Program::new(3);
    let c = sample_with(&p, 10, &NoiseModel::default(), Some(1), Backend::Auto);
    assert!(c.is_ok());
    let p = qasm::parse(&format!("{}h q[0];\n", hdr(1))).unwrap();
    for shots in [0usize, 1] {
        let _ = std::panic::catch_unwind(|| sample_with(&p, shots, &NoiseModel::default(), Some(1), Backend::Auto)).expect("panic la shots limită");
    }
    let bad = NoiseModel { depol1: 5.0, depol2: -1.0, readout: f64::NAN };
    let _ = std::panic::catch_unwind(|| sample_with(&p, 10, &bad, Some(1), Backend::Auto)).expect("panic la zgomot invalid");
}

#[test]
fn dynamic_circuit_semantics() {
    // teleportare reală: starea Ry(1.1)|0> ajunge pe q2
    let src = "OPENQASM 2.0;\nqreg q[3];\ncreg a[1];\ncreg b[1];\ncreg r[1];\nry(1.1) q[0];\nh q[1];\ncx q[1],q[2];\ncx q[0],q[1];\nh q[0];\nmeasure q[0] -> a[0];\nmeasure q[1] -> b[0];\nif(b==1) x q[2];\nif(a==1) z q[2];\nry(-1.1) q[2];\nmeasure q[2] -> r[0];\n";
    let p = qasm::parse(src).unwrap();
    let c = sample_with(&p, 2000, &NoiseModel::default(), Some(4), Backend::Auto).unwrap();
    for (k, _) in c.sorted() {
        assert!(!c.bitstring(&k).starts_with('1'), "teleportarea a eșuat: r=1 în {}", c.bitstring(&k));
    }
}

#[test]
fn wide_stabilizer_correlations_at_1000_qubits() {
    let n = 1000;
    let mut s = format!("OPENQASM 2.0;\nqreg q[{n}];\nh q[0];\n");
    for i in 0..n - 1 {
        s.push_str(&format!("cx q[{i}],q[{}];\n", i + 1));
    }
    let p = qasm::parse(&s).unwrap();
    let c = within(30, || sample_with(&p, 30, &NoiseModel::default(), Some(8), Backend::Auto).unwrap());
    let mut zeros = 0;
    for (k, m) in c.sorted() {
        let b = c.bitstring(&k);
        assert!(b.chars().all(|x| x == b.chars().next().unwrap()), "GHZ necorelat");
        if b.starts_with('0') {
            zeros += m;
        }
    }
    assert!(zeros <= 30);
}

#[test]
fn invalid_noise_probabilities_are_rejected() {
    let p = qasm::parse(&format!("{}h q[0];\n", hdr(1))).unwrap();
    for bad in [NoiseModel { depol1: 9.0, depol2: 0.0, readout: 0.0 }, NoiseModel { depol1: 0.0, depol2: -0.1, readout: 0.0 }, NoiseModel { depol1: 0.0, depol2: 0.0, readout: f64::NAN }] {
        assert!(sample_with(&p, 10, &bad, Some(1), Backend::Auto).is_err());
    }
}
