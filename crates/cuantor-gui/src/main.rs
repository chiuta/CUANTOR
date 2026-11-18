//! CUANTOR Composer — interfață desktop (Windows, macOS, Linux) de tip IBM Composer.

mod i18n;
use cuantor_core::examples::EXAMPLES;
use i18n::{Lang, tr, trf};
use cuantor_core::program::{fmt_angle, fmt_angle_q};
use cuantor_core::{Analysis, Backend, Counts, Instr, Kind, NoiseModel, Op, Program, analyze, choose_backend, qasm, sample_with};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use std::f32::consts::{PI, TAU};
use std::sync::mpsc::{Receiver, Sender, channel};

const CELL: f32 = 46.0;
const LABEL_W: f32 = 54.0;
const DIAL_W: f32 = 40.0;
const SIZES: [usize; 9] = [2, 5, 10, 20, 50, 100, 250, 500, 1000];

// ---------------------------------------------------------------- instrumente

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tool {
    H, X, Y, Z, S, Sdg, T, Tdg, Sx, P, Rx, Ry, Rz,
    Cx, Cz, Cy, Ch, Cp, Crz, Swap, Ccx, Cswap,
    Measure, Erase, Select,
}

const RED_ORANGE: Color32 = Color32::from_rgb(0xe8, 0x4a, 0x27);
const CRIMSON: Color32 = Color32::from_rgb(0xb8, 0x2a, 0x2a);
const PURPLE: Color32 = Color32::from_rgb(0x8a, 0x3f, 0xfc);
const BLUE: Color32 = Color32::from_rgb(0x4d, 0x8d, 0xff);
const GREY: Color32 = Color32::from_rgb(0x6f, 0x76, 0x8c);

impl Tool {
    fn label(self) -> &'static str {
        tr(self.label_ro())
    }
    fn label_ro(self) -> &'static str {
        match self {
            Tool::H => "H", Tool::X => "X", Tool::Y => "Y", Tool::Z => "Z", Tool::S => "S", Tool::Sdg => "S†",
            Tool::T => "T", Tool::Tdg => "T†", Tool::Sx => "√X", Tool::P => "P", Tool::Rx => "RX", Tool::Ry => "RY",
            Tool::Rz => "RZ", Tool::Cx => "CX", Tool::Cz => "CZ", Tool::Cy => "CY", Tool::Ch => "CH", Tool::Cp => "CP",
            Tool::Crz => "CRZ", Tool::Swap => "SW", Tool::Ccx => "CCX", Tool::Cswap => "CSW", Tool::Measure => "M",
            Tool::Erase => "Șt", Tool::Select => "Sel",
        }
    }
    fn tip(self) -> &'static str {
        tr(self.tip_ro())
    }
    fn tip_ro(self) -> &'static str {
        match self {
            Tool::H => "Hadamard: superpoziție egală",
            Tool::X => "NOT (Pauli-X)",
            Tool::Y => "Pauli-Y",
            Tool::Z => "Pauli-Z",
            Tool::S => "S (fază π/2)",
            Tool::Sdg => "S† (fază −π/2)",
            Tool::T => "T (fază π/4)",
            Tool::Tdg => "T† (fază −π/4)",
            Tool::Sx => "Rădăcina pătrată din X",
            Tool::P => "Poartă de fază P(φ)",
            Tool::Rx => "Rotație în jurul axei X",
            Tool::Ry => "Rotație în jurul axei Y",
            Tool::Rz => "Rotație în jurul axei Z (doar fază)",
            Tool::Cx => "CNOT: control apoi țintă",
            Tool::Cz => "CZ: control apoi țintă",
            Tool::Cy => "CY: control apoi țintă",
            Tool::Ch => "CH: control apoi țintă",
            Tool::Cp => "Fază controlată CP(φ)",
            Tool::Crz => "Rz controlat",
            Tool::Swap => "SWAP: doi qubiți",
            Tool::Ccx => "Toffoli: 2 controale, țintă",
            Tool::Cswap => "Fredkin: control, 2 qubiți",
            Tool::Measure => "Măsurare în baza Z",
            Tool::Erase => "Gumă",
            Tool::Select => "Selectează o poartă (pentru unghi)",
        }
    }
    fn color(self) -> Color32 {
        match self {
            Tool::H => RED_ORANGE,
            Tool::X | Tool::Y | Tool::Sx | Tool::Rx | Tool::Ry => CRIMSON,
            Tool::Z | Tool::S | Tool::Sdg | Tool::T | Tool::Tdg | Tool::P | Tool::Rz => BLUE,
            Tool::Cx | Tool::Cz | Tool::Cy | Tool::Ch | Tool::Cp | Tool::Crz | Tool::Swap | Tool::Ccx | Tool::Cswap => PURPLE,
            Tool::Measure | Tool::Erase | Tool::Select => GREY,
        }
    }
    fn has_angle(self) -> bool {
        matches!(self, Tool::P | Tool::Rx | Tool::Ry | Tool::Rz | Tool::Cp | Tool::Crz)
    }
    /// Câți qubiți cere poarta (1 pentru porți simple).
    fn arity(self) -> usize {
        match self {
            Tool::Cx | Tool::Cz | Tool::Cy | Tool::Ch | Tool::Cp | Tool::Crz | Tool::Swap => 2,
            Tool::Ccx | Tool::Cswap => 3,
            _ => 1,
        }
    }
    fn role(self, k: usize) -> &'static str {
        tr(self.role_ro(k))
    }
    fn role_ro(self, k: usize) -> &'static str {
        match (self, k) {
            (Tool::Swap, _) => "qubitul",
            (Tool::Ccx, 0) | (Tool::Ccx, 1) => "controlul",
            (Tool::Cswap, 0) => "controlul",
            (_, 0) if self.arity() > 1 => "controlul",
            _ => "ținta",
        }
    }
    fn make(self, qs: &[usize], a: f64) -> Option<Op> {
        let (kind, nc) = match self {
            Tool::H => (Kind::H, 0), Tool::X => (Kind::X, 0), Tool::Y => (Kind::Y, 0), Tool::Z => (Kind::Z, 0),
            Tool::S => (Kind::S, 0), Tool::Sdg => (Kind::Sdg, 0), Tool::T => (Kind::T, 0), Tool::Tdg => (Kind::Tdg, 0),
            Tool::Sx => (Kind::Sx, 0), Tool::P => (Kind::P(a), 0), Tool::Rx => (Kind::Rx(a), 0), Tool::Ry => (Kind::Ry(a), 0),
            Tool::Rz => (Kind::Rz(a), 0), Tool::Cx => (Kind::X, 1), Tool::Cz => (Kind::Z, 1), Tool::Cy => (Kind::Y, 1),
            Tool::Ch => (Kind::H, 1), Tool::Cp => (Kind::P(a), 1), Tool::Crz => (Kind::Rz(a), 1), Tool::Swap => (Kind::Swap, 0),
            Tool::Ccx => (Kind::X, 2), Tool::Cswap => (Kind::Swap, 1),
            _ => return None,
        };
        Some(Op::new(kind, qs[..nc].to_vec(), qs[nc..].to_vec()))
    }
}

const PALETTE: [Tool; 25] = [
    Tool::H, Tool::Cx, Tool::Ccx, Tool::Swap, Tool::Cswap, Tool::Cz,
    Tool::T, Tool::S, Tool::Z, Tool::Tdg, Tool::Sdg, Tool::P,
    Tool::Rz, Tool::Cp, Tool::Crz, Tool::Cy, Tool::Ch, Tool::Measure,
    Tool::Sx, Tool::X, Tool::Y, Tool::Rx, Tool::Ry, Tool::Select, Tool::Erase,
];

// ---------------------------------------------------------------- model

