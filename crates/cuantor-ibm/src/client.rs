//! Client Qiskit Runtime REST (IBM Quantum Platform). Cheia API nu este niciodată afișată sau scrisă în log.

use crate::transpile::{Basis, Target, Transpiled, transpile};
use cuantor_core::Program;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Config {
    pub api_key: String,
    pub crn: String,
    pub backend: String,
    pub iam_url: String,
    pub base_url: String,
    /// Antet IBM-API-Version (gol = nu se trimite). NEVERIFICAT: ajustează dacă serviciul îl cere.
    pub api_version: String,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config{{backend:{}, base_url:{}, api_key:<ascunsă>}}", self.backend, self.base_url)
    }
}

impl Config {
    /// Variabile de mediu: IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN, IBM_QUANTUM_BACKEND (+ opționale
    /// IBM_QUANTUM_BASE_URL, IBM_QUANTUM_IAM_URL, IBM_QUANTUM_API_VERSION), sau fișierul JSON local
    /// `~/.config/cuantor/ibm.json` cu cheile api_key, crn, backend.
    pub fn load() -> Result<Config, String> {
        let file: Value = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .and_then(|h| std::fs::read_to_string(std::path::Path::new(&h).join(".config/cuantor/ibm.json")).ok())
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or(Value::Null);
        let get = |env: &str, key: &str| std::env::var(env).ok().or_else(|| file[key].as_str().map(String::from));
        Ok(Config {
            api_key: get("IBM_QUANTUM_API_KEY", "api_key").ok_or("lipsește IBM_QUANTUM_API_KEY (sau api_key în ~/.config/cuantor/ibm.json)")?,
            crn: get("IBM_QUANTUM_CRN", "crn").ok_or("lipsește IBM_QUANTUM_CRN (CRN-ul instanței din quantum.cloud.ibm.com)")?,
            backend: get("IBM_QUANTUM_BACKEND", "backend").unwrap_or_default(),
            iam_url: get("IBM_QUANTUM_IAM_URL", "iam_url").unwrap_or_else(|| "https://iam.cloud.ibm.com".into()),
            base_url: get("IBM_QUANTUM_BASE_URL", "base_url").unwrap_or_else(|| "https://quantum.cloud.ibm.com/api/v1".into()),
            api_version: get("IBM_QUANTUM_API_VERSION", "api_version").unwrap_or_default(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct BackendConfig {
    pub name: String,
    pub n_qubits: usize,
    pub basis_gates: Vec<String>,
    pub coupling_map: Vec<(usize, usize)>,
}

impl BackendConfig {
    pub fn from_json(name: &str, v: &Value) -> Result<BackendConfig, String> {
        let n_qubits = v["n_qubits"].as_u64().ok_or("configurare fără n_qubits")? as usize;
        let basis_gates = v["basis_gates"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
        let coupling_map = v["coupling_map"]
            .as_array()
            .map(|a| a.iter().filter_map(|p| Some((p.get(0)?.as_u64()? as usize, p.get(1)?.as_u64()? as usize))).collect())
            .unwrap_or_default();
        Ok(BackendConfig { name: name.into(), n_qubits, basis_gates, coupling_map })
    }
    pub fn target(&self) -> Target<'_> {
        Target { n_qubits: self.n_qubits, coupling: &self.coupling_map, basis: Basis::from_gates(&self.basis_gates) }
    }
}

pub struct Client {
    pub cfg: Config,
    token: Option<(String, Instant)>,
}

fn http_err(e: ureq::Error) -> String {
    match e {
        ureq::Error::Status(code, r) => {
            let body = r.into_string().unwrap_or_default();
            format!("HTTP {code}: {}", body.chars().take(400).collect::<String>())
        }
        other => format!("rețea: {other}"),
    }
}

impl Client {
    pub fn new(cfg: Config) -> Client {
        Client { cfg, token: None }
    }

    fn bearer(&mut self) -> Result<String, String> {
        if let Some((t, exp)) = &self.token {
            if Instant::now() < *exp {
                return Ok(t.clone());
            }
        }
        let url = format!("{}/identity/token", self.cfg.iam_url.trim_end_matches('/'));
        let r: Value = ureq::post(&url)
            .set("Accept", "application/json")
            .send_form(&[("grant_type", "urn:ibm:params:oauth:grant-type:apikey"), ("apikey", &self.cfg.api_key)])
            .map_err(http_err)?
            .into_json()
            .map_err(|e| e.to_string())?;
        let t = r["access_token"].as_str().ok_or("răspuns IAM fără access_token")?.to_string();
        let ttl = r["expires_in"].as_u64().unwrap_or(3000).saturating_sub(120);
        self.token = Some((t.clone(), Instant::now() + Duration::from_secs(ttl)));
        Ok(t)
    }

    fn req(&mut self, method: &str, path: &str) -> Result<ureq::Request, String> {
        let tok = self.bearer()?;
        let mut r = ureq::request(method, &format!("{}{}", self.cfg.base_url.trim_end_matches('/'), path))
            .set("Authorization", &format!("Bearer {tok}"))
            .set("Service-CRN", &self.cfg.crn)
            .set("Accept", "application/json");
        if !self.cfg.api_version.is_empty() {
            r = r.set("IBM-API-Version", &self.cfg.api_version);
        }
        Ok(r)
    }

    fn get(&mut self, path: &str) -> Result<Value, String> {
        self.req("GET", path)?.call().map_err(http_err)?.into_json().map_err(|e| e.to_string())
    }

    pub fn list_backends(&mut self) -> Result<Vec<String>, String> {
        let v = self.get("/backends")?;
        let arr = v["devices"].as_array().or_else(|| v["backends"].as_array()).or_else(|| v.as_array()).ok_or("listă de backend-uri neașteptată")?;
        Ok(arr.iter().filter_map(|d| d["name"].as_str().or_else(|| d.as_str()).map(String::from)).collect())
    }

    pub fn backend_config(&mut self, name: &str) -> Result<BackendConfig, String> {
        let v = self.get(&format!("/backends/{name}/configuration"))?;
        BackendConfig::from_json(name, &v)
    }

    /// Transpilează și trimite un job Sampler. Întoarce (id job, circuitul ISA trimis).
    pub fn submit(&mut self, prog: &Program, shots: usize, bc: &BackendConfig) -> Result<(String, Transpiled), String> {
        let t = transpile(&(if prog.has_measure() { prog.clone() } else { prog.with_measure_all() }), &bc.target())?;
        let body = json!({
            "program_id": "sampler",
            "backend": bc.name,
            "params": { "pubs": [[t.to_qasm3(), Value::Null, shots]], "version": 2 }
        });
        let r: Value = self.req("POST", "/jobs")?.send_json(body).map_err(http_err)?.into_json().map_err(|e| e.to_string())?;
        let id = r["id"].as_str().ok_or("răspuns fără id de job")?.to_string();
        Ok((id, t))
    }

    pub fn status(&mut self, id: &str) -> Result<String, String> {
        let v = self.get(&format!("/jobs/{id}"))?;
        Ok(v["state"]["status"].as_str().or_else(|| v["status"].as_str()).unwrap_or("necunoscut").to_string())
    }

    pub fn wait(&mut self, id: &str, timeout: Duration, mut on_status: impl FnMut(&str)) -> Result<(), String> {
        let t0 = Instant::now();
        loop {
            let s = self.status(id)?;
            on_status(&s);
            match s.to_lowercase().as_str() {
                "completed" => return Ok(()),
                "failed" | "cancelled" | "canceled" => return Err(format!("job {s}")),
                _ => {}
            }
            if t0.elapsed() > timeout {
                return Err("timeout la așteptarea jobului".into());
            }
            std::thread::sleep(Duration::from_secs(3));
        }
    }

    /// Rezultate: bitstring (c[n-1]…c[0]) → număr.
    pub fn results(&mut self, id: &str) -> Result<Vec<(String, usize)>, String> {
        let v = self.get(&format!("/jobs/{id}/results"))?;
        decode_results(&v)
    }
}

pub fn decode_results(v: &Value) -> Result<Vec<(String, usize)>, String> {
    let data = &v["results"][0]["data"];
    let reg = data.as_object().and_then(|o| o.values().next()).ok_or("rezultat fără date")?;
    let nbits = reg["num_bits"].as_u64().unwrap_or(0) as usize;
    let samples = reg["samples"].as_array().ok_or("rezultat fără samples")?;
    let mut map: std::collections::BTreeMap<String, usize> = Default::default();
    for s in samples {
        let h = s.as_str().ok_or("sample nu e șir")?.trim_start_matches("0x");
        let mut bits = String::new();
        for ch in h.chars() {
            bits.push_str(&format!("{:04b}", ch.to_digit(16).ok_or("hex invalid")?));
        }
        let bits = bits.trim_start_matches('0').to_string();
        let w = nbits.max(bits.len());
        *map.entry(format!("{bits:0>w$}")).or_default() += 1;
    }
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(v)
}
