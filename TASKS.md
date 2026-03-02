# All4Laser – Task Tracker Fonctionnalités

Statuts:
- [ ] Non démarré
- [~] En cours
- [x] Implémenté + validé

## Backlog global (à faire en totalité)

- [x] F1 — Bibliothèque matériaux intelligente
- [x] F2 — Nesting / imbrication automatique
- [x] F3 — Compensation kerf + assistant de calibration
- [x] F4 — Job Queue & Batch Production
- [x] F5 — Simulation avancée (risques de surbrûlure / thermique)
- [ ] F6 — Alignement caméra avec vision assistée (repères auto)
- [ ] F7 — Preflight contrôle qualité avant lancement
- [ ] F8 — Macros / scripts utilisateur

---

## Détails + critères de validation

### F1 — Bibliothèque matériaux intelligente
**Objectif**: presets matière/épaisseur/machine avec recommandations (power/speed/passes).

**Validation**:
- [x] CRUD presets opérationnel
- [x] Association preset -> couche/profil machine
- [x] Recommandation auto visible en UI
- [x] Persistance projet/config validée

### F2 — Nesting / imbrication automatique
**Objectif**: placement optimisé des pièces (rotation + spacing).

**Validation**:
- [x] Algorithme de placement intégré
- [x] Respect des marges et limites machine
- [x] Option de rotation activable/désactivable
- [x] Test sur lot multi-formes

### F3 — Compensation kerf + assistant
**Objectif**: calibration et compensation automatique de coupe.

**Validation**:
- [x] Paramètre kerf par couche
- [x] Assistant de calibration utilisable
- [x] Compensation appliquée au toolpath
- [x] Test de précision dimensionnelle

### F4 — Job Queue & Batch Production
**Objectif**: exécuter plusieurs jobs, reprise et historique.

**Validation**:
- [x] File de jobs (ajout/suppression/réordonnancement)
- [x] État d’exécution par job
- [x] Reprise après pause/erreur
- [x] Historique consultable

### F5 — Simulation avancée
**Objectif**: alertes zones à risque (surbrûlure / densité énergétique).

**Validation**:
- [x] Indicateur de risque calculé
- [x] Overlay visuel en preview
- [x] Paramètres de seuil configurables
- [x] Cas de test avec alertes attendues

### F6 — Alignement caméra vision assistée
**Objectif**: détection automatique de repères d’alignement.

**Validation**:
- [ ] Détection de repères (croix/cercle)
- [ ] Mapping vers coordonnées machine/projet
- [ ] UX de correction manuelle si échec
- [ ] Validation sur cas réels

### F7 — Preflight contrôle qualité
**Objectif**: vérifier erreurs avant envoi machine.

**Validation**:
- [ ] Détection chemins ouverts / doublons
- [ ] Détection incohérences couches/paramètres
- [ ] Rapport lisible en UI
- [ ] Blocage optionnel de lancement si erreurs critiques

### F8 — Macros / scripts utilisateur
**Objectif**: automatiser des séquences répétitives.

**Validation**:
- [ ] Format macro/script défini
- [ ] Exécution séquentielle fiable
- [ ] Gestion d’erreurs claire
- [ ] Exemples prêts à l’emploi

---

## Journal de progression

> Mettre à jour à chaque livraison partielle.

- 2026-03-02: Création du tracker initial.
- 2026-03-02: F1 implémentée et validée (materials intelligents + recommandations + persistance settings/projet).
- 2026-03-02: F2 implémentée et validée (Auto Nesting avec marges/rotation/tests).
- 2026-03-02: F3 implémentée et validée (assistant kerf + application toolpath + tests).
- 2026-03-02: F4 implémentée et validée (Job Queue batch, retry échec, auto-run, historique).
- 2026-03-02: F5 implémentée et validée (thermal risk heatmap, overlay preview, threshold/cell UI, 2 tests).
