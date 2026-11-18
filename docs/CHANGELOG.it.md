# Registro delle modifiche

## 0.2.0

### Aggiunto

- Interfaccia in 8 lingue (GUI con selettore di lingua, guida ed errori della CLI).
- Suite di test avversariali: fuzzing del parser, bombe di risorse, verifiche incrociate tra motori, statistica, risposte REST ostili.
- Integrazione con IBM Quantum: transpilatore verso porte native con instradamento, client REST Runtime, esportazione in Composer.
- Sito di presentazione e documentazione in 8 lingue.

### Corretto

- L'espansione esponenziale di porte definite dall'utente poteva allocare gigabyte e interrompere il processo; l'espansione ora è limitata.
- Le espressioni molto annidate causavano overflow dello stack e l'interruzione del processo; la profondità è limitata a 128.
- I risultati del campionamento dipendevano dal numero di thread; ora dipendono solo dal seme.
- La GUI avviava un numero illimitato di thread di analisi con modifiche rapide (memoria esaurita a 4096 qubit); ora c'è un thread di lavoro e una richiesta in coda.
- Le probabilità di rumore non valide (superiori a 1, negative, NaN) venivano accettate in silenzio; ora vengono rifiutate.
- La CLI ignorava in silenzio opzioni numeriche non valide come --shots abc e motori sconosciuti; ora sono errori.
- Diversi simboli mancavano nel carattere predefinito e comparivano come riquadri vuoti; sono stati sostituiti da etichette di testo.

### Note

- Non verificato su hardware IBM reale.
- I circuiti dinamici non possono essere disegnati nell'editor né inviati all'hardware IBM.
- I titoli degli esempi restano in rumeno.
