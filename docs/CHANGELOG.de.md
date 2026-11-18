# Änderungsprotokoll

## 0.2.1

### Hinzugefügt

- Oberfläche in 8 Sprachen (GUI mit Sprachauswahl, CLI-Hilfe und Fehlermeldungen).
- Adversariale Testsuite: Parser-Fuzzing, Ressourcenbomben, Engine-Kreuzprüfungen, Statistik, feindliche REST-Antworten.
- IBM-Quantum-Integration: Transpiler in native Gatter mit Routing, Runtime-REST-Client, Composer-Export.
- Präsentationswebsite und Dokumentation in 8 Sprachen.

### Behoben

- Die exponentielle Expansion verschachtelter benutzerdefinierter Gatter konnte Gigabytes belegen und den Prozess abbrechen; die Expansion ist jetzt begrenzt.
- Tief verschachtelte Ausdrücke verursachten einen Stacküberlauf und brachen den Prozess ab; die Tiefe ist auf 128 begrenzt.
- Stichprobenergebnisse hingen von der Threadzahl ab; sie hängen jetzt nur noch vom Seed ab.
- Die GUI startete bei schnellen Änderungen unbegrenzt viele Analyse-Threads (Speicher erschöpft bei 4096 Qubits); jetzt gibt es einen Arbeits-Thread und eine wartende Anfrage.
- Ungültige Rauschwahrscheinlichkeiten (über 1, negativ, NaN) wurden stillschweigend akzeptiert; sie werden jetzt abgelehnt.
- Die CLI ignorierte ungültige numerische Optionen wie --shots abc und unbekannte Engines stillschweigend; das sind jetzt Fehler.
- Mehrere Symbole fehlten in der Standardschrift und erschienen als leere Kästchen; sie wurden durch Textbeschriftungen ersetzt.

### Hinweise

- Auf echter IBM-Hardware nicht verifiziert.
- Dynamische Schaltungen lassen sich weder im Editor zeichnen noch an IBM-Hardware senden.
- Die Titel der Beispiele bleiben rumänisch.
