use cuantor_core::*;
use std::f64::consts::PI;

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}
fn probs(q: &str) -> Vec<f64> {
    let p = qasm::parse(q).unwrap_or_else(|e| panic!("{e}: {q}"));
    final_state(&p).unwrap().probabilities()
}
fn h(n: usize) -> String {
    format!("OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[{n}];\ncreg c[{n}];\n")
}

#[test]
fn single_qubit_basics() {
    let p = probs(&(h(1) + "h q[0];"));
    assert!(near(p[0], 0.5) && near(p[1], 0.5));
    let p = probs(&(h(1) + "ry(pi/6) q[0];"));
    assert!((p[0] - 0.9330127).abs() < 1e-6);
    let p = probs(&(h(1) + "h q[0]; ry(pi/2) q[0];"));
    assert!(near(p[1], 1.0));
    for phi in [0.3f64, 1.7, 3.0] {
        let p = probs(&(h(1) + &format!("h q[0]; rz({phi}) q[0]; h q[0];")));
        assert!(near(p[0], (phi / 2.0).cos().powi(2)));
    }
}

#[test]
fn lesson_exercises() {
    let p = probs(&(h(2) + "ry(pi/3) q[0]; ry(2*acos(sqrt(0.6))) q[1];"));
    for (a, b) in p.iter().zip([0.45, 0.15, 0.30, 0.10]) {
        assert!(near(*a, b));
    }
    let p = probs(&(h(2) + "ry(pi/3) q[0]; ry(2*acos(sqrt(0.6))) q[1]; swap q[0],q[1];"));
    for (a, b) in p.iter().zip([0.45, 0.30, 0.15, 0.10]) {
        assert!(near(*a, b));
    }
    let p = probs(&(h(2) + "x q[0]; cx q[0],q[1];"));
    assert!(near(p[3], 1.0));
    let p = probs(&(h(2) + "h q[0]; ry(pi) q[1]; cx q[0],q[1];"));
    assert!(near(p[1], 0.5) && near(p[2], 0.5));
}

#[test]
fn algorithms() {
    let p = probs(&(h(3) + "h q[0]; cx q[0],q[1]; cx q[1],q[2];"));
    assert!(near(p[0], 0.5) && near(p[7], 0.5));
    let p = probs(&(h(2) + "h q[0];h q[1];cz q[0],q[1];h q[0];h q[1];x q[0];x q[1];cz q[0],q[1];x q[0];x q[1];h q[0];h q[1];"));
    assert!(near(p[3], 1.0));
    let ex = &examples::EXAMPLES;
    // toate exemplele statice se parsează
    for e in ex.iter() {
        qasm::parse(e.qasm).unwrap_or_else(|er| panic!("{}: {er}", e.title));
    }
    // QFT |5>
    let prog = qasm::parse(&(h(3) + "x q[0];x q[2];h q[2];cu1(pi/2) q[1],q[2];cu1(pi/4) q[0],q[2];h q[1];cu1(pi/2) q[0],q[1];h q[0];swap q[0],q[2];")).unwrap();
    let st = final_state(&prog).unwrap();
    for y in 0..8 {
        let ph = 2.0 * PI * 5.0 * y as f64 / 8.0;
        let a = st.amplitudes()[y];
        assert!(near(a.re, ph.cos() / 8f64.sqrt()) && near(a.im, ph.sin() / 8f64.sqrt()));
    }
}

#[test]
fn bloch_and_entanglement() {
    let st = final_state(&qasm::parse(&(h(2) + "h q[0]; cx q[0],q[1];")).unwrap()).unwrap();
    let b = st.bloch(0);
    assert!((b[0] * b[0] + b[1] * b[1] + b[2] * b[2]).sqrt() < 1e-9);
    assert!(near(bloch_entropy(0.0), 1.0));
    let st = final_state(&qasm::parse(&(h(1) + "rx(pi/2) q[0];")).unwrap()).unwrap();
    assert!(near(st.bloch(0)[1], -1.0));
    // ⟨ZZ⟩ = 1 pe Bell
    let st = final_state(&qasm::parse(&(h(2) + "h q[0]; cx q[0],q[1];")).unwrap()).unwrap();
    assert!(near(st.expect_pauli(&[(0, 'Z'), (1, 'Z')]).unwrap(), 1.0));
    assert!(near(st.expect_pauli(&[(0, 'X'), (1, 'X')]).unwrap(), 1.0));
}

#[test]
fn u3_and_controlled_phase_exact() {
    // U3(pi,0,pi) = X exact (inclusiv faza globală)
    let m = Kind::U(PI, 0.0, PI).matrix();
    assert!(near(m[0][1].re, 1.0) && near(m[1][0].re, 1.0) && m[0][0].norm_sqr() < 1e-18);
    // Sx*Sx = X
    let s = Kind::Sx.matrix();
    let x00 = s[0][0] * s[0][0] + s[0][1] * s[1][0];
    let x01 = s[0][0] * s[0][1] + s[0][1] * s[1][1];
    assert!(x00.norm_sqr() < 1e-18 && near(x01.re, 1.0));
}