#[derive(Clone, Debug, PartialEq)]
enum Item {
    Gate(Op),
    Measure(usize),
}
#[derive(Clone, Debug, PartialEq)]
struct Cell {
    item: Item,
    col: usize,
}
impl Cell {
    fn qubits(&self) -> Vec<usize> {
        match &self.item {
            Item::Gate(o) => o.qubits(),
            Item::Measure(q) => vec![*q],
        }
    }
}

#[derive(Clone, PartialEq)]
struct Doc {
    n: usize,
    cells: Vec<Cell>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CodeFmt {
    Qasm3,
    Qasm2,
    Qiskit,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomLeft {
    Probs,
    Shots,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomRight {
    QSphere,
    StateVec,
}

enum Msg {
    Analysis(u64, Result<Analysis, String>),
    Shots(Result<Counts, String>),
    Hw(Result<(String, Vec<(String, usize)>), String>),
    HwStatus(String),
}

struct App {
    doc: Doc,
    undo: Vec<Doc>,
    redo: Vec<Doc>,
    tool: Tool,
    pending: Option<(usize, Vec<usize>)>,
    angle_text: String,
    selected: Option<usize>,
    sel_dirty: bool,
    step: Option<usize>,
    backend: Backend,
    // analiză
    gen_id: u64,
    analysis: Option<Analysis>,
    analysis_err: Option<String>,
    busy: bool,
    an_running: bool,
    an_queued: bool,
    // măsurători
    shots: usize,
    noise: NoiseModel,
    counts: Option<Counts>,
    run_busy: bool,
    // panouri
    code_fmt: CodeFmt,
    code: String,
    code_dirty: bool,
    code_err: String,
    bl: BottomLeft,
    br: BottomRight,
    show_state: bool,
    show_phase: bool,
    inspect: bool,
    view: (f32, f32),
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
    status: String,
    file_path: String,
    ibm_backend: String,
    ibm_busy: bool,
    ibm_out: Vec<String>,
}

fn empty_doc(n: usize) -> Doc {
    Doc { n, cells: vec![] }
}

fn parse_angle(s: &str) -> Option<f64> {
    cuantor_core::expr::eval_const(s).ok()
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::with_ctx(&cc.egui_ctx)
    }

    fn with_ctx(ectx: &egui::Context) -> Self {
        let mut style = (*ectx.global_style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.panel_fill = Color32::from_rgb(0x16, 0x16, 0x1d);
        style.visuals.window_fill = Color32::from_rgb(0x1e, 0x1e, 0x28);
        ectx.set_global_style(style);
        i18n::set(Lang::detect());
        let (tx, rx) = channel();
        let mut app = App {
            doc: empty_doc(4),
            undo: vec![],
            redo: vec![],
            tool: Tool::H,
            pending: None,
            angle_text: "pi/2".into(),
            selected: None,
            sel_dirty: false,
            step: None,
            backend: Backend::Auto,
            gen_id: 0,
            analysis: None,
            analysis_err: None,
            busy: false,
            an_running: false,
            an_queued: false,
            shots: 1024,
            noise: NoiseModel::default(),
            counts: None,
            run_busy: false,
            code_fmt: CodeFmt::Qasm3,
            code: String::new(),
            code_dirty: false,
            code_err: String::new(),
            bl: BottomLeft::Probs,
            br: BottomRight::QSphere,
            show_state: true,
            show_phase: false,
            inspect: false,
            view: (0.5, 0.45),
            tx,
            rx,
            status: String::new(),
            file_path: "circuit.qasm".into(),
            ibm_backend: String::new(),
            ibm_busy: false,
            ibm_out: vec![],
        };
        app.doc.cells.push(Cell { item: Item::Gate(Op::single(Kind::H, 0)), col: 0 });
        app.changed(ectx);
        app
    }

    fn total_cols(&self) -> usize {
        self.doc.cells.iter().map(|c| c.col + 1).max().unwrap_or(0)
    }

    fn program_upto(&self, upto: Option<usize>) -> Program {
        let mut p = Program::new(self.doc.n);
        let _ = p.add_creg("c", self.doc.n);
        let mut cells: Vec<&Cell> = self.doc.cells.iter().filter(|c| upto.is_none_or(|k| c.col < k)).collect();
        cells.sort_by_key(|c| c.col);
        for c in cells {
            match &c.item {
                Item::Gate(o) => {
                    p.push(o.clone());
                }
                Item::Measure(q) => {
                    p.measure(*q, *q);
                }
            }
        }
        p
    }

    fn mutate(&mut self, ctx: &egui::Context, f: impl FnOnce(&mut Doc)) {
        self.undo.push(self.doc.clone());
        if self.undo.len() > 200 {
            self.undo.remove(0);
        }
        self.redo.clear();
        f(&mut self.doc);
        self.changed(ctx);
    }

    fn changed(&mut self, ctx: &egui::Context) {
        let n = self.doc.n;
        self.doc.cells.retain(|c| c.qubits().iter().all(|&q| q < n));
        self.pending = None;
        self.counts = None;
        if self.selected.is_some_and(|i| i >= self.doc.cells.len()) {
            self.selected = None;
        }
        if !self.code_dirty {
            self.refresh_code();
        }
        self.spawn_analysis(ctx);
    }

    fn refresh_code(&mut self) {
        let p = self.program_upto(None);
        self.code = match self.code_fmt {
            CodeFmt::Qasm3 => p.to_qasm3(),
            CodeFmt::Qasm2 => p.to_qasm(),
            CodeFmt::Qiskit => p.to_qiskit(),
        };
    }

    fn spawn_analysis(&mut self, ctx: &egui::Context) {
        // un singur fir de analiză activ; cererile noi se strâng într-una singură în coadă
        self.busy = true;
        if self.an_running {
            self.an_queued = true;
            return;
        }
        self.an_running = true;
        self.gen_id += 1;
        let id = self.gen_id;
        let prog = self.program_upto(self.step);
        let req = self.backend;
        let k = if self.doc.n <= 5 { 1usize << self.doc.n } else { 32 };
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let r = analyze(&prog, req, k, 512, None).map_err(|e| e.0);
            let _ = tx.send(Msg::Analysis(id, r));
            ctx.request_repaint();
        });
    }

    fn run_shots(&mut self, ctx: &egui::Context) {
        let prog = self.program_upto(self.step);
        let (shots, noise, backend) = (self.shots, self.noise.clone(), self.backend);
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        self.run_busy = true;
        self.bl = BottomLeft::Shots;
        std::thread::spawn(move || {
            let r = sample_with(&prog, shots, &noise, None, backend).map_err(|e| e.0);
            let _ = tx.send(Msg::Shots(r));
            ctx.request_repaint();
        });
    }

    fn run_ibm(&mut self, ctx: &egui::Context) {
        let prog = self.program_upto(None);
        let (shots, name) = (self.shots, self.ibm_backend.clone());
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        self.ibm_busy = true;
        self.ibm_out.clear();
        std::thread::spawn(move || {
            let res = (|| -> Result<(String, Vec<(String, usize)>), String> {
                let mut cfg = cuantor_ibm::Config::load()?;
                if !name.is_empty() {
                    cfg.backend = name;
                }
                if cfg.backend.is_empty() {
                    return Err("alege un backend".into());
                }
                let mut c = cuantor_ibm::Client::new(cfg.clone());
                let bc = c.backend_config(&cfg.backend)?;
                let (id, t) = c.submit(&prog, shots, &bc)?;
                let _ = tx.send(Msg::HwStatus(format!("job {id} trimis pe {}", cfg.backend)));
                ctx.request_repaint();
                let txs = tx.clone();
                c.wait(&id, std::time::Duration::from_secs(3600), |s| {
                    let _ = txs.send(Msg::HwStatus(format!("IBM: {s}")));
                })?;
                let rows = c.results(&id)?;
                Ok((format!("{} · {} porți 2q · {} swap-uri", cfg.backend, t.two_qubit_count(), t.swaps), rows))
            })();
            let _ = tx.send(Msg::Hw(res));
            ctx.request_repaint();
        });
    }

    fn poll(&mut self, ctx: &egui::Context) {
        while let Ok(m) = self.rx.try_recv() {
            match m {
                Msg::Analysis(id, r) => {
                    self.an_running = false;
                    if self.an_queued {
                        self.an_queued = false;
                        self.spawn_analysis(ctx);
                    }
                    if id == self.gen_id || !self.an_running {
                        if !self.an_running {
                            self.busy = false;
                        }
                        match r {
                            Ok(a) => {
                                self.analysis = Some(a);
                                self.analysis_err = None;
                            }
                            Err(e) => self.analysis_err = Some(e),
                        }
                    }
                }
                Msg::HwStatus(t) => self.status = t,
                Msg::Hw(r) => {
                    self.ibm_busy = false;
                    match r {
                        Ok((info, rows)) => {
                            let tot: usize = rows.iter().map(|r| r.1).sum::<usize>().max(1);
                            self.ibm_out = std::iter::once(info).chain(rows.iter().take(12).map(|(k, n)| format!("{k}  {n}  {:.1}%", *n as f64 * 100.0 / tot as f64))).collect();
                            self.status = tr("Rezultate IBM primite").into();
                        }
                        Err(e) => self.status = format!("IBM: {e}"),
                    }
                }
                Msg::Shots(r) => {
                    self.run_busy = false;
                    match r {
                        Ok(c) => self.counts = Some(c),
                        Err(e) => self.status = trf("Eroare la măsurare: {}", &[&e]),
                    }
                }
            }
        }
    }

    fn import_program(&mut self, ctx: &egui::Context, p: &Program) -> Result<(), String> {
        let mut next = vec![0usize; p.n_qubits];
        let mut cells = vec![];
        for ins in &p.instrs {
            let (item, qs) = match ins {
                Instr::Gate(o) => (Item::Gate(o.clone()), o.qubits()),
                Instr::Measure { q, .. } => (Item::Measure(*q), vec![*q]),
                Instr::Barrier => continue,
                Instr::Reset(_) => return Err(tr("reset nu poate fi desenat în editor (folosește CLI-ul pentru circuite dinamice)").into()),
                Instr::If { .. } => return Err(tr("instrucțiunile if nu pot fi desenate în editor (folosește CLI-ul)").into()),
            };
            let col = qs.iter().map(|&q| next[q]).max().unwrap_or(0);
            for &q in &qs {
                next[q] = col + 1;
            }
            cells.push(Cell { item, col });
        }
        let n = p.n_qubits;
        self.mutate(ctx, |d| {
            d.n = n;
            d.cells = cells;
        });
        self.step = None;
        Ok(())
    }

    fn load_qasm(&mut self, ctx: &egui::Context, text: &str) {
        match qasm::parse(text) {
            Ok(p) => match self.import_program(ctx, &p) {
                Ok(()) => {
                    self.code_err.clear();
                    self.code_dirty = false;
                    self.refresh_code();
                }
                Err(e) => self.code_err = e,
            },
            Err(e) => self.code_err = e.0,
        }
    }

    // ---------------------------------------------------------------- colocare
    fn click_cell(&mut self, ctx: &egui::Context, q: usize, c: usize) {
        let occ = self.doc.cells.iter().position(|x| x.col == c && x.qubits().contains(&q));
        match self.tool {
            Tool::Erase => {
                self.pending = None;
                if let Some(i) = occ {
                    self.mutate(ctx, |d| {
                        d.cells.remove(i);
                    });
                }
            }
            Tool::Select => {
                self.pending = None;
                self.selected = occ.filter(|&i| matches!(&self.doc.cells[i].item, Item::Gate(o) if o.kind.angle().is_some()));
                self.sel_dirty = false;
                if let Some(i) = self.selected {
                    if let Item::Gate(o) = &self.doc.cells[i].item {
                        self.angle_text = fmt_angle_q(o.kind.angle().unwrap_or(0.0));
                    }
                }
            }
            Tool::Measure => {
                self.mutate(ctx, |d| {
                    if let Some(i) = occ {
                        d.cells.remove(i);
                    }
                    d.cells.push(Cell { item: Item::Measure(q), col: c });
                });
            }
            t if t.arity() == 1 => {
                let a = if t.has_angle() {
                    match parse_angle(&self.angle_text) {
                        Some(a) => a,
                        None => {
                            self.status = tr("Unghi invalid").into();
                            return;
                        }
                    }
                } else {
                    0.0
                };
                if let Some(op) = t.make(&[q], a) {
                    self.mutate(ctx, |d| {
                        if let Some(i) = occ {
                            d.cells.remove(i);
                        }
                        d.cells.push(Cell { item: Item::Gate(op), col: c });
                    });
                }
            }
            t => {
                if occ.is_some() {
                    self.status = tr("Celulă ocupată: alege alt pas sau șterge poarta.").into();
                    return;
                }
                match &mut self.pending {
                    Some((pc, qs)) if *pc == c && !qs.contains(&q) => qs.push(q),
                    Some((pc, qs)) if *pc == c && qs.contains(&q) => {
                        self.pending = None;
                        return;
                    }
                    _ => self.pending = Some((c, vec![q])),
                }
                let done = self.pending.as_ref().is_some_and(|(_, qs)| qs.len() == t.arity());
                if done {
                    let (_, qs) = self.pending.take().unwrap();
                    let a = if t.has_angle() { parse_angle(&self.angle_text).unwrap_or(std::f64::consts::FRAC_PI_2) } else { 0.0 };
                    if let Some(op) = t.make(&qs, a) {
                        self.mutate(ctx, |d| d.cells.push(Cell { item: Item::Gate(op), col: c }));
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- culori / desen

fn phase_color(phase: f64) -> Color32 {
    // 0 → albastru, π/2 → magenta, π → galben, 3π/2 → verde (ca în inelul de fază IBM)
    let h = ((240.0 + phase.to_degrees()).rem_euclid(360.0)) as f32 / 360.0;
    let c = egui::ecolor::Hsva::new(h, 0.78, 0.95, 1.0);
    Color32::from(c)
}

fn draw_gate_box(p: &egui::Painter, center: Pos2, label: &str, color: Color32, sub: Option<&str>) {
    let r = Rect::from_center_size(center, vec2(34.0, 34.0));
    p.rect_filled(r, 4.0, color);
    p.text(center - vec2(0.0, if sub.is_some() { 3.0 } else { 0.0 }), Align2::CENTER_CENTER, label, FontId::proportional(15.0), Color32::WHITE);
    if let Some(s) = sub {
        p.text(center + vec2(0.0, 11.0), Align2::CENTER_CENTER, s, FontId::proportional(9.0), Color32::WHITE);
    }
}

fn draw_dial(p: &egui::Painter, c: Pos2, p1: f64) {
    let r = 13.0;
    p.circle_filled(c, r, Color32::from_rgb(0x1e, 0x1e, 0x28));
    // umplere de jos în sus proporțională cu P(1)
    let steps = 24;
    let level = (c.y + r) - 2.0 * r * p1 as f32;
    for i in 0..steps {
        let y = c.y - r + 2.0 * r * (i as f32 + 0.5) / steps as f32;
        if y >= level {
            let dy = y - c.y;
            let hw = (r * r - dy * dy).max(0.0).sqrt();
            p.line_segment([pos2(c.x - hw, y), pos2(c.x + hw, y)], Stroke::new(2.0 * r / steps as f32 + 0.5, Color32::from_rgb(0x4d, 0x8d, 0xff)));
        }
    }
    p.circle_stroke(c, r, Stroke::new(1.5, Color32::from_gray(200)));
}

fn proj(v: (f32, f32), x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let (ca, sa, ce, se) = (v.0.cos(), v.0.sin(), v.1.cos(), v.1.sin());
    let x1 = x * ca - y * sa;
    let y1 = x * sa + y * ca;
    (x1, -(y1 * se + z * ce), y1 * ce - z * se)
}

fn draw_qsphere(ui: &mut egui::Ui, app: &mut App, size: egui::Vec2) {
    let (resp, p) = ui.allocate_painter(size, Sense::drag());
    if resp.dragged() {
        let d = resp.drag_delta();
        app.view.0 += d.x * 0.012;
        app.view.1 = (app.view.1 + d.y * 0.012).clamp(-1.4, 1.4);
    }
    let rect = resp.rect;
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.36).max(20.0);
    p.circle_stroke(c, r, Stroke::new(1.2, Color32::from_gray(90)));
    // paralele
    for k in 1..4 {
        let lat = PI * k as f32 / 4.0;
        let pts: Vec<Pos2> = (0..=64)
            .map(|i| {
                let ph = TAU * i as f32 / 64.0;
                let q = proj(app.view, lat.sin() * ph.cos(), lat.sin() * ph.sin(), lat.cos());
                pos2(c.x + r * q.0, c.y + r * q.1)
            })
            .collect();
        p.add(egui::Shape::line(pts, Stroke::new(0.8, Color32::from_gray(55))));
    }
    let Some(a) = app.analysis.as_ref() else {
        p.text(c, Align2::CENTER_CENTER, tr("se calculează…"), FontId::proportional(13.0), Color32::GRAY);
        return;
    };
    let n = a.n.max(1);
    // grupare pe greutate Hamming
    let mut by_w: std::collections::BTreeMap<usize, Vec<usize>> = Default::default();
    for (i, t) in a.top.iter().enumerate() {
        by_w.entry(t.bits.chars().filter(|&ch| ch == '1').count()).or_default().push(i);
    }
    let mut draws: Vec<(f32, usize)> = vec![];
    let mut pts = vec![];
    for (w, idxs) in &by_w {
        let lat = PI * *w as f32 / n as f32;
        let mut sorted = idxs.clone();
        sorted.sort_by(|&x, &y| a.top[x].bits.cmp(&a.top[y].bits));
        for (k, &i) in sorted.iter().enumerate() {
            let lon = TAU * k as f32 / sorted.len().max(1) as f32 + 0.3;
            let q = proj(app.view, lat.sin() * lon.cos(), lat.sin() * lon.sin(), lat.cos());
            pts.push((i, pos2(c.x + r * q.0, c.y + r * q.1), q.2));
            draws.push((q.2, pts.len() - 1));
        }
    }
    draws.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)); // departe întâi
    p.circle_filled(c, 3.0, Color32::from_rgb(0x4d, 0x8d, 0xff));
    let mut labeled = 0;
    for (_, k) in draws {
        let (i, pos, depth) = pts[k];
        let t = &a.top[i];
        let col = match t.amp {
            Some(z) => phase_color(z.arg()),
            None => Color32::from_rgb(0x8d, 0xa6, 0xff),
        };
        let alpha = if depth > 0.0 { 120 } else { 255 };
        let col = Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), alpha);
        p.line_segment([c, pos], Stroke::new(1.6, col));
        let rad = (2.5 + 8.0 * t.prob.sqrt() as f32).min(11.0);
        p.circle_filled(pos, rad, col);
        if (app.show_state || app.show_phase) && labeled < 14 {
            let mut s = String::new();
            if app.show_state {
                let b = &t.bits;
                s = if b.len() > 10 { format!("|{}…{}>", &b[..4], &b[b.len() - 4..]) } else { format!("|{b}>") };
            }
            if app.show_phase {
                if let Some(z) = t.amp {
                    s = format!("{s} {}", fmt_angle(z.arg()));
                }
            }
            p.text(pos + vec2(0.0, -rad - 7.0), Align2::CENTER_CENTER, s, FontId::monospace(10.0), Color32::from_gray(210));
            labeled += 1;
        }
    }
    // inel de fază
    let rc = pos2(rect.min.x + 44.0, rect.max.y - 46.0);
    for i in 0..72 {
        let a0 = TAU * i as f32 / 72.0;
        let a1 = TAU * (i + 1) as f32 / 72.0;
        let col = phase_color(a0 as f64 + 0.04);
        p.line_segment([pos2(rc.x + 20.0 * a0.cos(), rc.y - 20.0 * a0.sin()), pos2(rc.x + 20.0 * a1.cos(), rc.y - 20.0 * a1.sin())], Stroke::new(6.0, col));
    }
    p.text(rc, Align2::CENTER_CENTER, tr("Fază"), FontId::proportional(10.0), Color32::LIGHT_GRAY);
    for (txt, dx, dy) in [("0", 34.0, 0.0), ("π/2", 0.0, -33.0), ("π", -32.0, 0.0), ("3π/2", 0.0, 33.0)] {
        p.text(rc + vec2(dx, dy), Align2::CENTER_CENTER, txt, FontId::proportional(10.0), Color32::GRAY);
    }
}

fn draw_probs(ui: &mut egui::Ui, app: &App, size: egui::Vec2) {
    let (resp, p) = ui.allocate_painter(size, Sense::hover());
    let rect = resp.rect;
    let plot = Rect::from_min_max(pos2(rect.min.x + 38.0, rect.min.y + 8.0), pos2(rect.max.x - 8.0, rect.max.y - 54.0));
    for k in 0..=5 {
        let y = plot.max.y - plot.height() * k as f32 / 5.0;
        p.line_segment([pos2(plot.min.x, y), pos2(plot.max.x, y)], Stroke::new(0.8, Color32::from_gray(60)));
        p.text(pos2(plot.min.x - 6.0, y), Align2::RIGHT_CENTER, format!("{}", k * 20), FontId::proportional(10.0), Color32::GRAY);
    }
    let mut rows: Vec<(String, f64, Option<f64>)> = vec![];
    match app.bl {
        BottomLeft::Probs => {
            if let Some(a) = &app.analysis {
                let mut v: Vec<_> = a.top.iter().map(|t| (t.bits.clone(), t.prob, None)).collect();
                if a.n <= 5 {
                    // toate 2^n stările, în ordine
                    let mut all: Vec<(String, f64, Option<f64>)> = (0..1usize << a.n).map(|i| (format!("{:0w$b}", i, w = a.n), 0.0, None)).collect();
                    for (b, pr, _) in v {
                        if let Ok(i) = usize::from_str_radix(&b, 2) {
                            all[i].1 = pr;
                        }
                    }
                    v = all;
                }
                rows = v;
            }
        }
        BottomLeft::Shots => {
            if let Some(c) = &app.counts {
                for (k, cnt) in c.sorted().into_iter().take(32) {
                    rows.push((c.bitstring(&k), cnt as f64 / c.shots as f64, None));
                }
            }
        }
    }
    if rows.is_empty() {
        p.text(plot.center(), Align2::CENTER_CENTER, if app.bl == BottomLeft::Shots { tr("Apasă «Rulează» pentru măsurători") } else { tr("se calculează…") }, FontId::proportional(13.0), Color32::GRAY);
        return;
    }
    let slot = plot.width() / rows.len() as f32;
    let bw = (slot * 0.55).clamp(4.0, 30.0);
    for (i, (bits, pr, _)) in rows.iter().enumerate() {
        let x = plot.min.x + slot * (i as f32 + 0.5);
        let h = plot.height() * (*pr as f32).clamp(0.0, 1.0);
        p.rect_filled(Rect::from_min_max(pos2(x - bw / 2.0, plot.max.y - h), pos2(x + bw / 2.0, plot.max.y)), 0.0, Color32::from_rgb(0x33, 0x9a, 0xff));
        let label = if bits.len() > 12 { format!("{}…{}", &bits[..4], &bits[bits.len() - 4..]) } else { bits.clone() };
        if slot >= 11.0 {
            let galley = p.layout_no_wrap(label, FontId::monospace(9.0), Color32::GRAY);
            p.add(egui::epaint::TextShape::new(pos2(x - 5.0, plot.max.y + 4.0), galley, Color32::GRAY).with_angle(PI / 2.0));
        }
        if *pr > 0.004 && h > 12.0 {
            p.text(pos2(x, plot.max.y - h - 7.0), Align2::CENTER_CENTER, format!("{:.0}", pr * 100.0), FontId::proportional(9.0), Color32::LIGHT_GRAY);
        }
    }
    p.text(pos2(rect.min.x + 4.0, rect.min.y + 6.0), Align2::LEFT_TOP, "%", FontId::proportional(10.0), Color32::GRAY);
}

// ---------------------------------------------------------------- UI principal

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

impl App {
    fn draw(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.poll(&ctx);
        if self.busy || self.run_busy {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        self.top_bar(ui, &ctx);
        self.palette_panel(ui, &ctx);
        self.code_panel(ui, &ctx);
        self.bottom_panels(ui);
        self.circuit_panel(ui, &ctx);
    }

    fn top_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Panel::top("top").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.label(egui::RichText::new("CUANTOR Composer").strong().color(Color32::from_rgb(0x8a, 0xb4, 0xff)));
                ui.separator();
                ui.menu_button(tr("Fișier"), |ui| {
                    if ui.button(tr("Nou")).clicked() {
                        self.mutate(ctx, |d| d.cells.clear());
                        ui.close();
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(tr("Cale:"));
                        ui.text_edit_singleline(&mut self.file_path);
                    });
                    if ui.button(tr("Deschide .qasm")).clicked() {
                        match std::fs::read_to_string(&self.file_path) {
                            Ok(t) => {
                                self.load_qasm(ctx, &t);
                                self.status = trf("Deschis {}", &[&self.file_path]);
                            }
                            Err(e) => self.status = trf("Nu pot citi {}: {}", &[&self.file_path, &e.to_string()]),
                        }
                        ui.close();
                    }
                    if ui.button(tr("Salvează .qasm (OpenQASM 3)")).clicked() {
                        let t = self.program_upto(None).to_qasm3();
                        self.status = match std::fs::write(&self.file_path, t) {
                            Ok(()) => trf("Salvat {}", &[&self.file_path]),
                            Err(e) => trf("Nu pot scrie {}: {}", &[&self.file_path, &e.to_string()]),
                        };
                        ui.close();
                    }
                    ui.separator();
                    ui.menu_button(tr("Exemple"), |ui| {
                        let mut last = "";
                        for e in EXAMPLES {
                            if e.group != last {
                                ui.label(egui::RichText::new(e.group).weak());
                                last = e.group;
                            }
                            if ui.button(e.title).on_hover_text(e.desc).clicked() {
                                self.load_qasm(ctx, e.qasm);
                                ui.close();
                            }
                        }
                    });
                });
                ui.menu_button(tr("Editare"), |ui| {
                    if ui.add_enabled(!self.undo.is_empty(), egui::Button::new(tr("Anulează"))).clicked() {
                        if let Some(d) = self.undo.pop() {
                            self.redo.push(self.doc.clone());
                            self.doc = d;
                            self.changed(ctx);
                        }
                        ui.close();
                    }
                    if ui.add_enabled(!self.redo.is_empty(), egui::Button::new(tr("Refă"))).clicked() {
                        if let Some(d) = self.redo.pop() {
                            self.undo.push(self.doc.clone());
                            self.doc = d;
                            self.changed(ctx);
                        }
                        ui.close();
                    }
                    if ui.button(tr("Golește circuitul")).clicked() {
                        self.mutate(ctx, |d| d.cells.clear());
                        ui.close();
                    }
                });
                ui.menu_button(tr("Vizualizare"), |ui| {
                    ui.checkbox(&mut self.inspect, tr("Inspectare (matricea porții la hover)"));
                });
                ui.separator();
                ui.label(tr("Qubiți:"));
                let mut n = self.doc.n;
                egui::ComboBox::from_id_salt("nq").selected_text(n.to_string()).width(60.0).show_ui(ui, |ui| {
                    for s in SIZES {
                        ui.selectable_value(&mut n, s, s.to_string());
                    }
                });
                let mut dn = n;
                ui.add(egui::DragValue::new(&mut dn).range(1..=cuantor_core::MAX_QUBITS_TOTAL).speed(0.2));
                if dn != self.doc.n {
                    self.mutate(ctx, |d| d.n = dn);
                } else if n != self.doc.n {
                    self.mutate(ctx, |d| d.n = n);
                }
                ui.label(tr("Motor:"));
                let before = self.backend;
                egui::ComboBox::from_id_salt("be").selected_text(self.backend.name()).show_ui(ui, |ui| {
                    for b in Backend::ALL {
                        ui.selectable_value(&mut self.backend, b, b.name());
                    }
                });
                if before != self.backend {
                    self.changed(ctx);
                }
                ui.label(tr("Limbă:"));
                let mut lang = i18n::get();
                egui::ComboBox::from_id_salt("lang").selected_text(lang.name()).show_ui(ui, |ui| {
                    for l in Lang::ALL {
                        ui.selectable_value(&mut lang, l, l.name());
                    }
                });
                if lang != i18n::get() {
                    i18n::set(lang);
                    lang.save();
                    self.refresh_code();
                }
                let run = ui.add_enabled(!self.run_busy, egui::Button::new(egui::RichText::new(tr("Rulează")).strong()).fill(Color32::from_rgb(0x42, 0x3b, 0xd9)));
                if run.clicked() {
                    self.run_shots(ctx);
                }
            });
            ui.horizontal(|ui| {
                let p = self.program_upto(None);
                let active = choose_backend(&p, self.backend).map(|b| b.name().to_string()).unwrap_or_else(|e| format!("⚠ {e}"));
                ui.small(format!("{} {active}", tr("Motor activ:")));
                if let Some(a) = &self.analysis {
                    ui.small(format!("· {} ≈ {}", tr("memorie"), human_bytes(a.memory_bytes)));
                    if let Some(s) = a.support {
                        ui.small(format!("· {s} {}", tr("amplitudini nenule")));
                    }
                }
                if self.busy {
                    ui.small(format!("· {}", tr("calculez…")));
                }
                if !self.status.is_empty() {
                    ui.small(format!("· {}", self.status));
                }
            });
        });
    }

    fn palette_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Panel::left("palette").default_size(250.0).resizable(true).show(ui, |ui| {
            ui.label(egui::RichText::new(tr("Operații")).strong());
            ui.add_space(4.0);
            egui::Grid::new("pal").spacing([4.0, 4.0]).show(ui, |ui| {
                for (i, t) in PALETTE.iter().enumerate() {
                    let sel = self.tool == *t;
                    let (rect, resp) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::click());
                    let p = ui.painter();
                    p.rect_filled(rect, 3.0, t.color());
                    if sel {
                        p.rect_stroke(rect, 3.0, Stroke::new(2.5, Color32::WHITE), StrokeKind::Outside);
                    }
                    p.text(rect.center(), Align2::CENTER_CENTER, t.label(), FontId::proportional(14.0), Color32::WHITE);
                    if resp.on_hover_text(t.tip()).clicked() {
                        self.tool = *t;
                        self.pending = None;
                        if *t != Tool::Select {
                            self.selected = None;
                        }
                    }
                    if (i + 1) % 6 == 0 {
                        ui.end_row();
                    }
                }
            });
            ui.add_space(6.0);
            let needs = self.tool.has_angle() || self.selected.is_some();
            if needs {
                ui.horizontal(|ui| {
                    ui.label(tr("Unghi (rad):"));
                    let r = ui.add(egui::TextEdit::singleline(&mut self.angle_text).desired_width(90.0));
                    if r.changed() {
                        if let (Some(i), Some(a)) = (self.selected, parse_angle(&self.angle_text)) {
                            if !self.sel_dirty {
                                self.undo.push(self.doc.clone());
                                self.redo.clear();
                                self.sel_dirty = true;
                            }
                            if let Some(Cell { item: Item::Gate(o), .. }) = self.doc.cells.get_mut(i) {
                                o.kind = o.kind.with_angle(a);
                            }
                            self.changed(ctx);
                        }
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    for (l, v) in [("π/4", "pi/4"), ("π/3", "pi/3"), ("π/2", "pi/2"), ("π", "pi"), ("−π/2", "-pi/2")] {
                        if ui.small_button(l).clicked() {
                            self.angle_text = v.into();
                            if let (Some(i), Some(a)) = (self.selected, parse_angle(v)) {
                                self.undo.push(self.doc.clone());
                                if let Some(Cell { item: Item::Gate(o), .. }) = self.doc.cells.get_mut(i) {
                                    o.kind = o.kind.with_angle(a);
                                }
                                self.changed(ctx);
                            }
                        }
                    }
                });
                match parse_angle(&self.angle_text) {
                    Some(a) => ui.small(format!("= {a:.4} rad ({})", fmt_angle(a))),
                    None => ui.small(egui::RichText::new(tr("expresie invalidă")).color(Color32::LIGHT_RED)),
                };
            }
            ui.separator();
            if self.tool.arity() > 1 {
                let k = self.pending.as_ref().map_or(0, |(_, q)| q.len());
                ui.small(trf("{}: alege {} (atinge un fir; toți qubiții pe aceeași coloană).", &[self.tool.tip(), self.tool.role(k)]));
            } else {
                ui.small(self.tool.tip());
            }
            ui.separator();
            egui::CollapsingHeader::new(tr("Execuție și zgomot")).default_open(true).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(tr("Măsurători:"));
                    ui.add(egui::DragValue::new(&mut self.shots).range(1..=1_000_000).speed(16.0));
                });
                ui.add(egui::Slider::new(&mut self.noise.depol1, 0.0..=0.1).text(tr("depolarizare 1q")));
                ui.add(egui::Slider::new(&mut self.noise.depol2, 0.0..=0.1).text(tr("depolarizare 2q")));
                ui.add(egui::Slider::new(&mut self.noise.readout, 0.0..=0.2).text(tr("eroare citire")));
            });
            egui::CollapsingHeader::new(tr("IBM Quantum")).default_open(false).show(ui, |ui| {
                ui.small(tr("Cheia API și CRN se citesc local din variabile de mediu sau ~/.config/cuantor/ibm.json; nu se salvează în aplicație."));
                ui.horizontal(|ui| {
                    ui.label(tr("Backend:"));
                    ui.add(egui::TextEdit::singleline(&mut self.ibm_backend).desired_width(120.0).hint_text("ex. ibm_brisbane"));
                });
                if ui.add_enabled(!self.ibm_busy, egui::Button::new(tr("Trimite pe hardware IBM"))).on_hover_text(tr("Transpilează în porți native și trimite un job Sampler (neverificat pe hardware real)")).clicked() {
                    self.run_ibm(ctx);
                }
                for l in &self.ibm_out {
                    ui.monospace(l);
                }
                if ui.button(tr("Deschide în IBM Composer")).on_hover_text(tr("Salvează circuitul ca .qasm și deschide Composer; apoi Fișier → Upload .qasm file")).clicked() {
                    let path = std::env::temp_dir().join("cuantor_composer.qasm");
                    let t = self.program_upto(None).to_qasm3();
                    self.status = match std::fs::write(&path, t) {
                        Ok(()) => {
                            open_url("https://quantum.cloud.ibm.com/composer");
                            trf("Salvat {} — încarcă-l în Composer (File → Upload .qasm file)", &[&path.display().to_string()])
                        }
                        Err(e) => trf("Nu pot scrie fișierul: {}", &[&e.to_string()]),
                    };
                }
            });
        });
    }

    fn code_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Panel::right("code").default_size(330.0).resizable(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                let before = self.code_fmt;
                egui::ComboBox::from_id_salt("fmt")
                    .selected_text(match self.code_fmt {
                        CodeFmt::Qasm3 => "OpenQASM 3.0",
                        CodeFmt::Qasm2 => "OpenQASM 2.0",
                        CodeFmt::Qiskit => "Qiskit (doar citire)",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.code_fmt, CodeFmt::Qasm3, "OpenQASM 3.0");
                        ui.selectable_value(&mut self.code_fmt, CodeFmt::Qasm2, "OpenQASM 2.0");
                        ui.selectable_value(&mut self.code_fmt, CodeFmt::Qiskit, "Qiskit (doar citire)");
                    });
                if before != self.code_fmt {
                    self.code_dirty = false;
                    self.refresh_code();
                }
                if ui.button(tr("Copiază")).clicked() {
                    ctx.copy_text(self.code.clone());
                }
            });
            let editable = self.code_fmt != CodeFmt::Qiskit;
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut text = self.code.clone();
                let r = ui.add_enabled(editable, egui::TextEdit::multiline(&mut text).font(FontId::monospace(12.0)).desired_width(f32::INFINITY).desired_rows(24));
                if r.changed() {
                    self.code = text;
                    self.code_dirty = true;
                }
            });
            if editable && self.code_dirty && ui.button(tr("Aplică în circuit")).clicked() {
                let t = self.code.clone();
                self.load_qasm(ctx, &t);
            }
            if !self.code_err.is_empty() {
                ui.colored_label(Color32::LIGHT_RED, &self.code_err);
            }
        });
    }

    fn bottom_panels(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("bottom").default_size(300.0).resizable(true).show(ui, |ui| {
            let w = ui.available_width();
            let h = ui.available_height().max(180.0);
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(w * 0.5 - 8.0);
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("bl").selected_text(if self.bl == BottomLeft::Probs { tr("Probabilități") } else { tr("Rezultate măsurare") }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.bl, BottomLeft::Probs, tr("Probabilități"));
                            ui.selectable_value(&mut self.bl, BottomLeft::Shots, tr("Rezultate măsurare"));
                        });
                        if let Some(a) = &self.analysis {
                            if !a.top_exact && self.bl == BottomLeft::Probs {
                                ui.small(trf("eșantion din {} rulări", &[&a.samples_used.to_string()]));
                            }
                        }
                        if self.run_busy {
                            ui.spinner();
                        }
                    });
                    if let Some(e) = &self.analysis_err {
                        ui.colored_label(Color32::LIGHT_RED, e);
                    }
                    draw_probs(ui, self, vec2(ui.available_width(), h - 34.0));
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("br").selected_text(if self.br == BottomRight::QSphere { tr("Q-sferă") } else { tr("Vector de stare") }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.br, BottomRight::QSphere, tr("Q-sferă"));
                            ui.selectable_value(&mut self.br, BottomRight::StateVec, tr("Vector de stare"));
                        });
                        ui.checkbox(&mut self.show_state, tr("Stare"));
                        ui.checkbox(&mut self.show_phase, tr("Unghi de fază"));
                    });
                    match self.br {
                        BottomRight::QSphere => {
                            let size = vec2(ui.available_width(), h - 34.0);
                            draw_qsphere(ui, self, size);
                        }
                        BottomRight::StateVec => {
                            egui::ScrollArea::vertical().max_height(h - 40.0).show(ui, |ui| {
                                if let Some(a) = &self.analysis {
                                    if !a.stabilizers.is_empty() {
                                        ui.label("Generatori de stabilizatori:");
                                        for s in &a.stabilizers {
                                            ui.monospace(s);
                                        }
                                        ui.separator();
                                    }
                                    for t in &a.top {
                                        match t.amp {
                                            Some(z) => ui.monospace(format!("|{}>  {:+.4}{:+.4}i   {:.2}%", t.bits, z.re, z.im, t.prob * 100.0)),
                                            None => ui.monospace(format!("|{}>  ≈ {:.2}%", t.bits, t.prob * 100.0)),
                                        };
                                    }
                                }
                            });
                        }
                    }
                });
            });
        });
    }

    fn circuit_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let total = self.total_cols();
                let mut k = self.step.unwrap_or(total).min(total);
                ui.label(tr("Starea după pasul:"));
                if ui.add_enabled(total > 0, egui::Slider::new(&mut k, 0..=total.max(1))).changed() {
                    self.step = if k >= total { None } else { Some(k) };
                    self.spawn_analysis(ctx);
                }
            });
            let n = self.doc.n;
            let cols = (self.total_cols() + 6).max(16);
            let size = vec2(LABEL_W + cols as f32 * CELL + DIAL_W + 20.0, n as f32 * CELL + 6.0);
            let occ: std::collections::HashMap<(usize, usize), usize> = self.doc.cells.iter().enumerate().flat_map(|(i, c)| c.qubits().into_iter().map(move |q| ((q, c.col), i))).collect();
            let mut clicked: Option<(usize, usize)> = None;
            let mut hover_tip: Option<String> = None;
            egui::ScrollArea::both().auto_shrink([false, false]).show_viewport(ui, |ui, viewport| {
                let (resp, p) = ui.allocate_painter(size, Sense::click());
                let origin = resp.rect.min;
                let r0 = ((viewport.min.y / CELL).floor().max(0.0) as usize).min(n);
                let r1 = (((viewport.max.y / CELL).ceil() as usize) + 1).min(n);
                let c0 = (((viewport.min.x - LABEL_W) / CELL).floor().max(0.0) as usize).min(cols);
                let c1 = ((((viewport.max.x - LABEL_W) / CELL).ceil().max(0.0) as usize) + 1).min(cols);
                let wire = Color32::from_gray(95);
                for q in r0..r1 {
                    let y = origin.y + q as f32 * CELL + CELL / 2.0;
                    p.line_segment([pos2(origin.x + viewport.min.x.max(0.0) + LABEL_W - 4.0, y), pos2(origin.x + LABEL_W + cols as f32 * CELL, y)], Stroke::new(1.5, wire));
                }
                // conectori verticali
                for c in &self.doc.cells {
                    let qs = c.qubits();
                    if qs.len() > 1 && c.col >= c0 && c.col < c1 {
                        let (mn, mx) = (*qs.iter().min().unwrap(), *qs.iter().max().unwrap());
                        let x = origin.x + LABEL_W + c.col as f32 * CELL + CELL / 2.0;
                        p.line_segment([pos2(x, origin.y + mn as f32 * CELL + CELL / 2.0), pos2(x, origin.y + mx as f32 * CELL + CELL / 2.0)], Stroke::new(2.0, Color32::from_rgb(0x9c, 0x8c, 0xff)));
                    }
                }
                let ptr = resp.hover_pos();
                for q in r0..r1 {
                    for c in c0..c1 {
                        let center = pos2(origin.x + LABEL_W + c as f32 * CELL + CELL / 2.0, origin.y + q as f32 * CELL + CELL / 2.0);
                        let rect = Rect::from_center_size(center, vec2(CELL - 2.0, CELL - 2.0));
                        let pend = self.pending.as_ref().is_some_and(|(pc, qs)| *pc == c && qs.contains(&q));
                        if pend {
                            p.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(255, 200, 80, 60));
                        } else if ptr.is_some_and(|pp| rect.contains(pp)) {
                            p.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(120, 160, 255, 28));
                        }
                        if let Some(&i) = occ.get(&(q, c)) {
                            let cell = &self.doc.cells[i];
                            let sel = self.selected == Some(i);
                            match &cell.item {
                                Item::Measure(_) => {
                                    p.rect_filled(Rect::from_center_size(center, vec2(34.0, 34.0)), 4.0, GREY);
                                    p.text(center, Align2::CENTER_CENTER, "M", FontId::proportional(15.0), Color32::WHITE);
                                }
                                Item::Gate(o) => {
                                    let pos_in = o.qubits().iter().position(|&x| x == q).unwrap_or(0);
                                    let nc = o.controls.len();
                                    let is_ctrl = pos_in < nc;
                                    let base = o.label();
                                    let col = match &o.kind {
                                        Kind::H => RED_ORANGE,
                                        Kind::X | Kind::Y | Kind::Sx | Kind::Rx(_) | Kind::Ry(_) => CRIMSON,
                                        Kind::Swap => PURPLE,
                                        _ => BLUE,
                                    };
                                    if is_ctrl {
                                        p.circle_filled(center, 5.5, Color32::from_rgb(0x9c, 0x8c, 0xff));
                                    } else if o.kind == Kind::Swap {
                                        p.text(center, Align2::CENTER_CENTER, "×", FontId::proportional(24.0), Color32::from_rgb(0x9c, 0x8c, 0xff));
                                    } else if o.kind == Kind::X && nc > 0 {
                                        p.circle_filled(center, 12.0, PURPLE);
                                        p.line_segment([center - vec2(7.0, 0.0), center + vec2(7.0, 0.0)], Stroke::new(2.0, Color32::WHITE));
                                        p.line_segment([center - vec2(0.0, 7.0), center + vec2(0.0, 7.0)], Stroke::new(2.0, Color32::WHITE));
                                    } else if o.kind == Kind::Z && nc > 0 {
                                        p.circle_filled(center, 5.5, Color32::from_rgb(0x9c, 0x8c, 0xff));
                                    } else {
                                        let lab = if nc > 0 { base.trim_start_matches('C').to_string() } else { base.clone() };
                                        let sub = o.kind.angle().map(fmt_angle);
                                        draw_gate_box(&p, center, &lab, col, sub.as_deref());
                                    }
                                    if sel {
                                        p.rect_stroke(Rect::from_center_size(center, vec2(40.0, 40.0)), 4.0, Stroke::new(2.0, Color32::WHITE), StrokeKind::Outside);
                                    }
                                    if self.inspect && ptr.is_some_and(|pp| rect.contains(pp)) {
                                        let m = o.kind.matrix();
                                        let f = |z: cuantor_core::C64| format!("{:+.3}{:+.3}i", z.re, z.im);
                                        hover_tip = Some(format!("{} sur q{:?}\n[{}  {}]\n[{}  {}]", o.label(), o.qubits(), f(m[0][0]), f(m[0][1]), f(m[1][0]), f(m[1][1])));
                                    }
                                }
                            }
                        }
                    }
                }
                // linia pasului curent
                if let Some(k) = self.step {
                    let x = origin.x + LABEL_W + k as f32 * CELL;
                    p.line_segment([pos2(x, origin.y), pos2(x, origin.y + n as f32 * CELL)], Stroke::new(1.5, Color32::from_rgb(0xff, 0xc8, 0x57)));
                }
                // etichete și cadrane „lipite” de marginea vizibilă
                let lx = origin.x + viewport.min.x;
                let rx = origin.x + viewport.max.x - DIAL_W / 2.0 - 4.0;
                let bg = Color32::from_rgb(0x16, 0x16, 0x1d);
                p.rect_filled(Rect::from_min_max(pos2(lx, origin.y + viewport.min.y), pos2(lx + LABEL_W - 6.0, origin.y + viewport.max.y)), 0.0, bg);
                p.rect_filled(Rect::from_min_max(pos2(origin.x + viewport.max.x - DIAL_W, origin.y + viewport.min.y), pos2(origin.x + viewport.max.x, origin.y + viewport.max.y)), 0.0, bg);
                for q in r0..r1 {
                    let y = origin.y + q as f32 * CELL + CELL / 2.0;
                    p.text(pos2(lx + 6.0, y), Align2::LEFT_CENTER, format!("q[{q}]"), FontId::monospace(12.0), Color32::from_gray(190));
                    if let Some(a) = &self.analysis {
                        if a.n == n {
                            draw_dial(&p, pos2(rx, y), a.p_one.get(q).copied().unwrap_or(0.0));
                        }
                    }
                }
                if resp.clicked() {
                    if let Some(pp) = resp.interact_pointer_pos() {
                        let rel = pp - origin;
                        if rel.x >= LABEL_W {
                            let c = ((rel.x - LABEL_W) / CELL) as usize;
                            let q = (rel.y / CELL) as usize;
                            if q < n && c < cols {
                                clicked = Some((q, c));
                            }
                        }
                    }
                }
            });
            if let Some(t) = hover_tip {
                egui::Tooltip::always_open(ctx.clone(), ui.layer_id(), egui::Id::new("insp"), egui::PopupAnchor::Pointer).show(|ui| {
                    ui.monospace(t);
                });
            }
            if let Some((q, c)) = clicked {
                self.click_cell(ctx, q, c);
            }
        });
    }
}

