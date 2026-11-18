# Journal des modifications

## 0.2.0

### Ajouté

- Interface en 8 langues (interface graphique avec sélecteur de langue, aide et erreurs de la CLI).
- Suite de tests adversariaux : fuzzing de l'analyseur, bombes de ressources, recoupements entre moteurs, statistiques, réponses REST hostiles.
- Intégration IBM Quantum : transpileur vers les portes natives avec routage, client REST Runtime, export vers Composer.
- Site de présentation et documentation en 8 langues.

### Corrigé

- L'expansion exponentielle de portes définies par l'utilisateur pouvait allouer des gigaoctets et arrêter le processus ; l'expansion est désormais bornée.
- Des expressions profondément imbriquées provoquaient un débordement de pile et l'arrêt du processus ; la profondeur est limitée à 128.
- Les résultats d'échantillonnage dépendaient du nombre de threads ; ils ne dépendent plus que de la graine.
- L'interface lançait un nombre illimité de threads d'analyse lors de modifications rapides (mémoire épuisée à 4096 qubits) ; il y a maintenant un thread de travail et une requête en file.
- Les probabilités de bruit invalides (supérieures à 1, négatives, NaN) étaient acceptées silencieusement ; elles sont maintenant rejetées.
- La CLI ignorait silencieusement des options numériques invalides comme --shots abc et des moteurs inconnus ; ce sont maintenant des erreurs.
- Plusieurs symboles manquaient dans la police par défaut et s'affichaient comme des cases vides ; ils sont remplacés par des étiquettes textuelles.

### Notes

- Non vérifié sur du matériel IBM réel.
- Les circuits dynamiques ne peuvent être ni dessinés dans l'éditeur ni envoyés au matériel IBM.
- Les titres des exemples restent en roumain.
