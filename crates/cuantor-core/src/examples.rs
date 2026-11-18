//! Exemple și exerciții (inspirate de Workshop 07 «Introduction to Quantum Computing», UPB).

pub struct Example {
    pub group: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub qasm: &'static str,
}

macro_rules! hdr {
    ($n:literal) => {
        concat!("OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[", $n, "];\ncreg c[", $n, "];\n")
    };
}

pub const EXAMPLES: &[Example] = &[
    Example { group: "Din lecție", title: "Superpoziție (Hadamard)", desc: "H|0⟩ = |+⟩: 50% |0⟩, 50% |1⟩.", qasm: concat!(hdr!(1), "h q[0];\n") },
    Example { group: "Din lecție", title: "Superpoziție vs. amestec statistic", desc: "H apoi Ry(π/2). Superpoziție reală: p(1)=100%. Un amestec statistic ar da 50/50.", qasm: concat!(hdr!(1), "h q[0];\nry(pi/2) q[0];\n") },
    Example { group: "Din lecție", title: "Testul Ramsey", desc: "H – Rz(φ) – H cu φ=π/2+π/6: p(0)=cos²(φ/2).", qasm: concat!(hdr!(1), "h q[0];\nrz(pi/2+pi/6) q[0];\nh q[0];\n") },
    Example { group: "Din lecție", title: "Produs Kronecker", desc: "Ry(π/3) pe q0 (75/25) și Ry(π/4) pe q1: probabilitățile compuse sunt produse.", qasm: concat!(hdr!(2), "ry(pi/3) q[0];\nry(pi/4) q[1];\n") },
    Example { group: "Din lecție", title: "Încâlcire: starea Bell", desc: "H pe q0, apoi CNOT: (|00⟩+|11⟩)/√2. Qubiții nu mai sunt independenți.", qasm: concat!(hdr!(2), "h q[0];\ncx q[0],q[1];\n") },
    Example { group: "Din lecție", title: "Generator aleator pe 4 qubiți", desc: "H pe toți: fiecare măsurare dă un întreg uniform între 0 și 15.", qasm: concat!(hdr!(4), "h q[0];\nh q[1];\nh q[2];\nh q[3];\n") },
    Example { group: "Din lecție", title: "Ex. 2 — 75% |0⟩, 25% |1⟩", desc: "θ = 2·acos(√0,75) = π/3.", qasm: concat!(hdr!(1), "ry(pi/3) q[0];\n") },
    Example { group: "Din lecție", title: "Ex. 3 — NOT peste 75/25", desc: "X inversează probabilitățile: 25/75.", qasm: concat!(hdr!(1), "ry(pi/3) q[0];\nx q[0];\n") },
    Example { group: "Din lecție", title: "Ex. 4 — Rz nu schimbă probabilitățile", desc: "Rz modifică doar faza; vezi fazele în tabelul de stare.", qasm: concat!(hdr!(1), "ry(pi/3) q[0];\nrz(1.2) q[0];\n") },
    Example { group: "Din lecție", title: "Ex. 5 — Kronecker 75/25 × 60/40", desc: "Rezultate 0,45 · 0,15 · 0,30 · 0,10 pentru |00⟩, |01⟩, |10⟩, |11⟩.", qasm: concat!(hdr!(2), "ry(pi/3) q[0];\nry(2*acos(sqrt(0.6))) q[1];\n") },
    Example { group: "Din lecție", title: "Ex. 6 — SWAP", desc: "Schimbă între ele cele două probabilități: ordinea produsului se inversează.", qasm: concat!(hdr!(2), "ry(pi/3) q[0];\nry(2*acos(sqrt(0.6))) q[1];\nswap q[0],q[1];\n") },
    Example { group: "Din lecție", title: "Ex. 8 — CNOT condiționat", desc: "q0=1 ⇒ q1 se inversează: |01⟩ → |11⟩.", qasm: concat!(hdr!(2), "x q[0];\ncx q[0],q[1];\n") },
    Example { group: "Din lecție", title: "Ex. 9 — 50% |01⟩ + 50% |10⟩", desc: "Încâlcire anticorelată: H, Ry(π) pe q1, CNOT.", qasm: concat!(hdr!(2), "h q[0];\nry(pi) q[1];\ncx q[0],q[1];\n") },
    Example { group: "Algoritmi", title: "GHZ pe 3 qubiți", desc: "(|000⟩+|111⟩)/√2 — încâlcire pe trei qubiți.", qasm: concat!(hdr!(3), "h q[0];\ncx q[0],q[1];\ncx q[1],q[2];\n") },
    Example { group: "Algoritmi", title: "Deutsch–Jozsa (oracol echilibrat)", desc: "f(x)=x0⊕x1. Rezultat 11 (nenul) ⇒ funcția e echilibrată, dintr-o singură interogare.", qasm: concat!(hdr!(3), "x q[2];\nh q[0];\nh q[1];\nh q[2];\ncx q[0],q[2];\ncx q[1],q[2];\nh q[0];\nh q[1];\nmeasure q[0] -> c[0];\nmeasure q[1] -> c[1];\n") },
    Example { group: "Algoritmi", title: "Bernstein–Vazirani (secret 101)", desc: "Găsește șirul ascuns dintr-o singură interogare: măsurarea dă 101.", qasm: concat!(hdr!(4), "x q[3];\nh q[0];\nh q[1];\nh q[2];\nh q[3];\ncx q[0],q[3];\ncx q[2],q[3];\nh q[0];\nh q[1];\nh q[2];\nmeasure q[0] -> c[0];\nmeasure q[1] -> c[1];\nmeasure q[2] -> c[2];\n") },
    Example { group: "Algoritmi", title: "Grover pe 2 qubiți (caută |11⟩)", desc: "O singură iterație amplifică |11⟩ la 100%.", qasm: concat!(hdr!(2), "h q[0];\nh q[1];\ncz q[0],q[1];\nh q[0];\nh q[1];\nx q[0];\nx q[1];\ncz q[0],q[1];\nx q[0];\nx q[1];\nh q[0];\nh q[1];\n") },
    Example { group: "Algoritmi", title: "Transformata Fourier cuantică (3 qubiți, intrare |5⟩)", desc: "Probabilități uniforme; informația stă în faze.", qasm: concat!(hdr!(3), "x q[0];\nx q[2];\nh q[2];\ncu1(pi/2) q[1],q[2];\ncu1(pi/4) q[0],q[2];\nh q[1];\ncu1(pi/2) q[0],q[1];\nh q[0];\nswap q[0],q[2];\n") },
    Example { group: "Mașină virtuală (circuite dinamice)", title: "Teleportare cuantică reală", desc: "Măsurare la mijlocul circuitului + corecții clasice cu if: starea lui q0 apare pe q2.", qasm: "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[3];\ncreg c0[1];\ncreg c1[1];\nry(1.0) q[0];\nrz(0.7) q[0];\nh q[1];\ncx q[1],q[2];\ncx q[0],q[1];\nh q[0];\nmeasure q[0] -> c0[0];\nmeasure q[1] -> c1[0];\nif(c1==1) x q[2];\nif(c0==1) z q[2];\n" },
    Example { group: "Mașină virtuală (circuite dinamice)", title: "Poartă definită de utilizator", desc: "gate bell a,b { h a; cx a,b; } — expandată de parser.", qasm: "OPENQASM 2.0;\ninclude \"qelib1.inc\";\ngate bell a,b { h a; cx a,b; }\nqreg q[2];\ncreg c[2];\nbell q[0],q[1];\nmeasure q -> c;\n" },
    Example { group: "Mașină virtuală (circuite dinamice)", title: "Reset după măsurare", desc: "H, măsurare, reset: qubitul revine mereu la |0⟩, iar bitul clasic rămâne aleator.", qasm: "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[1];\ncreg c[1];\nh q[0];\nmeasure q[0] -> c[0];\nreset q[0];\n" },
];