fn human_bytes(b: u128) -> String {
    const U: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut v = b as f64;
    let mut i = 0;
    while v >= 1024.0 && i < U.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    format!("{v:.1} {}", U[i])
}

fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1360.0, 860.0]).with_title("CUANTOR Composer"),
        ..Default::default()
    };
    eframe::run_native("CUANTOR Composer", opts, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(app: &mut App, ctx: &egui::Context, clicks: Vec<egui::Pos2>) {
        let mut events = vec![];
        for p in clicks {
            events.push(egui::Event::PointerMoved(p));
            events.push(egui::Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() });
            events.push(egui::Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() });
        }
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1360.0, 860.0))),
            events,
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| app.draw(ui));
    }

    #[test]
    fn renders_and_scales_without_panics() {
        let ctx = egui::Context::default();
        let mut app = App::with_ctx(&ctx);
        for _ in 0..3 {
            frame(&mut app, &ctx, vec![]);
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
        assert!(app.analysis.is_some());
        // Bell
        app.load_qasm(&ctx, EXAMPLES.iter().find(|e| e.title.contains("Bell")).unwrap().qasm);
        assert_eq!(app.doc.n, 2);
        // 1000 de qubiți
        app.mutate(&ctx, |d| d.n = 1000);
        for _ in 0..40 {
            frame(&mut app, &ctx, vec![]);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        let a = app.analysis.as_ref().expect("analiză 1000 qubiți");
        assert_eq!(a.n, 1000);
        assert_eq!(a.backend, Backend::Stabilizer);
        // GHZ pe 1000 prin QASM
        let mut q = String::from("OPENQASM 2.0;\nqreg q[1000];\nh q[0];\n");
        for i in 0..999 {
            q.push_str(&format!("cx q[{}],q[{}];\n", i, i + 1));
        }
        app.load_qasm(&ctx, &q);
        assert_eq!(app.doc.n, 1000);
        for _ in 0..80 {
            frame(&mut app, &ctx, vec![]);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        let a = app.analysis.as_ref().unwrap();
        assert!(a.top.iter().all(|t| t.bits.chars().all(|c| c == '0') || t.bits.chars().all(|c| c == '1')));
        assert!((a.p_one[500] - 0.5).abs() < 1e-9);
        assert!(app.code.contains("qubit[1000] q;"));
        // randare cu Q-sferă / vector de stare
        app.br = BottomRight::StateVec;
        frame(&mut app, &ctx, vec![]);
    }

    #[test]
    fn editor_places_gates_by_click_logic() {
        let ctx = egui::Context::default();
        let mut app = App::with_ctx(&ctx);
        app.doc.cells.clear();
        app.tool = Tool::H;
        app.click_cell(&ctx, 1, 0);
        app.tool = Tool::Cx;
        app.click_cell(&ctx, 1, 1);
        assert!(app.pending.is_some());
        app.click_cell(&ctx, 2, 1);
        assert_eq!(app.doc.cells.len(), 2);
        app.tool = Tool::Ccx;
        app.click_cell(&ctx, 0, 2);
        app.click_cell(&ctx, 1, 2);
        app.click_cell(&ctx, 2, 2);
        assert_eq!(app.doc.cells.len(), 3);
        app.tool = Tool::Rz;
        app.angle_text = "pi/3".into();
        app.click_cell(&ctx, 0, 3);
        let p = app.program_upto(None);
        assert_eq!(p.gates().count(), 4);
        assert!(p.to_qasm3().contains("rz(pi/3) q[0];"));
        app.tool = Tool::Erase;
        app.click_cell(&ctx, 0, 3);
        assert_eq!(app.doc.cells.len(), 3);
        // anulare
        let before = app.doc.cells.len();
        if let Some(d) = app.undo.pop() {
            app.doc = d;
        }
        assert!(app.doc.cells.len() <= before + 1);
    }

    #[test]
    fn adversarial_ui_inputs_never_panic() {
        let ctx = egui::Context::default();
        let mut app = App::with_ctx(&ctx);
        // circuite dinamice nu pot fi desenate: eroare, nu panică
        app.load_qasm(&ctx, "OPENQASM 2.0;\nqreg q[2];\ncreg c[2];\nmeasure q[0] -> c[0];\nif(c==1) x q[1];\n");
        assert!(!app.code_err.is_empty());
        app.load_qasm(&ctx, "gunoi ))) ;;; qreg");
        assert!(!app.code_err.is_empty());
        // unghi invalid la plasare
        app.tool = Tool::Rz;
        app.angle_text = "((((".into();
        let before = app.doc.cells.len();
        app.click_cell(&ctx, 0, 5);
        assert_eq!(app.doc.cells.len(), before);
        // celule foarte departe / fire reduse la 1 cu porți pe fire șterse
        app.tool = Tool::Cx;
        app.click_cell(&ctx, 0, 40);
        app.click_cell(&ctx, 3, 40);
        app.mutate(&ctx, |d| d.n = 1);
        assert!(app.doc.cells.iter().all(|c| c.qubits().iter().all(|&q| q < 1)));
        // 4096 qubiți + poartă lată
        app.mutate(&ctx, |d| d.n = 4096);
        app.tool = Tool::H;
        app.click_cell(&ctx, 4095, 0);
        app.tool = Tool::Ccx;
        for q in [0, 2048, 4095] {
            app.click_cell(&ctx, q, 1);
        }
        for _ in 0..30 {
            frame(&mut app, &ctx, vec![]);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        // anulări repetate peste limită
        for _ in 0..500 {
            app.mutate(&ctx, |d| d.cells.clear());
        }
        assert!(app.undo.len() <= 200);
        // măsurare 0 și zgomot extrem
        app.shots = 1;
        app.noise.depol1 = 0.1;
        app.run_shots(&ctx);
        std::thread::sleep(std::time::Duration::from_millis(300));
        let c2 = ctx.clone();
        app.poll(&c2);
    }

    #[test]
    fn every_palette_tool_is_translated() {
        for lang in Lang::ALL {
            i18n::set(lang);
            for t in PALETTE {
                let lab = t.label_ro();
                for key in [Some(t.tip_ro()), (lab == "Șt" || lab == "Sel").then_some(lab)].into_iter().flatten() {
                    assert!(i18n::TABLE.iter().any(|r| r.0 == key), "neprevăzut în tabel: «{key}»");
                }
            }
        }
        i18n::set(Lang::Ro);
    }

    #[test]
    fn app_renders_in_every_language() {
        let ctx = egui::Context::default();
        let mut app = App::with_ctx(&ctx);
        for lang in Lang::ALL {
            i18n::set(lang);
            frame(&mut app, &ctx, vec![]);
        }
        i18n::set(Lang::Ro);
    }
}
