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
- [x] F6 — Alignement caméra avec vision assistée (repères auto)
- [x] F7 — Preflight contrôle qualité avant lancement
- [x] F8 — Macros / scripts utilisateur

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
- [x] Détection de repères (croix/cercle)
- [x] Mapping vers coordonnées machine/projet
- [x] UX de correction manuelle si échec
- [x] Validation sur cas réels

### F7 — Preflight contrôle qualité
**Objectif**: vérifier erreurs avant envoi machine.

**Validation**:
- [x] Détection chemins ouverts / doublons
- [x] Détection incohérences couches/paramètres
- [x] Rapport lisible en UI
- [x] Blocage optionnel de lancement si erreurs critiques

### F8 — Macros / scripts utilisateur
**Objectif**: automatiser des séquences répétitives.

**Validation**:
- [x] Format macro/script défini
- [x] Exécution séquentielle fiable
- [x] Gestion d’erreurs claire
- [x] Exemples prêts à l’emploi

---

## Journal de progression

> Mettre à jour à chaque livraison partielle.

- 2026-03-02: Création du tracker initial.
- 2026-03-02: F1 implémentée et validée (materials intelligents + recommandations + persistance settings/projet).
- 2026-03-02: F2 implémentée et validée (Auto Nesting avec marges/rotation/tests).
- 2026-03-02: F3 implémentée et validée (assistant kerf + application toolpath + tests).
- 2026-03-02: F4 implémentée et validée (Job Queue batch, retry échec, auto-run, historique).
- 2026-03-02: F5 implémentée et validée (thermal risk heatmap, overlay preview, threshold/cell UI, 2 tests).
- 2026-03-02: F6 implémentée et validée (Alignement caméra vision auto-detect repères).
- 2026-03-02: F7 implémentée et validée (Preflight contrôle qualité avant lancement).
- 2026-03-02: F8 implémentée et validée (Macros / scripts utilisateur fiables via job execution).