#[test]
fn qasm_roundtrip_and_errors() {
    let src = h(3) + "h q[0];cu1(pi/3) q[0],q[2];ry(1.234567) q[1];ccx q[0],q[1],q[2];swap q[0],q[1];cswap q[0],q[1],q[2];measure q[0] -> c[0];";
    let a = qasm::parse(&src).unwrap();
    let b = qasm::parse(&a.to_qasm()).unwrap();
    assert_eq!(a.instrs.len(), b.instrs.len());
    for (x, y) in a.gates().zip(b.gates()) {
        assert_eq!(x.controls, y.controls);
        assert_eq!(x.targets, y.targets);
        assert_eq!(x.label(), y.label());
    }
    assert!(qasm::parse(&(h(2) + "foo q[0];")).is_err());
    assert!(qasm::parse(&(h(2) + "cx q[0],q[0];")).is_err());
    assert!(qasm::parse(&(h(2) + "h q[5];")).is_err());
    assert!(qasm::parse("OPENQASM 2.0; h q[0];").is_err());
    // definiție de poartă cu parametri
    let p = qasm::parse("OPENQASM 2.0;\nqreg q[1];\ngate tw(a,b) x { rz(a+b) x; rx(a*2) x; }\ntw(pi/4,pi/4) q[0];").unwrap();
    assert_eq!(p.gates().count(), 2);
    // comentarii și broadcast
    let p = qasm::parse("OPENQASM 2.0; /* c */ qreg q[3]; // x\nh q;").unwrap();
    assert_eq!(p.gates().count(), 3);
}

#[test]
fn ideal_sampling_statistics() {
    let p = qasm::parse(&(h(2) + "h q[0]; cx q[0],q[1];")).unwrap();
    let c = sample(&p, 20000, &NoiseModel::default(), Some(7)).unwrap();
    assert!(!c.trajectories);
    assert_eq!(c.map.len(), 2);
    assert!((c.freq(0b00) - 0.5).abs() < 0.02 && (c.freq(0b11) - 0.5).abs() < 0.02);
    // reproductibil
    let c2 = sample(&p, 20000, &NoiseModel::default(), Some(7)).unwrap();
    assert_eq!(c.map, c2.map);
}

#[test]
fn noise_depolarizing_rate() {
    let p = qasm::parse(&(h(1) + "x q[0];")).unwrap();
    let nm = NoiseModel { depol1: 0.15, depol2: 0.0, readout: 0.0 };
    let c = sample(&p, 40000, &nm, Some(3)).unwrap();
    assert!(c.trajectories);
    let p1 = c.freq(1);
    assert!((p1 - (1.0 - 2.0 * 0.15 / 3.0)).abs() < 0.01, "p1={p1}");
    // eroare de citire pură
    let nm = NoiseModel { readout: 0.1, ..Default::default() };
    let c = sample(&p, 40000, &nm, Some(4)).unwrap();
    assert!(!c.trajectories && (c.freq(1) - 0.9).abs() < 0.01);
}

#[test]
fn real_teleportation_with_midcircuit_measurement() {
    let ex = examples::EXAMPLES.iter().find(|e| e.title.starts_with("Teleportare")).unwrap();
    let p = qasm::parse(ex.qasm).unwrap();
    assert!(p.instrs.iter().any(|i| matches!(i, Instr::If { .. })));
    let mut vm = Vm::new(3, Rng::from_seed(11), NoiseModel::default()).unwrap();
    let (sx, sy, sz) = (1f64.sin() * 0.7f64.cos(), 1f64.sin() * 0.7f64.sin(), 1f64.cos());
    let mut seen = std::collections::HashSet::new();
    for _ in 0..200 {
        vm.run(&p).unwrap();
        seen.insert(vm.clbits.clone());
        let b = vm.state.bloch(2);
        assert!(near(b[0], sx) && near(b[1], sy) && near(b[2], sz), "bloch {b:?}");
    }
    assert!(seen.len() >= 3, "toate cele 4 rezultate clasice ar trebui să apară");
}

#[test]
fn reset_and_dynamic_counts() {
    let ex = examples::EXAMPLES.iter().find(|e| e.title.starts_with("Reset")).unwrap();
    let p = qasm::parse(ex.qasm).unwrap();
    let c = sample(&p, 4000, &NoiseModel::default(), Some(5)).unwrap();
    assert!(c.trajectories);
    assert!((c.freq(1) - 0.5).abs() < 0.05);
    let mut vm = Vm::new(1, Rng::from_seed(1), NoiseModel::default()).unwrap();
    for _ in 0..50 {
        vm.run(&p).unwrap();
        assert!(near(vm.state.prob_one(0), 0.0));
    }
}

