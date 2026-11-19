# CUANTOR

> Un simulator de computer cuantic pentru desktop: de la 5 la 1000 de qubiți, cu interfață în stilul IBM Quantum Composer.

**E un calculator cuantic de jucărie în calculatorul tău: matematică exactă, dar nu hardware cuantic real.**

[Română](README.ro.md) · [English](../README.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Italiano](README.it.md) · [Português](README.pt.md) · [Magyar](README.hu.md)

> **ELI5 — pe scurt:** CUANTOR e un calculator cuantic de jucărie pe ecran. Apeși pe blocuri colorate, dai Rulează și vezi răspunsul. E doar de joacă, nu cel adevărat.

## Ce face

- O planșetă de desen ca Composer-ul IBM: alegi blocuri colorate (porți), le pui pe linii (qubiți) și vezi bare și o minge care arată ce se întâmplă. Un panou de text scrie în direct același circuit în OpenQASM și Qiskit.
- Trei calculatoare înăuntru: unul care ține minte fiecare număr (până la 28 de qubiți), unul foarte rapid pentru porți simple (până la 4096 de qubiți) și unul pentru circuite cu puține ramuri.
- Poate decide pe parcurs (măsurare, reset, if). Îți poți inventa porți, poți adăuga greșeli prefăcute (zgomot) și poți repeta orice rezultat cu o sămânță (seed).
- Citește fișiere OpenQASM 2.0 și 3.0 și salvează OpenQASM 3.0 sau Qiskit.
- Vorbește cu IBM Quantum: deschizi circuitul în IBM Composer sau îl trimiți pe o mașină IBM reală, cu cheia contului tău.
- Vorbește 8 limbi: română, engleză, franceză, germană, spaniolă, italiană, portugheză, maghiară.
- Merge pe Windows, macOS și Linux. Înăuntru e un singur proiect Rust cu patru părți: nucleu, fereastră, linie de comandă și legătura cu IBM.

> **ELI5 — pe scurt:** Blocuri colorate, trei calculatoare, 8 limbi, merge pe orice calculator.

## Limite fizice

Fiecare qubit în plus dublează munca, ca un caiet în care fiecare qubit nou dublează paginile. 24 de qubiți cer cam 256 MiB, 28 cam 4 GiB. Dincolo de asta merg doar tiparele simple. Un grup de 1000 de qubiți care se pun toți de acord (GHZ) merge instant; un circuit oarecare pe 1000 de qubiți e imposibil pentru orice calculator obișnuit.

> **ELI5 — pe scurt:** Mai mulți qubiți înseamnă mult mai multă muncă. Tiparele simple pot fi uriașe, cele încurcate nu.

## Pornire rapidă

Cel mai simplu: descarci programul (vezi Descărcare) și dai dublu clic pe cuantor-gui. Ca să-l construiești singur, cu un Rust recent:

```
git clone <repo-url> cuantor && cd cuantor
cargo run --release -p cuantor-gui
cargo run --release -p cuantor-cli -- example Bell
cargo run --release -p cuantor-cli -- ghz 1000 --shots 8
cargo test --workspace --release
```

> **ELI5 — pe scurt:** Descarci, dai dublu clic pe cuantor-gui și te joci.

## IBM Quantum (quantum.cloud.ibm.com)

- Composer: în CUANTOR apeși „Deschide în IBM Composer”. Se salvează un fișier .qasm. În Composer-ul IBM alegi File → Upload .qasm file.
- O mașină reală: pui cheia IBM, instanța (CRN) și numele mașinii în IBM_QUANTUM_API_KEY, IBM_QUANTUM_CRN și IBM_QUANTUM_BACKEND (sau în ~/.config/cuantor/ibm.json), apoi rulezi `cuantor ibm run circuit.qasm --backend NUME`. Cheia nu iese din calculatorul tău decât spre IBM.

> Avertisment sincer: partea aceasta nu a fost încă încercată pe o mașină IBM reală, doar pe un server simulat și cu simulatorul încorporat. Pot fi necesare mici corecturi. Circuitele cu reset sau if nu pot fi trimise.

> **ELI5 — pe scurt:** Îți poți trimite circuitul pe site-ul IBM sau pe o mașină IBM reală. Mașina reală încă n-am încercat-o.

## Calitate și audit

Înainte de versiunea 0.2.1 am încercat din greu să stricăm programul: 59 de teste automate, date aruncate la întâmplare, circuite făcute să mănânce toată memoria, unghiuri imposibile, cele trei calculatoare verificate unul cu altul și răspunsuri IBM false care mint. Am găsit și reparat 7 greșeli (vezi jurnalul de modificări).

> **ELI5 — pe scurt:** Am încercat din răsputeri să-l stricăm, am găsit 7 greșeli și le-am reparat.

[Jurnal de modificări](CHANGELOG.ro.md)

## Descărcare

Programe pentru Windows, macOS (cip Apple și Intel) și Linux sunt atașate fiecărei versiuni. Alege-o pe a ta mai jos.

**[Ultima versiune](https://github.com/chiuta/CUANTOR/releases/latest)**

- Windows · .zip
- macOS · Apple · .tar.gz
- macOS · Intel · .tar.gz
- Linux · .tar.gz

### Ghid de utilizare (PDF)

Explicat ca pentru un copil de 10 ani, cu un rezumat mic pentru unul de 5 ani după fiecare capitol. Același ghid în 8 limbi.

**[Descarcă ghidul (PDF)](guide/CUANTOR-guide.ro.pdf)** · Toate limbile: [Română](guide/CUANTOR-guide.ro.pdf) · [English](guide/CUANTOR-guide.en.pdf) · [Français](guide/CUANTOR-guide.fr.pdf) · [Deutsch](guide/CUANTOR-guide.de.pdf) · [Español](guide/CUANTOR-guide.es.pdf) · [Italiano](guide/CUANTOR-guide.it.pdf) · [Português](guide/CUANTOR-guide.pt.pdf) · [Magyar](guide/CUANTOR-guide.hu.pdf)

> **ELI5 — pe scurt:** Alegi calculatorul tău, descarci, gata. Ghidul e un PDF.

## Licență

Nu a fost setată încă o licență (alegerea autorului).

---
Construit cu Claude. Doar simulare: nu se pretinde avantaj cuantic.
