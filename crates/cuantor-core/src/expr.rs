use crate::{Result, err};
use std::collections::HashMap;
use std::f64::consts::PI;

/// Evaluează o expresie numerică OpenQASM: + - * / ^, paranteze, `pi`,
/// funcții (sin cos tan asin acos atan exp ln sqrt) și parametri numiți.
pub fn eval(src: &str, env: &HashMap<String, f64>) -> Result<f64> {
    let s: Vec<char> = src.replace('π', "pi").replace('−', "-").chars().filter(|c| !c.is_whitespace()).collect();
    if s.is_empty() {
        return Err(err!("expresie goală"));
    }
    let mut p = P { s, i: 0, env, depth: 0 };
    let v = p.expr()?;
    if p.i != p.s.len() {
        return Err(err!("caracter neașteptat '{}' în «{}»", p.s[p.i], src));
    }
    if !v.is_finite() {
        return Err(err!("valoare nefinită în «{}»", src));
    }
    Ok(v)
}

pub fn eval_const(src: &str) -> Result<f64> {
    eval(src, &HashMap::new())
}

struct P<'a> {
    s: Vec<char>,
    i: usize,
    env: &'a HashMap<String, f64>,
    depth: usize,
}

impl P<'_> {
    fn peek(&self) -> Option<char> {
        self.s.get(self.i).copied()
    }
    fn expr(&mut self) -> Result<f64> {
        let mut v = self.term()?;
        while let Some(c @ ('+' | '-')) = self.peek() {
            self.i += 1;
            let w = self.term()?;
            v = if c == '+' { v + w } else { v - w };
        }
        Ok(v)
    }
    fn term(&mut self) -> Result<f64> {
        let mut v = self.unary()?;
        while let Some(c @ ('*' | '/')) = self.peek() {
            self.i += 1;
            let w = self.unary()?;
            v = if c == '*' { v * w } else { v / w };
        }
        Ok(v)
    }
    fn enter(&mut self) -> Result<()> {
        self.depth += 1;
        if self.depth > 128 {
            return Err(err!("expresie imbricată prea adânc"));
        }
        Ok(())
    }
    fn unary(&mut self) -> Result<f64> {
        self.enter()?;
        let r = self.unary_inner();
        self.depth -= 1;
        r
    }
    fn unary_inner(&mut self) -> Result<f64> {
        match self.peek() {
            Some('-') => {
                self.i += 1;
                Ok(-self.unary()?)
            }
            Some('+') => {
                self.i += 1;
                self.unary()
            }
            _ => self.power(),
        }
    }
    fn power(&mut self) -> Result<f64> {
        let b = self.primary()?;
        if self.peek() == Some('^') {
            self.i += 1;
            let e = self.unary()?;
            return Ok(b.powf(e));
        }
        Ok(b)
    }
    fn primary(&mut self) -> Result<f64> {
        match self.peek() {
            Some('(') => {
                self.i += 1;
                self.enter()?;
                let v = self.expr();
                self.depth -= 1;
                let v = v?;
                if self.peek() != Some(')') {
                    return Err(err!("paranteză lipsă"));
                }
                self.i += 1;
                Ok(v)
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let st = self.i;
                while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
                    self.i += 1;
                }
                let id: String = self.s[st..self.i].iter().collect();
                let low = id.to_ascii_lowercase();
                if low == "pi" {
                    return Ok(PI);
                }
                if let Some(v) = self.env.get(&id) {
                    return Ok(*v);
                }
                let f: fn(f64) -> f64 = match low.as_str() {
                    "sin" => f64::sin,
                    "cos" => f64::cos,
                    "tan" => f64::tan,
                    "asin" => f64::asin,
                    "acos" => f64::acos,
                    "atan" => f64::atan,
                    "exp" => f64::exp,
                    "ln" => f64::ln,
                    "sqrt" => f64::sqrt,
                    _ => return Err(err!("simbol necunoscut: {id}")),
                };
                if self.peek() != Some('(') {
                    return Err(err!("lipsește ( după {id}"));
                }
                self.i += 1;
                let v = self.expr()?;
                if self.peek() != Some(')') {
                    return Err(err!("paranteză lipsă"));
                }
                self.i += 1;
                Ok(f(v))
            }
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let st = self.i;
                while self.peek().is_some_and(|c| c.is_ascii_digit() || c == '.') {
                    self.i += 1;
                }
                if matches!(self.peek(), Some('e' | 'E')) {
                    let save = self.i;
                    self.i += 1;
                    if matches!(self.peek(), Some('+' | '-')) {
                        self.i += 1;
                    }
                    if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                            self.i += 1;
                        }
                    } else {
                        self.i = save;
                    }
                }
                let t: String = self.s[st..self.i].iter().collect();
                t.parse::<f64>().map_err(|_| err!("număr invalid: {t}"))
            }
            Some(c) => Err(err!("caracter neașteptat '{c}'")),
            None => Err(err!("expresie incompletă")),
        }
    }
}