#[test]
fn parallel_equals_serial_and_norm_preserved() {
    // 17 qubiți: peste pragul de paralelizare
    let n = 17;
    let mut p = Program::new(n);
    for q in 0..n {
        p.h(q);
        p.rz(q, 0.1 * (q as f64 + 1.0));
    }
    for q in 0..n - 1 {
        p.cx(q, q + 1);
        p.cp(q + 1, q, 0.37);
    }
    p.swap(0, n - 1);
    p.ccx(1, 2, 3);
    set_threads(1);
    let a = final_state(&p).unwrap();
    set_threads(6);
    let b = final_state(&p).unwrap();
    set_threads(0);
    assert!(near(a.norm_sqr(), 1.0));
    assert_eq!(a.amplitudes(), b.amplitudes());
}

#[test]
fn rng_uniformity() {
    let mut r = Rng::from_seed(99);
    let mut hist = [0usize; 10];
    for _ in 0..100_000 {
        hist[(r.next_f64() * 10.0) as usize] += 1;
    }
    for h in hist {
        assert!((h as f64 - 10_000.0).abs() < 500.0);
    }
}

// ---------- motoare pentru scară mare ----------
use cuantor_core::machine::Machine as _;

fn big_ghz(n: usize) -> Program {
    let mut p = Program::new(n);
    p.h(0);
    for q in 0..n - 1 {
        p.cx(q, q + 1);
    }
    p
}

#[test]
fn stabilizer_ghz_1000_qubits() {
    let p = big_ghz(1000);
    assert!(p.is_clifford());
    assert_eq!(choose_backend(&p, Backend::Auto).unwrap(), Backend::Stabilizer);
    let c = sample_with(&p, 40, &NoiseModel::default(), Some(1), Backend::Auto).unwrap();
    assert_eq!(c.backend, Backend::Stabilizer);
    // doar tot-0 sau tot-1
    assert_eq!(c.map.len(), 2, "{:?}", c.map.keys().map(|k| k.to_string_msb(1000).matches('1').count()).collect::<Vec<_>>());
    for k in c.map.keys() {
        let ones = (0..1000).filter(|&i| k.get(i)).count();
        assert!(ones == 0 || ones == 1000);
    }
}

#[test]
fn sparse_ghz_1000_qubits_with_t_gate() {
    let mut p = big_ghz(1000);
    p.gate(Kind::T, 999); // non-Clifford
    assert!(!p.is_clifford());
    assert_eq!(choose_backend(&p, Backend::Auto).unwrap(), Backend::Sparse);
    let a = analyze(&p, Backend::Auto, 4, 0, Some(1)).unwrap();
    assert_eq!(a.backend, Backend::Sparse);
    assert_eq!(a.support, Some(2));
    assert!(near(a.top[0].prob, 0.5));
    assert!(near(a.p_one[500], 0.5));
    let c = sample_with(&p, 500, &NoiseModel::default(), Some(2), Backend::Auto).unwrap();
    assert_eq!(c.map.len(), 2);
    // faza lui T pe ramura |1…1⟩
    let top = a.top.iter().find(|t| t.bits.starts_with('1')).unwrap();
    let amp = top.amp.unwrap();
    assert!(near(amp.arg(), PI / 4.0));
}

#[test]
fn sparse_blowup_is_reported() {
    let mut p = Program::new(40);
    for q in 0..40 {
        p.h(q);
    }
    p.gate(Kind::T, 0);
    let e = analyze(&p, Backend::Sparse, 4, 0, Some(1)).unwrap_err();
    assert!(e.0.contains("ramuri"), "{e}");
}

