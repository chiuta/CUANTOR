# CUANTOR — plan de scalare de la 5 la 1000 de qubiți

## Adevărul matematic (de ce nu există „un singur motor”)

Un vector de stare complet pentru *n* qubiți are 2ⁿ amplitudini complexe (16 octeți fiecare).
Memoria crește exponențial și nu poate fi ocolită prin inginerie:

| Qubiți | Memorie (vector de stare) | Observație |
|---:|---|---|
| 5 | 512 B | instantaneu |
| 20 | 16 MiB | telefon |
| 24 | 256 MiB | limita „Auto” a CUANTOR |
| 28 | 4 GiB | limita maximă a motorului vector de stare |
| 30 | 16 GiB | stație de lucru |
| 40 | 16 TiB | cluster |
| 50 | 16 PiB | peste orice calculator existent |
| 1000 | ≈ 10²⁹⁰ B | imposibil: universul observabil are ≈ 10⁸⁰ atomi |

Concluzie: **1000 de qubiți se simulează exact doar pentru circuite cu structură**.
Un circuit aleator, adânc, pe 1000 de qubiți nu poate fi simulat clasic (acesta e
chiar motivul pentru care există calculatoare cuantice reale). Planul de mai jos
acoperă toată scara cu motorul potrivit fiecărei clase de circuite și spune
sincer unde se oprește fiecare.

## Arhitectura: o interfață, mai multe motoare

Toate motoarele implementează `Machine` (`apply`, `measure`, `reset`), iar
`Backend::Auto` îl alege după circuit. Interfața grafică, CLI-ul, parserul
OpenQASM și zgomotul sunt comune.

| Motor | Qubiți | Ce circuite | Exact? | Stare |
|---|---|---|---|---|
| **Vector de stare** | 1–28 (Auto ≤ 24) | orice, inclusiv zgomot | da | ✅ făcut, multi-thread |
| **Stabilizator** (Aaronson–Gottesman) | 1–4096 | doar Clifford: H, S, CNOT, Pauli, CZ, SWAP, rotații la multipli de π/2 | da | ✅ făcut |
| **Sparse** | 1–4096 | orice poartă, cât timp ramurile nenule rămân puține (GHZ, Bernstein–Vazirani, aritmetică reversibilă) | da | ✅ făcut |
| **MPS** (matrix product state) | 1–1000+ | entanglement mic: lanțuri 1D, QAOA superficial, dinamică locală | aproximativ (eroare de trunchiere raportată) | ⏳ faza 4 |
| **Clifford+T (rang de stabilizator)** | 1–1000 | Clifford + puține porți T (≲ 30–40) | da | ⏳ faza 5 |
| **Propagare Pauli** | 1–1000+ | valori medii de observabile pe circuite superficiale | aproximativ | ⏳ faza 6 |

## Etapele, cu criterii de acceptare

**Faza 0 — 5 qubiți (făcut).** Editor de circuit, vector de stare, OpenQASM 2/3, zgomot.
*Acceptare:* cele 14 exerciții din lecția UPB reproduc probabilitățile exacte.

**Faza 1 — 6–28 qubiți (făcut în mare parte).** Porți paralelizate pe fire, selecție
automată, măsurare/reset/`if` la mijlocul circuitului.
*Acceptare:* execuția paralelă dă amplitudini identice bit cu bit cu cea serială (test).
*De făcut:* fuziunea porților vecine și blocarea în cache (≈ 2–4× la 24+ qubiți), `f32` opțional (până la 29 qubiți pe 4 GiB).

**Faza 2 — 29–1000+ qubiți, circuite Clifford (făcut).** Tablou de stabilizatori
împachetat pe biți, cu zgomot de depolarizare (Pauli este Clifford).
*Acceptare:* GHZ pe 1000 de qubiți dă doar tot-0/tot-1; 30 de circuite Clifford
aleatoare pe 2–5 qubiți dau aceleași distribuții ca vectorul de stare (teste).
*Limită cunoscută:* eșantionarea completă costă ≈ O(n²·n/64) per măsurătoare; la
1000 de qubiți se folosesc ~8–16 eșantioane în previzualizare. Îmbunătățire planificată: eșantionare prin eliminare gaussiană o singură dată, apoi O(n²/64) per tragere.

**Faza 3 — circuite cu puține ramuri (făcut).** Motor sparse cu dicționar
de amplitudini; eroare explicită când numărul de ramuri depășește pragul.
*Acceptare:* GHZ pe 1000 de qubiți cu o poartă T, faza e π/4 pe ramura |1…1⟩ (test).

**Faza 4 — MPS pentru entanglement scăzut.** Tensori A[i] de formă (χ, 2, χ);
porți pe doi qubiți vecini prin SVD cu trunchiere la dimensiunea de legătură χ
(implicit 64, configurabilă); porți la distanță prin lanț SWAP; raportare a
erorii cumulate de trunchiere (suma greutăților eliminate). SVD complex Jacobi
unilateral, fără dependențe externe.
*Acceptare:* pe ≤ 16 qubiți, cu χ nelimitat, MPS = vector de stare (test); lanț
Ising 1D pe 1000 de qubiți rulează în < 10 s cu χ = 32.

**Faza 5 — Clifford+T.** Descompunerea T în sumă de două stabilizatori
(rang de stabilizator 2ᵗ); practic până la ~30–40 de porți T.

**Faza 6 — observabile.** Propagare Pauli în sens invers prin circuit, cu tăiere
a termenilor mici, pentru valori medii ⟨P⟩ pe circuite superficiale.

**Faza 7 — interfața la scară mare.** Fire de circuit virtualizate (se desenează
doar rândurile vizibile), registre grupate (q[0..99] pliabile), histogramă din
„cele mai probabile K stări”, Q-sferă din ramurile dominante, bară de stare cu
motorul activ și memoria folosită.

## Ce rămâne sincer imposibil

- Circuite non-Clifford, adânci și aleatoare pe > 50 de qubiți: nu există motor clasic exact.
- „1000 de qubiți” în interfață înseamnă 1000 de fire de circuit, simulate cu motorul
  potrivit structurii; aplicația afișează explicit motorul folosit și motivul
  alegerii, ca să nu existe impresia că vectorul de stare complet a fost calculat.

## Conexiunea cu hardware-ul real (IBM Quantum Platform)

Fluxul planificat, bazat pe API-ul REST Qiskit Runtime:

1. Token Bearer din cheia API (IAM IBM Cloud); antete `Authorization` și `Service-CRN`.
2. Circuitul trebuie să fie **ISA** (transpilat la porțile native și la harta de
   cuplaj a dispozitivului): CUANTOR include un transpilator mic (rz, sx, x +
   CZ/ECR/CX, rutare cu SWAP) verificat numeric cu propriul simulator.
3. `POST /api/v1/jobs` cu `program_id: "sampler"`, `pubs: [[qasm3, null, shots]]`.
4. Sondare `GET /api/v1/jobs/{id}`, rezultate la `…/results`, cu eșantioanele în
   hexazecimal pe fiecare registru clasic.
5. Rezultatele hardware se afișează lângă simulare (distanță de variație totală).
