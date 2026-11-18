# Jurnal de modificări

## 0.2.1

### Adăugat

- Interfață în 8 limbi (GUI cu selector de limbă, ajutor și erori în CLI).
- Suită de teste adversariale: fuzzing de parser, bombe de resurse, verificări între motoare, statistică, răspunsuri REST ostile.
- Integrare IBM Quantum: transpilator în porți native cu rutare, client REST Runtime, export Composer.
- Site de prezentare și documentație în 8 limbi.

### Reparat

- Expansiunea exponențială a porților definite de utilizator putea aloca gigaocteți și opri procesul; expansiunea este acum limitată.
- Expresiile imbricate adânc provocau depășirea stivei și oprirea procesului; adâncimea e limitată la 128.
- Rezultatele eșantionării depindeau de numărul de fire; acum depind doar de sămânță.
- GUI pornea un număr nelimitat de fire de analiză la editări rapide (memorie epuizată la 4096 de qubiți); acum există un fir de lucru și o cerere în coadă.
- Probabilități de zgomot invalide (peste 1, negative, NaN) erau acceptate tăcut; acum sunt respinse.
- CLI ignora tăcut opțiuni numerice invalide, precum --shots abc, și motoare necunoscute; acum sunt erori.
- Mai multe simboluri lipseau din fontul implicit și apăreau ca pătrate goale; au fost înlocuite cu etichete text.

### Note

- Neverificat pe hardware IBM real.
- Circuitele dinamice nu pot fi desenate în editor sau trimise pe hardware IBM.
- Titlurile exemplelor rămân în română.
