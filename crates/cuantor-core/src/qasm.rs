//! Parser OpenQASM 2.0: registre multiple, porți standard (qelib1), definiții
//! `gate`, `measure`, `reset`, `barrier` și `if (creg == n)`.

use crate::expr;
use crate::program::{Instr, Kind, Op, Program};
use crate::{Result, err};
use std::collections::HashMap;
use std::f64::consts::PI;

struct GateDef {
    params: Vec<String>,
    args: Vec<String>,
    body: Vec<String>,
}

struct Ctx {
    qregs: Vec<(String, usize, usize)>, // nume, offset, dimensiune
    nq: usize,
    prog: Program,
    defs: HashMap<String, GateDef>,
    /// contor de muncă pentru expansiunea definițiilor (protecție împotriva bombelor exponențiale)
    work: std::cell::Cell<u64>,
}

/// Limite anti-abuz pentru expansiunea definițiilor de porți.
const MAX_EXPANSION_WORK: u64 = 20_000_000;
const MAX_EXPANDED_OPS: usize = 2_000_000;

fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let b: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < b.len() {
        if b[i] == '/' && i + 1 < b.len() && b[i + 1] == '/' {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
        } else if b[i] == '/' && i + 1 < b.len() && b[i + 1] == '*' {
            i += 2;
            while i + 1 < b.len() && !(b[i] == '*' && b[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            out.push(' ');
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    out
}

fn split_statements(src: &str) -> Result<Vec<String>> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in src.chars() {
        match ch {
            '{' => {
                depth += 1;
                cur.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return Err(err!("'}}' fără pereche"));
                }
                cur.push(ch);
                if depth == 0 {
                    out.push(std::mem::take(&mut cur));
                }
            }
            ';' if depth == 0 => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    if depth != 0 {
        return Err(err!("'{{' neînchis"));
    }
    if !cur.trim().is_empty() {
        return Err(err!("instrucțiune neterminată cu ';': {}", cur.trim()));
    }
    Ok(out.into_iter().map(|s| s.split_whitespace().collect::<Vec<_>>().join(" ")).filter(|s| !s.is_empty()).collect())
}

fn ident(s: &str) -> (&str, &str) {
    let end = s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(s.len());
    (&s[..end], s[end..].trim_start())
}

fn split_top(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut d = 0;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' => {
                d += 1;
                cur.push(ch)
            }
            ')' => {
                d -= 1;
                cur.push(ch)
            }
            ',' if d == 0 => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out.into_iter().map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()
}

/// `nume(p1,p2) a,b` → (nume, [p1,p2], [a,b])
fn parse_call(st: &str) -> Result<(String, Vec<String>, Vec<String>)> {
    let (name, rest) = ident(st);
    if name.is_empty() {
        return Err(err!("instrucțiune necunoscută: {st}"));
    }
    let mut params = vec![];
    let mut rest = rest;
    if rest.starts_with('(') {
        let mut d = 0;
        let mut end = None;
        for (i, ch) in rest.char_indices() {
            if ch == '(' {
                d += 1;
            } else if ch == ')' {
                d -= 1;
                if d == 0 {
                    end = Some(i);
                    break;
                }
            }
        }
        let e = end.ok_or_else(|| err!("paranteză neînchisă în: {st}"))?;
        params = split_top(&rest[1..e]);
        rest = rest[e + 1..].trim_start();
    }
    Ok((name.to_string(), params, split_top(rest)))
}

struct Spec {
    kind: Kind,
    ctrl: usize,
    targets: usize,
    np: usize,
}

fn builtin(name: &str, p: &[f64]) -> Option<Spec> {
    let s = |kind: Kind, ctrl, targets, np| Some(Spec { kind, ctrl, targets, np });
    match name {
        "h" => s(Kind::H, 0, 1, 0),
        "x" => s(Kind::X, 0, 1, 0),
        "y" => s(Kind::Y, 0, 1, 0),
        "z" => s(Kind::Z, 0, 1, 0),
        "s" => s(Kind::S, 0, 1, 0),
        "sdg" => s(Kind::Sdg, 0, 1, 0),
        "t" => s(Kind::T, 0, 1, 0),
        "tdg" => s(Kind::Tdg, 0, 1, 0),
        "sx" => s(Kind::Sx, 0, 1, 0),
        "rx" => s(Kind::Rx(*p.first()?), 0, 1, 1),
        "ry" => s(Kind::Ry(*p.first()?), 0, 1, 1),
        "rz" => s(Kind::Rz(*p.first()?), 0, 1, 1),
        "u1" | "p" => s(Kind::P(*p.first()?), 0, 1, 1),
        "u2" => s(Kind::U(PI / 2.0, *p.first()?, *p.get(1)?), 0, 1, 2),
        "u3" | "u" | "U" => s(Kind::U(*p.first()?, *p.get(1)?, *p.get(2)?), 0, 1, 3),
        "cx" | "CX" | "cnot" => s(Kind::X, 1, 1, 0),
        "cy" => s(Kind::Y, 1, 1, 0),
        "cz" => s(Kind::Z, 1, 1, 0),
        "ch" => s(Kind::H, 1, 1, 0),
        "swap" => s(Kind::Swap, 0, 2, 0),
        "cp" | "cu1" => s(Kind::P(*p.first()?), 1, 1, 1),
        "crz" => s(Kind::Rz(*p.first()?), 1, 1, 1),
        "ccx" | "toffoli" => s(Kind::X, 2, 1, 0),
        "ccz" => s(Kind::Z, 2, 1, 0),
        "cswap" | "fredkin" => s(Kind::Swap, 1, 2, 0),
        _ => None,
    }
}

fn expected_params(name: &str) -> Option<usize> {
    Some(match name {
        "h" | "x" | "y" | "z" | "s" | "sdg" | "t" | "tdg" | "sx" | "cx" | "CX" | "cnot" | "cy" | "cz" | "ch" | "swap" | "ccx"
        | "toffoli" | "ccz" | "cswap" | "fredkin" => 0,
        "rx" | "ry" | "rz" | "u1" | "p" | "cp" | "cu1" | "crz" => 1,
        "u2" => 2,
        "u3" | "u" | "U" => 3,
        _ => return None,
    })
}

impl Ctx {
    fn resolve(&self, tok: &str) -> Result<Vec<usize>> {
        let (name, rest) = ident(tok);
        let reg = self.qregs.iter().find(|r| r.0 == name).ok_or_else(|| err!("registru cuantic necunoscut: {tok}"))?;
        if rest.is_empty() {
            return Ok((0..reg.2).map(|k| reg.1 + k).collect());
        }
        let inner = rest.strip_prefix('[').and_then(|r| r.strip_suffix(']')).ok_or_else(|| err!("argument invalid: {tok}"))?;
        let k: usize = inner.trim().parse().map_err(|_| err!("index invalid: {tok}"))?;
        if k >= reg.2 {
            return Err(err!("index în afara registrului: {tok}"));
        }
        Ok(vec![reg.1 + k])
    }

    fn call_single(&self, name: &str, params: &[f64], qs: &[usize], depth: usize, out: &mut Vec<Op>) -> Result<()> {
        if depth > 64 {
            return Err(err!("recursivitate prea adâncă în definiții de porți"));
        }
        self.work.set(self.work.get() + 1);
        if self.work.get() > MAX_EXPANSION_WORK || out.len() > MAX_EXPANDED_OPS {
            return Err(err!("expansiunea definițiilor de porți depășește limita ({MAX_EXPANDED_OPS} operații)"));
        }
        if let Some(def) = self.defs.get(name) {
            if params.len() != def.params.len() || qs.len() != def.args.len() {
                return Err(err!("{name}: număr greșit de parametri/argumente"));
            }
            let env: HashMap<String, f64> = def.params.iter().cloned().zip(params.iter().copied()).collect();
            let amap: HashMap<&str, usize> = def.args.iter().map(|s| s.as_str()).zip(qs.iter().copied()).collect();
            for st in &def.body {
                let (n2, p2, a2) = parse_call(st)?;
                if n2 == "barrier" {
                    continue;
                }
                let pv = p2.iter().map(|e| expr::eval(e, &env)).collect::<Result<Vec<_>>>()?;
                let qv = a2
                    .iter()
                    .map(|a| amap.get(a.as_str()).copied().ok_or_else(|| err!("argument necunoscut «{a}» în poarta {name}")))
                    .collect::<Result<Vec<_>>>()?;
                self.call_single(&n2, &pv, &qv, depth + 1, out)?;
            }
            return Ok(());
        }
        if name == "id" || name == "i" {
            return Ok(());
        }
        let np = expected_params(name).ok_or_else(|| err!("poartă necunoscută: {name}"))?;
        if params.len() != np {
            return Err(err!("{name} cere {np} parametri, primite {}", params.len()));
        }
        let sp = builtin(name, params).ok_or_else(|| err!("poartă necunoscută: {name}"))?;
        debug_assert_eq!(sp.np, np);
        if qs.len() != sp.ctrl + sp.targets {
            return Err(err!("{name} cere {} qubiți, primiți {}", sp.ctrl + sp.targets, qs.len()));
        }
        let op = Op::new(sp.kind, qs[..sp.ctrl].to_vec(), qs[sp.ctrl..].to_vec());
        op.validate(self.nq)?;
        out.push(op);
        Ok(())
    }

    fn call(&self, st: &str) -> Result<Vec<Op>> {
        let (name, ps, args) = parse_call(st)?;
        let env = HashMap::new();
        let params = ps.iter().map(|e| expr::eval(e, &env)).collect::<Result<Vec<_>>>()?;
        let lists = args.iter().map(|a| self.resolve(a)).collect::<Result<Vec<_>>>()?;
        let len = lists.iter().map(|l| l.len()).max().unwrap_or(1);
        if lists.iter().any(|l| l.len() != 1 && l.len() != len) {
            return Err(err!("registre de dimensiuni diferite în: {st}"));
        }
        let mut out = vec![];
        for i in 0..len {
            let qs: Vec<usize> = lists.iter().map(|l| if l.len() == 1 { l[0] } else { l[i] }).collect();
            self.call_single(&name, &params, &qs, 0, &mut out)?;
        }
        Ok(out)
    }
}

fn parse_decl(rest: &str) -> Result<(String, usize)> {
    let (name, r) = ident(rest);
    let inner = r.strip_prefix('[').and_then(|x| x.strip_suffix(']')).ok_or_else(|| err!("declarație invalidă: {rest}"))?;
    let n: usize = inner.trim().parse().map_err(|_| err!("dimensiune invalidă: {rest}"))?;
    if name.is_empty() || n == 0 {
        return Err(err!("declarație invalidă: {rest}"));
    }
    Ok((name.to_string(), n))
}

fn do_measure(cx: &mut Ctx, cregs_ready: &[(String, usize, usize)], st: &str) -> Result<()> {
    let rest = st.trim_start_matches(|c: char| c.is_ascii_alphabetic()).trim();
    let (a, b) = rest.split_once("->").ok_or_else(|| err!("measure invalid: {st}"))?;
    let qs = cx.resolve(a.trim())?;
    let (cname, crest) = ident(b.trim());
    let (_, off, size) = cregs_ready.iter().find(|c| c.0 == cname).cloned().ok_or_else(|| err!("registru clasic necunoscut: {cname}"))?;
    let cs: Vec<usize> = if crest.is_empty() {
        (0..size).map(|k| off + k).collect()
    } else {
        let inner = crest.strip_prefix('[').and_then(|x| x.strip_suffix(']')).ok_or_else(|| err!("measure invalid: {st}"))?;
        let k: usize = inner.trim().parse().map_err(|_| err!("index invalid: {st}"))?;
        if k >= size {
            return Err(err!("index în afara registrului clasic: {st}"));
        }
        vec![off + k]
    };
    if qs.len() != cs.len() {
        return Err(err!("measure: dimensiuni diferite: {st}"));
    }
    for (q, c) in qs.into_iter().zip(cs) {
        cx.prog.instrs.push(Instr::Measure { q, c });
    }
    Ok(())
}

pub fn parse(text: &str) -> Result<Program> {
    let src = strip_comments(text);
    let stmts = split_statements(&src)?;
    let mut cx = Ctx { qregs: vec![], nq: 0, prog: Program::new(1), defs: HashMap::new(), work: std::cell::Cell::new(0) };
    let mut cregs_ready: Vec<(String, usize, usize)> = vec![]; // nume, offset, size
    for st in &stmts {
        let (head, rest) = ident(st);
        let hl = head.to_ascii_lowercase();
        // OpenQASM 3: `c[0] = measure q[0]` -> `measure q[0] -> c[0]`
        if hl != "if" && st.contains('=') && st.contains("measure") {
            let (lhs, rhs) = st.split_once('=').unwrap();
            let rhs = rhs.trim().strip_prefix("measure").ok_or_else(|| err!("instrucțiune invalidă: {st}"))?;
            let st2 = format!("measure {} -> {}", rhs.trim(), lhs.trim());
            do_measure(&mut cx, &cregs_ready, &st2)?;
            continue;
        }
        match hl.as_str() {
            "openqasm" | "include" | "opaque" => {}
            "qubit" | "bit" => {
                // OpenQASM 3: `qubit[4] q;` / `bit[4] c;` / `qubit q;`
                let (n, name) = if let Some(r) = rest.strip_prefix('[') {
                    let e = r.find(']').ok_or_else(|| err!("declarație invalidă: {st}"))?;
                    (r[..e].trim().parse::<usize>().map_err(|_| err!("dimensiune invalidă: {st}"))?, r[e + 1..].trim().to_string())
                } else {
                    (1, rest.trim().to_string())
                };
                let decl = format!("{name}[{n}]");
                if hl == "qubit" {
                    let (name, n) = parse_decl(&decl)?;
                    if cx.nq + n > crate::MAX_QUBITS_TOTAL {
                        return Err(err!("prea mulți qubiți (maxim {})", crate::MAX_QUBITS_TOTAL));
                    }
                    cx.qregs.push((name, cx.nq, n));
                    cx.nq += n;
                } else {
                    let (name, n) = parse_decl(&decl)?;
                    cx.prog.add_creg(&name, n)?;
                    let off = cx.prog.n_clbits - n;
                    cregs_ready.push((name, off, n));
                }
            }
            "qreg" => {
                let (name, n) = parse_decl(rest)?;
                if cx.nq + n > crate::MAX_QUBITS_TOTAL {
                    return Err(err!("prea mulți qubiți (maxim {})", crate::MAX_QUBITS_TOTAL));
                }
                cx.qregs.push((name, cx.nq, n));
                cx.nq += n;
            }
            "creg" => {
                let (name, n) = parse_decl(rest)?;
                cx.prog.add_creg(&name, n)?;
                let off = cx.prog.n_clbits - n;
                cregs_ready.push((name, off, n));
            }
            "gate" => {
                let (name, r) = ident(rest);
                let (params, r) = if let Some(r2) = r.strip_prefix('(') {
                    let e = r2.find(')').ok_or_else(|| err!("paranteză lipsă în definiția porții {name}"))?;
                    (split_top(&r2[..e]), r2[e + 1..].trim_start())
                } else {
                    (vec![], r)
                };
                let ob = r.find('{').ok_or_else(|| err!("lipsește '{{' în poarta {name}"))?;
                let args = split_top(&r[..ob]);
                let body_txt = r[ob + 1..].trim_end().strip_suffix('}').ok_or_else(|| err!("lipsește '}}' în poarta {name}"))?;
                let body = body_txt.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                cx.defs.insert(name.to_string(), GateDef { params, args, body });
            }
            "barrier" => cx.prog.instrs.push(Instr::Barrier),
            "reset" => {
                for q in cx.resolve(rest)? {
                    cx.prog.instrs.push(Instr::Reset(q));
                }
            }
            "measure" => do_measure(&mut cx, &cregs_ready, st)?,
            "if" => {
                let r = rest.strip_prefix('(').ok_or_else(|| err!("if invalid: {st}"))?;
                let e = r.find(')').ok_or_else(|| err!("if invalid: {st}"))?;
                let (cn, val) = r[..e].split_once("==").ok_or_else(|| err!("if cere (registru==număr): {st}"))?;
                let creg = cx.prog.cregs.iter().position(|c| c.name == cn.trim()).ok_or_else(|| err!("registru clasic necunoscut: {}", cn.trim()))?;
                if cx.prog.cregs[creg].size > 64 {
                    return Err(err!("if: registrul {} are peste 64 de biți", cn.trim()));
                }
                let value: u64 = val.trim().parse().map_err(|_| err!("valoare invalidă în if: {st}"))?;
                for op in cx.call(r[e + 1..].trim())? {
                    cx.prog.instrs.push(Instr::If { creg, value, op });
                }
            }
            _ => {
                for op in cx.call(st)? {
                    cx.prog.instrs.push(Instr::Gate(op));
                }
            }
        }
    }
    if cx.nq == 0 {
        return Err(err!("lipsește declarația qreg"));
    }
    cx.prog.n_qubits = cx.nq;
    cx.prog.check()?;
    Ok(cx.prog)
}
