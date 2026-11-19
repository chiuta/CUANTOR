#!/usr/bin/env python3
"""Generează ghidul de utilizare PDF (ELI10 + rezumate ELI5) în 8 limbi, cu Chrome headless.
Rulare:  python3 tools/gen_guide.py   → docs/guide/CUANTOR-guide.<lang>.pdf (+ copie în site/guide/)"""
import html, os, shutil, subprocess, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from eli_guide import G, LABELS

VER = "0.2.1"
LANGS = ["ro", "en", "fr", "de", "es", "it", "pt", "hu"]
CSS = """
@page{size:A4;margin:18mm 16mm}
*{box-sizing:border-box}
body{font:11.2pt/1.55 "DejaVu Sans","Noto Sans",system-ui,sans-serif;color:#1b1b28;margin:0}
.cover{padding:34mm 0 14mm;border-bottom:4px solid #6a6cff;margin-bottom:10mm}
.cover h1{font-size:30pt;margin:0 0 6mm;color:#2a2c9a;letter-spacing:.5px}
.cover p{margin:0;color:#444;font-size:12pt}
.ver{margin-top:6mm;color:#777;font-size:10pt}
.toc{margin:0 0 10mm;padding:6mm 8mm;background:#f2f3ff;border-radius:3mm}
.toc h2{margin:0 0 3mm;font-size:13pt;color:#2a2c9a}
.toc ol{margin:0;padding-left:6mm;columns:2;column-gap:10mm}
.toc li{margin:1mm 0;break-inside:avoid}
h2.ch{font-size:16pt;color:#2a2c9a;margin:9mm 0 3mm;break-after:avoid}
h2.ch span{display:inline-block;min-width:9mm;color:#fff;background:#6a6cff;border-radius:50%;text-align:center;margin-right:3mm;font-size:12pt;line-height:9mm;height:9mm}
p{margin:0 0 3mm}
ul,ol{margin:0 0 3mm;padding-left:6.5mm}
li{margin:0 0 1.6mm}
pre{background:#16161d;color:#e8e8f0;border-radius:2.5mm;padding:3.5mm 4.5mm;font:9.4pt/1.45 "DejaVu Sans Mono",monospace;white-space:pre-wrap;margin:0 0 3.5mm;break-inside:avoid}
.note{background:#fff7e0;border-left:1.2mm solid #f0b429;padding:2.6mm 4mm;margin:0 0 3.5mm;border-radius:0 2mm 2mm 0;break-inside:avoid}
.eli5{background:#e9fbef;border:.4mm solid #5be08a;border-radius:2.5mm;padding:2.6mm 4mm;margin:4mm 0 2mm;break-inside:avoid}
.eli5 b{color:#12803a}
.foot{margin-top:12mm;padding-top:3mm;border-top:.3mm solid #ccc;color:#777;font-size:9pt}
"""


def build(l):
    L, e = LABELS[l], html.escape
    toc = "".join(f"<li>{e(t)}</li>" for t, _, _ in G[l])
    out = [f'<!doctype html><html lang="{l}"><head><meta charset="utf-8"><title>{e(L["title"])}</title><style>{CSS}</style></head><body>',
           f'<div class="cover"><h1>{e(L["title"])}</h1><p>{e(L["sub"])}</p><div class="ver">CUANTOR {VER}</div></div>',
           f'<div class="toc"><h2>{e(L["toc"])}</h2><ol>{toc}</ol></div>']
    for i, (title, blocks, eli5) in enumerate(G[l], 1):
        out.append(f'<h2 class="ch"><span>{i}</span>{e(title)}</h2>')
        for kind, val in blocks:
            if kind == "p": out.append(f"<p>{e(val)}</p>")
            elif kind in ("ul", "ol"): out.append(f"<{kind}>" + "".join(f"<li>{e(x)}</li>" for x in val) + f"</{kind}>")
            elif kind == "code": out.append(f"<pre>{e(val)}</pre>")
            elif kind == "note": out.append(f'<div class="note">{e(val)}</div>')
        out.append(f'<div class="eli5"><b>{e(L["eli5"])}:</b> {e(eli5)}</div>')
    out.append(f'<div class="foot">{e(L["foot"])}</div></body></html>')
    return "".join(out)


def main():
    os.makedirs("docs/guide", exist_ok=True)
    os.makedirs("site/guide", exist_ok=True)
    tmp = tempfile.mkdtemp()
    for l in LANGS:
        h = os.path.join(tmp, f"g-{l}.html")
        open(h, "w", encoding="utf-8").write(build(l))
        pdf = os.path.abspath(f"docs/guide/CUANTOR-guide.{l}.pdf")
        subprocess.run(["google-chrome", "--headless=new", "--no-sandbox", "--disable-gpu", "--no-pdf-header-footer",
                        f"--print-to-pdf={pdf}", "file://" + h], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=120)
        shutil.copy(pdf, f"site/guide/CUANTOR-guide.{l}.pdf")
        print(l, os.path.getsize(pdf) // 1024, "KiB")


main()