#[test]
fn stabilizer_matches_statevector_on_random_clifford_circuits() {
    let mut rng = Rng::from_seed(2024);
    for trial in 0..30 {
        let n = 2 + (trial % 4);
        let mut p = Program::new(n);
        for _ in 0..25 {
            let a = (rng.next_u64() as usize) % n;
            let mut b = (rng.next_u64() as usize) % n;
            if b == a {
                b = (a + 1) % n;
            }
            match rng.next_u64() % 12 {
                0 => p.h(a),
                1 => p.gate(Kind::S, a),
                2 => p.gate(Kind::Sdg, a),
                3 => p.x(a),
                4 => p.gate(Kind::Y, a),
                5 => p.z(a),
                6 => p.cx(a, b),
                7 => p.cz(a, b),
                8 => p.swap(a, b),
                9 => p.rx(a, PI / 2.0),
                10 => p.ry(a, PI / 2.0 * (1 + rng.next_u64() % 3) as f64),
                _ => p.push(Op::new(Kind::Y, vec![a], vec![b])),
            };
        }
        assert!(p.is_clifford());
        let sv = analyze(&p, Backend::StateVector, 1 << n, 0, Some(1)).unwrap();
        let st = analyze(&p, Backend::Stabilizer, 1, 0, Some(1)).unwrap();
        for q in 0..n {
            assert!(near(sv.p_one[q], st.p_one[q]), "trial {trial} q{q}: sv={} stab={}", sv.p_one[q], st.p_one[q]);
        }
        // distribuțiile complete coincid statistic
        let cs = sample_with(&p, 6000, &NoiseModel::default(), Some(9), Backend::Stabilizer).unwrap();
        let mut support: Vec<usize> = vec![];
        for t in &sv.top {
            if t.prob > 1e-9 {
                support.push(usize::from_str_radix(&t.bits, 2).unwrap());
            }
        }
        for (k, c) in cs.map.iter() {
            let idx = (0..n).fold(0usize, |a, b| a | ((k.get(b) as usize) << b));
            assert!(support.contains(&idx), "trial {trial}: rezultat imposibil {idx}");
            let want = sv.top.iter().find(|t| usize::from_str_radix(&t.bits, 2).unwrap() == idx).unwrap().prob;
            assert!((*c as f64 / 6000.0 - want).abs() < 0.035, "trial {trial}: {idx} {} vs {want}", *c as f64 / 6000.0);
        }
    }
}

#[test]
fn stabilizer_teleportation_dynamic() {
    let mut p = Program::new(3);
    p.add_creg("c0", 1).unwrap();
    p.add_creg("c1", 1).unwrap();
    // starea de teleportat: |1⟩ (Clifford)
    p.x(0);
    p.h(1);
    p.cx(1, 2);
    p.cx(0, 1);
    p.h(0);
    p.measure(0, 0);
    p.measure(1, 1);
    p.instrs.push(Instr::If { creg: 1, value: 1, op: Op::single(Kind::X, 2) });
    p.instrs.push(Instr::If { creg: 0, value: 1, op: Op::single(Kind::Z, 2) });
    p.measure(2, 0); // rescrie c0, citește q2
    // q2 trebuie să fie mereu |1⟩
    let mut m = Stabilizer::new(3).unwrap();
    let mut rng = Rng::from_seed(5);
    for _ in 0..100 {
        let cl = run_program(&mut m, &p, &NoiseModel::default(), &mut rng).unwrap();
        assert!(cl.get(0), "q2 ar trebui să fie |1⟩");
    }
}

#[test]
fn stabilizer_rejects_non_clifford_and_strings() {
    let mut p = Program::new(2);
    p.h(0).cx(0, 1);
    let a = analyze(&p, Backend::Stabilizer, 4, 64, Some(3)).unwrap();
    assert_eq!(a.stabilizers.len(), 2);
    assert!(a.stabilizers.contains(&"+XX".to_string()) && a.stabilizers.contains(&"+ZZ".to_string()), "{:?}", a.stabilizers);
    p.gate(Kind::T, 0);
    assert!(choose_backend(&p, Backend::Stabilizer).is_err());
}

#[test]
fn qasm3_import_export() {
    let src = "OPENQASM 3.0;\ninclude \"stdgates.inc\";\nqubit[4] q;\nbit[4] c;\nh q[0];\ncx q[0], q[1];\nc[0] = measure q[0];\n";
    let p = qasm::parse(src).unwrap();
    assert_eq!(p.n_qubits, 4);
    assert_eq!(p.gates().count(), 2);
    assert!(p.has_measure());
    let out = p.to_qasm3();
    assert!(out.contains("qubit[4] q;") && out.contains("c[0] = measure q[0];") && out.contains("cx q[0], q[1];"));
    let p2 = qasm::parse(&out).unwrap();
    assert_eq!(p.instrs.len(), p2.instrs.len());
    let qk = p.to_qiskit();
    assert!(qk.contains("circuit.h(qreg_q[0])") && qk.contains("circuit.cx(qreg_q[0], qreg_q[1])"));
}

#[test]
fn thousand_qubit_register_parses_and_measures() {
    let src = "OPENQASM 2.0;\nqreg q[1000];\ncreg c[1000];\nh q[0];\ncx q[0],q[1];\nmeasure q -> c;\n";
    let p = qasm::parse(src).unwrap();
    assert_eq!(p.n_qubits, 1000);
    let c = sample(&p, 30, &NoiseModel::default(), Some(1)).unwrap();
    assert_eq!(c.backend, Backend::Stabilizer);
    for k in c.map.keys() {
        assert_eq!(k.get(0), k.get(1));
        assert!((2..1000).all(|i| !k.get(i)));
    }
}
