# CUANTOR

> Kvantumszámítógép-szimulátor az asztali gépedre: 5-től 1000 qubitig, IBM Quantum Composer stílusú felülettel.

**Egy játék-kvantumszámítógép a számítógépedben: pontos matematika, de nem valódi kvantum-hardver.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — egy szuszra:** A CUANTOR játék kvantumszámítógép a képernyődön. Színes kockákra kattintasz, megnyomod a Futtatást, és látod a választ. Színlelt, nem az igazi.

## Mit tud

- Rajztábla, mint az IBM Composere: színes építőkockákat (kapukat) választasz, vonalakra (qubitekre) teszed őket, és oszlopok meg egy golyó mutatja, mi történik. Egy szövegpanel élőben leírja ugyanazt az áramkört OpenQASM-ben és Qiskitben.
- Három számoló van benne: az egyik minden számot megjegyez (28 qubitig), egy villámgyors az egyszerű kapukhoz (4096 qubitig), a harmadik a kevés ágú áramkörökhöz való.
- Menet közben is dönthet (mérés, reset, if). Kitalálhatsz saját kapukat, adhatsz hozzá színlelt hibákat (zajt), és bármelyik eredményt megismételheted egy seed-del.
- OpenQASM 2.0 és 3.0 fájlokat olvas, OpenQASM 3.0-t vagy Qiskitet ment.
- Beszél az IBM Quantummal: nyisd meg az áramkört az IBM Composerben, vagy küldd el egy valódi IBM-gépre a fiókod kulcsával.
- 8 nyelven beszél: román, angol, francia, német, spanyol, olasz, portugál, magyar.
- Fut Windowson, macOS-en és Linuxon. Belül egyetlen Rust-projekt van négy résszel: mag, ablak, parancssor és IBM-kapcsolat.

> **ELI5 — egy szuszra:** Színes kockák, három számoló, 8 nyelv, minden számítógépen fut.

## Fizikai korlátok

Minden további qubit megduplázza a munkát, mint egy füzet, ahol minden új qubit duplájára növeli az oldalakat. 24 qubithez kb. 256 MiB kell, 28-hoz kb. 4 GiB. Utána már csak egyszerű minták működnek. Egy 1000 qubites csoport, amelyben mind egyetért (GHZ), azonnal lefut; egy tetszőleges 1000 qubites áramkör bármelyik hétköznapi számítógépnek lehetetlen.

> **ELI5 — egy szuszra:** Több qubit sokkal több munka. Az egyszerű minták lehetnek óriásiak, a kusza nem.

## Gyors kezdés

A legegyszerűbb: töltsd le a programot (lásd Letöltés), és kattints duplán a cuantor-gui-ra. Ha magad fordítanád, egy friss Rusttal:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — egy szuszra:** Letöltöd, duplán kattintasz a cuantor-gui-ra, és játszol.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: a CUANTOR-ban nyomd meg az „Megnyitás az IBM Composerben” gombot. Egy .qasm fájl mentődik. Az IBM Composerében válaszd a File → Upload .qasm file parancsot.
- Valódi gép: írd be az IBM-kulcsodat, a példányt (CRN) és a gép nevét az IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN és IBM_QUANTUM_BACKEND változókba (vagy a ~/.config/cuantor/ibm.json fájlba), aztán futtasd: `cuantor ibm run circuit.qasm --backend NÉV`. A kulcs csak az IBM felé hagyja el a gépedet.

> Őszinte figyelmeztetés: ezt a részt valódi IBM-gépen még nem próbáltuk ki, csak színlelt szerveren és a beépített szimulátorral. Apró javításokra szükség lehet. A reset vagy if tartalmú áramkörök nem küldhetők el.

> **ELI5 — egy szuszra:** Elküldheted az áramkört az IBM weboldalára vagy egy valódi IBM-gépre. A valódit még nem próbáltuk.

## Minőség és audit

A(z) 0.2.1 verzió előtt mindent megtettünk, hogy elrontsuk a programot: 59 automatikus teszt, értelmetlen adatok, a teljes memóriát felfaló áramkörök, lehetetlen szögek, a három számoló összevetése egymással, és hazudó, hamis IBM-válaszok. 7 hibát találtunk és javítottunk ki (lásd a változásnaplót).

> **ELI5 — egy szuszra:** Mindent megtettünk, hogy elrontsuk, 7 hibát találtunk, és kijavítottuk őket.

[Változásnapló](CHANGELOG.hu.md)

## Letöltés

A Windowsra, macOS-re (Apple-chip és Intel) és Linuxra készült programok minden kiadáshoz csatolva vannak. Válaszd ki lent a tiédet.

**[Legújabb kiadás](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Felhasználói útmutató (PDF)

Úgy elmagyarázva, mintha 10 évesnek szólna, minden fejezet végén egy 5 éves szintű mini-összefoglalóval. Ugyanaz az útmutató 8 nyelven.

**[Útmutató letöltése (PDF)](guide/CUANTOR-guide.hu.pdf)** · Minden nyelv: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — egy szuszra:** Válaszd ki a géped, töltsd le, kész. Az útmutató PDF.

## Licenc

Licenc még nincs megadva (a szerző döntése).

---
Claude segítségével készült. Csak szimuláció: kvantumelőnyt nem állít.
