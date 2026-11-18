# Változásnapló

## 0.2.0

### Hozzáadva

- 8 nyelvű felület (grafikus felület nyelvválasztóval, CLI súgó és hibaüzenetek).
- Ellenséges tesztcsomag: elemző-fuzzolás, erőforrás-bombák, motorok közti ellenőrzések, statisztika, ellenséges REST-válaszok.
- IBM Quantum integráció: transzpiler natív kapukra útválasztással, Runtime REST kliens, Composer export.
- Bemutatkozó weboldal és dokumentáció 8 nyelven.

### Javítva

- A felhasználói kapuk exponenciális kibontása gigabájtokat foglalhatott le, és leállíthatta a folyamatot; a kibontás most korlátozott.
- A mélyen egymásba ágyazott kifejezések veremtúlcsordulást okoztak és leállították a folyamatot; a mélység 128-ra korlátozott.
- A mintavétel eredménye függött a szálak számától; mostantól csak a magértéktől függ.
- A grafikus felület gyors szerkesztéskor korlátlan számú elemző szálat indított (4096 qubitnél elfogyott a memória); most egy munkaszál és egy sorban álló kérés van.
- Az érvénytelen zajvalószínűségeket (1 felett, negatív, NaN) csendben elfogadta; most elutasítja.
- A CLI csendben figyelmen kívül hagyta az érvénytelen számopciókat, például a --shots abc értéket, és az ismeretlen motorokat; ezek most hibák.
- Több szimbólum hiányzott az alapértelmezett betűtípusból, és üres négyzetként jelent meg; szöveges címkékre cseréltük őket.

### Megjegyzések

- Valódi IBM hardveren nem ellenőrzött.
- Dinamikus áramkörök nem rajzolhatók meg a szerkesztőben és nem küldhetők IBM hardverre.
- A példák címei románul maradnak.
