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
- [ ] F9 — Import/Export LightBurn (.lbrn2)
- [ ] F10 — Connexion réseau (WiFi / TCP / WebSocket)
- [x] F11 — Profils machine multiples
- [x] F12 — Power Ramping (puissance variable le long du chemin)
- [ ] F13 — Gravure 3D / Relief (grayscale depth map)
- [x] F14 — Notifications & son de fin de job
- [x] F15 — Rapport de job exportable (PDF/CSV)
- [ ] F16 — Undo/Redo global (toutes opérations)
- [x] F17 — Estimation de coût du job
- [ ] F18 — Print & Cut (repères d'alignement imprimés)
- [ ] F19 — Auto-focus axe Z (probe)
- [ ] F20 — Raccourcis clavier LightBurn-compatible
- [ ] F21 — Variable Text / Sérialisation (CSV data merge)
- [x] F22 — Tabs / Bridges de maintien
- [x] F23 — Lead-in / Lead-out (amorce de coupe)
- [x] F24 — Multi-pass avec offset progressif
- [ ] F25 — Bibliothèque de formes paramétriques (Box Maker, engrenages…)
- [ ] F26 — Simulation temporelle animée (dry run visuel)
- [x] F27 — Suivi de maintenance machine
- [ ] F28 — Watch Folder / Dossier surveillé (auto-queue)
- [ ] F29 — Live progress overlay caméra
- [ ] F30 — Communauté presets matériaux en ligne
- [ ] F31 — Mode kiosk / opérateur
- [x] F32 — Contrôle air assist automatique par couche
- [x] F33 — Mode perforation / pointillés
- [ ] F34 — Dithering avancé (Floyd-Steinberg, Jarvis, Stucki, ordered)
- [ ] F35 — Scanner matériau via caméra (détection bords + placement auto)
- [x] F36 — Reprise de job après coupure (power failure recovery)
- [ ] F37 — Dashboard de monitoring distant (web)
- [ ] F38 — Timelapse caméra du job
- [x] F39 — Outils d'alignement objets (centrer, distribuer, snapping)
- [x] F40 — Réduction de puissance dans les coins (corner power control)
- [x] F41 — Simplification de chemin (réduction de nœuds)
- [ ] F42 — Post-processeurs personnalisables
- [x] F43 — Assistant premier lancement (startup wizard)
- [x] F44 — Gravure bi-directionnelle optimisée
- [ ] F45 — Import G-code depuis URL / cloud
- [ ] F46 — Mode multi-têtes / dual laser
- [x] F47 — Détection automatique du firmware
- [ ] F48 — Interpolation spline / courbes de Bézier (G5)
- [ ] F49 — Crosshatch / hachures pour remplissage
- [x] F50 — Outil de mesure sur canvas
- [x] F51 — Groupement d'objets
- [x] F52 — Historique de jobs avec statistiques (dashboard)
- [ ] F53 — Mode tactile (tablette)
- [x] F54 — Export SVG / DXF depuis le projet
- [x] F55 — Palette de couleurs personnalisable par couche
- [ ] F56 — Mode accessibilité (daltonien, contraste élevé)
- [ ] F57 — API REST / ligne de commande (headless mode)
- [ ] F58 — Planification horaire des jobs
- [ ] F59 — Détection de collision / zones interdites
- [ ] F60 — Répétition intelligente (array circulaire, le long d'un chemin)
- [ ] F61 — Import PDF (vectoriel)
- [ ] F62 — Import AI (Adobe Illustrator)
- [ ] F63 — Import HPGL (.plt)
- [x] F64 — Export GCode commenté / annoté
- [x] F65 — Tooltips contextuels & aide intégrée
- [ ] F66 — Thèmes utilisateur importables (JSON)
- [x] F67 — Journal d'événements machine (event log)
- [ ] F68 — Système de favoris / épingles
- [ ] F69 — Texte sur chemin (text on path)
- [ ] F70 — Mode chambre noire (red-only UI pour lunettes laser)
- [x] F71 — Auto-save & récupération après crash
- [x] F72 — Drag & drop natif depuis l'explorateur de fichiers
- [ ] F73 — Codes-barres avancés (EAN, Code128, DataMatrix)
- [ ] F74 — Remplissage spiralé / radial
- [ ] F75 — Mode tampon / stamp (négatif pour caoutchouc)
- [ ] F76 — Templates de gabarits / fixations (jigs)
- [x] F77 — Contrôle ventilation / extraction fumées
- [ ] F78 — Détection feu / fumée via caméra
- [ ] F79 — Photo engraving wizard (assistant gravure photo)
- [ ] F80 — Vue split (design + GCode côte à côte)
- [ ] F81 — Centerline trace (vectorisation au trait central)
- [ ] F82 — Gravure cylindrique déroulée (unwrap rotary)
- [ ] F83 — Mode découpe multi-matériaux (sandwich / Z-layers)
- [x] F84 — Détection automatique d'images dupliquées / superposées
- [ ] F85 — Générateur de motifs (patterns fill)
- [ ] F86 — Weld / Trim / Extend vecteurs
- [ ] F87 — Gradient fill vectoriel
- [ ] F88 — Contour multi-offset (relief en escalier)
- [ ] F89 — Raccourcis personnalisables par l'utilisateur
- [x] F90 — Panneau de notes / annotations projet
- [ ] F91 — Comparateur de paramètres avant/après
- [ ] F92 — Mode multi-fenêtres / panneaux détachables
- [x] F93 — Générateur de test de focus Z
- [x] F94 — Contrôle interlock (capot, flux d'eau, température)
- [ ] F95 — Langues supplémentaires (chinois, russe, turc, coréen, polonais)
- [x] F96 — Conversion automatique unités (mm ↔ pouces)
- [x] F97 — Estimation usure tube laser par job
- [ ] F98 — Mode présentation / démo (watermark + limitations)
- [ ] F99 — Système de plugins Lua/WASM
- [ ] F100 — Sauvegarde cloud & synchronisation multi-postes
- [ ] F101 — Stippling / gravure par points
- [ ] F102 — Séparation couleurs (image → couches par couleur)
- [x] F103 — Géométrie de construction (lignes de référence non gravées)
- [ ] F104 — Contrôle par gamepad / joystick (jog physique)
- [ ] F105 — Intégration MQTT / IoT
- [ ] F106 — Templates de jobs réutilisables
- [ ] F107 — Halftone avancé (circulaire, linéaire, personnalisé)
- [ ] F108 — Snap aux points clés (centres, milieux, intersections)
- [ ] F109 — Traitement batch multi-fichiers
- [ ] F110 — Versioning de projet (historique des révisions)
- [ ] F111 — Marquage industriel UDI/DMC (conformité traçabilité)
- [ ] F112 — Calibration axes (steps/mm, équerrage, backlash)
- [ ] F113 — Edge detection pour vectorisation (Canny, Sobel)
- [ ] F114 — Workflow bijouterie (gravure intérieure bague, pendentif)
- [ ] F115 — Workflow signalétique (plaques, panneaux multicouches)
- [ ] F116 — Mode textile / cuir (gestion du déplacement matériau souple)
- [ ] F117 — QR code scanner via caméra pour chargement de job
- [ ] F118 — Voix synthétisée pour statut machine (TTS)
- [ ] F119 — Pendant / boîtier de commande physique USB
- [ ] F120 — Dot peen marking simulation
- [ ] F121 — Intégration ERP / gestion de commandes
- [ ] F122 — Générateur de lithophanie
- [ ] F123 — Multi-utilisateurs temps réel (collaboration)
- [ ] F124 — Système de scoring qualité du design

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

### F9 — Import/Export LightBurn (.lbrn2)
**Objectif**: compatibilité avec le format LightBurn pour faciliter la migration des utilisateurs.

**Validation**:
- [ ] Import .lbrn2 (XML) : couches, couleurs, vecteurs, paramètres power/speed
- [ ] Mapping couches LightBurn → couches All4Laser
- [ ] Export projet vers .lbrn2
- [ ] Test aller-retour (roundtrip) sans perte de données critiques

### F10 — Connexion réseau (WiFi / TCP / WebSocket)
**Objectif**: supporter les contrôleurs réseau (FluidNC, ESP32-GRBL, LaserGRBL WiFi).

**Validation**:
- [ ] Backend TCP client configurable (IP:port)
- [ ] Backend WebSocket client
- [ ] Détection automatique des machines sur le réseau local (mDNS/broadcast)
- [ ] Même UX que la connexion série (transparent pour l'utilisateur)

### F11 — Profils machine multiples
**Objectif**: sauvegarder et basculer entre plusieurs machines.

**Validation**:
- [x] Liste de profils (CRUD) persistée
- [x] Sélecteur de profil actif en UI
- [x] Import/export de profils (JSON)
- [ ] Association profil → presets matériaux

### F12 — Power Ramping (puissance variable le long du chemin)
**Objectif**: moduler la puissance graduellement (coins, débuts/fins de coupe).

**Validation**:
- [ ] Paramètre ramping par couche (longueur de rampe, % début, % fin)
- [ ] Génération GCode avec S progressif
- [ ] Visualisation du ramping en preview (dégradé de couleur)
- [ ] Test sur coupe bois : absence de brûlure aux extrémités

### F13 — Gravure 3D / Relief (grayscale depth map)
**Objectif**: convertir une image niveaux de gris en gravure relief (power variable).

**Validation**:
- [ ] Import image grayscale → mapping puissance
- [ ] Paramètres : résolution, passes, profondeur max
- [ ] Preview 3D simplifiée du relief attendu
- [ ] Test sur bois avec résultat mesurable

### F14 — Notifications & son de fin de job
**Objectif**: alerter l'utilisateur en fin de job ou sur erreur.

**Validation**:
- [x] Son configurable (fin de job, erreur, alarme)
- [x] Notification bureau (Windows toast / Linux notify)
- [x] Option mute / volume
- [ ] Test sur job complet + test sur erreur

### F15 — Rapport de job exportable (PDF/CSV)
**Objectif**: générer un rapport détaillé après exécution.

**Validation**:
- [ ] Contenu : date, durée, matériau, paramètres, distance, nb passes
- [ ] Export CSV
- [ ] Export PDF (ou HTML imprimable)
- [ ] Historique des rapports consultable

### F16 — Undo/Redo global (toutes opérations)
**Objectif**: historique d'annulation couvrant toutes les opérations, pas seulement les shapes.

**Validation**:
- [ ] Stack undo/redo pour : déplacements, paramètres couches, ajout/suppression d'éléments
- [ ] Limite configurable (nb d'étapes)
- [ ] Raccourcis Ctrl+Z / Ctrl+Y fonctionnels partout
- [ ] Pas de corruption d'état après undo/redo multiples

### F17 — Estimation de coût du job
**Objectif**: calculer un coût basé sur temps machine, matériau et énergie.

**Validation**:
- [ ] Configuration tarifs : €/h machine, €/m² matériau, €/kWh
- [ ] Calcul automatique affiché en status bar
- [ ] Prise en compte du nombre de passes
- [ ] Export du coût dans le rapport de job (F15)

### F18 — Print & Cut (repères d'alignement imprimés)
**Objectif**: aligner une découpe laser sur un graphique pré-imprimé.

**Validation**:
- [ ] Détection de registration marks via caméra (extension F6)
- [ ] Calcul de transformation affine (translation + rotation + scale)
- [ ] Application auto au toolpath
- [ ] Test sur cas réel imprimé

### F19 — Auto-focus axe Z (probe)
**Objectif**: focus automatique via palpage Z-probe.

**Validation**:
- [ ] Séquence de probe configurable (vitesse, retrait)
- [ ] Commande GRBL $J ou G38.2 pour palpage
- [ ] Offset Z automatique après probe
- [ ] Sécurité : limites de course Z respectées

### F20 — Raccourcis clavier LightBurn-compatible
**Objectif**: faciliter la transition des utilisateurs LightBurn.

**Validation**:
- [ ] Mapping des raccourcis LightBurn les plus courants
- [ ] Mode sélectionnable (All4Laser natif / LightBurn)
- [ ] Persistance du choix dans settings
- [ ] Documentation des différences

### F21 — Variable Text / Sérialisation (CSV data merge)
**Objectif**: graver en batch avec données variables (numéros de série, noms, QR codes uniques).

**Validation**:
- [ ] Import CSV avec colonnes mappables (texte, QR, code-barres)
- [ ] Placeholder dans le design (ex: {col1}, {serial})
- [ ] Preview par entrée CSV avec navigation
- [ ] Génération batch intégrée à la Job Queue (F4)

### F22 — Tabs / Bridges de maintien
**Objectif**: ajouter des ponts de matière pour empêcher les pièces de tomber.

**Validation**:
- [ ] Insertion automatique de tabs sur chemins fermés
- [ ] Paramètres : nombre, largeur, espacement min
- [ ] Placement manuel drag & drop
- [ ] Visualisation en preview (couleur distincte)

### F23 — Lead-in / Lead-out (amorce de coupe)
**Objectif**: éviter les marques de brûlure au point de départ de coupe.

**Validation**:
- [ ] Paramètre par couche : type (ligne/arc), longueur, angle
- [ ] Génération GCode avec amorce/sortie
- [ ] Visualisation en preview
- [ ] Test : pas de sur-brûlure au point d'entrée

### F24 — Multi-pass avec offset progressif
**Objectif**: décaler chaque passe pour un kerf plus propre sur matériaux épais.

**Validation**:
- [ ] Paramètre offset par passe (mm)
- [ ] Direction d'offset configurable (intérieur/extérieur)
- [ ] Compatibilité avec la compensation kerf (F3)
- [ ] Test sur acrylique épais

### F25 — Bibliothèque de formes paramétriques
**Objectif**: générateur intégré de formes paramétriques (boîtes, engrenages, charnières).

**Validation**:
- [ ] Box Maker : dimensions, épaisseur, type d'assemblage (doigts, T-slot)
- [ ] Engrenages : module, nb dents, angle de pression
- [ ] Living Hinge : patterns configurables
- [ ] Insertion directe dans le projet avec couches assignées

### F26 — Simulation temporelle animée (dry run visuel)
**Objectif**: simuler le parcours laser en temps réel ou accéléré.

**Validation**:
- [ ] Animation du parcours avec curseur temporel
- [ ] Vitesse réglable (1x, 2x, 5x, 10x)
- [ ] Distinction visuelle G0 (travel) vs G1 (burn)
- [ ] Pause/reprise de la simulation

### F27 — Suivi de maintenance machine
**Objectif**: tracker l'usure et planifier la maintenance.

**Validation**:
- [ ] Compteur heures tube laser (persisté)
- [ ] Rappels configurables (nettoyage lentille, miroirs, courroie)
- [ ] Alerte visuelle quand seuil atteint
- [ ] Reset compteur après maintenance

### F28 — Watch Folder / Dossier surveillé
**Objectif**: automatiser l'ajout de jobs depuis un dossier.

**Validation**:
- [ ] Configuration du dossier à surveiller
- [ ] Détection auto des fichiers (GCode, SVG, DXF)
- [ ] Ajout automatique à la Job Queue (F4)
- [ ] Option : lancer automatiquement ou attendre validation

### F29 — Live progress overlay caméra
**Objectif**: afficher la progression du job sur le flux caméra.

**Validation**:
- [ ] Superposition du chemin parcouru (vert) et restant (gris)
- [ ] Position courante du laser (point rouge)
- [ ] Synchronisation avec le statut GRBL en temps réel
- [ ] Compatible avec la calibration caméra existante (F6)

### F30 — Communauté presets matériaux en ligne
**Objectif**: partager et télécharger des presets validés par la communauté.

**Validation**:
- [ ] Format JSON standardisé pour l'échange
- [ ] Téléchargement depuis un repo GitHub / API
- [ ] Notation / validation communautaire
- [ ] Import en un clic dans la bibliothèque locale (F1)

### F31 — Mode kiosk / opérateur
**Objectif**: interface simplifiée verrouillée pour opérateur atelier.

**Validation**:
- [ ] Vue réduite : charger fichier, preview, lancer
- [ ] Pas d'accès aux paramètres avancés
- [ ] Protection par mot de passe pour sortir du mode
- [ ] Boutons gros et lisibles (tactile-friendly)

### F32 — Contrôle air assist automatique par couche
**Objectif**: activer/désactiver l'air assist via M-codes selon le matériau.

**Validation**:
- [x] Paramètre air assist par couche (on/off, M7/M8/M9)
- [ ] Association automatique via preset matériau (F1)
- [x] Insertion M-codes dans le GCode généré
- [ ] Test : air assist activé sur coupe, désactivé sur gravure

### F33 — Mode perforation / pointillés
**Objectif**: découper en pointillés pour créer des lignes pré-découpées (tear-off, pliages).

**Validation**:
- [x] Paramètre par couche : longueur coupe, longueur gap
- [x] Conversion automatique des chemins continus en segments perforés
- [ ] Visualisation en preview (tirets)
- [ ] Test : pliage/déchirure propre sur carton

### F34 — Dithering avancé (Floyd-Steinberg, Jarvis, Stucki, ordered)
**Objectif**: offrir plusieurs algorithmes de tramage pour la gravure photo.

**Validation**:
- [ ] Au moins 4 algorithmes sélectionnables (Floyd-Steinberg, Jarvis, Stucki, ordered/Bayer)
- [ ] Preview en temps réel du résultat
- [ ] Paramètres : seuil, résolution DPI, contraste
- [ ] Comparatif visuel côte à côte

### F35 — Scanner matériau via caméra (détection bords + placement auto)
**Objectif**: détecter automatiquement les bords du matériau et placer le design.

**Validation**:
- [ ] Détection de contour du matériau via caméra (extension F6)
- [ ] Calcul de la zone utilisable
- [ ] Placement automatique du design dans la zone détectée
- [ ] Gestion des chutes (réutilisation de matériau partiellement coupé)

### F36 — Reprise de job après coupure (power failure recovery)
**Objectif**: reprendre un job interrompu par une coupure de courant.

**Validation**:
- [x] Sauvegarde périodique de la ligne GCode en cours (checkpoint)
- [x] Au redémarrage : détection du dernier checkpoint
- [x] UI de reprise : choix de la ligne de reprise avec preview
- [ ] Test : interruption simulée + reprise sans doublon ni saut

### F37 — Dashboard de monitoring distant (web)
**Objectif**: surveiller la machine à distance via navigateur web.

**Validation**:
- [ ] Serveur HTTP/WebSocket intégré (optionnel, activable)
- [ ] Page web : statut machine, progression job, preview
- [ ] Contrôles basiques : pause, reprise, stop
- [ ] Flux caméra en temps réel (MJPEG)

### F38 — Timelapse caméra du job
**Objectif**: capturer automatiquement un timelapse pendant la gravure.

**Validation**:
- [ ] Capture d'image à intervalle configurable (secondes / lignes GCode)
- [ ] Assemblage en GIF ou vidéo (ou export d'images numérotées)
- [ ] Déclenchement auto au début du job, arrêt à la fin
- [ ] Sauvegarde dans le dossier du projet

### F39 — Outils d'alignement objets (centrer, distribuer, snapping)
**Objectif**: aligner et distribuer les objets dans le design.

**Validation**:
- [x] Alignement : gauche, droite, haut, bas, centre H/V
- [x] Distribution : espacement égal horizontal/vertical
- [ ] Snap to grid configurable
- [ ] Guides magnétiques (smart guides)

### F40 — Réduction de puissance dans les coins (corner power control)
**Objectif**: réduire automatiquement la puissance dans les virages serrés.

**Validation**:
- [x] Détection d'angle entre segments consécutifs
- [x] Réduction de S proportionnelle à l'angle (configurable)
- [x] Seuil d'angle minimum pour activation
- [ ] Test : pas de brûlure dans les coins sur bois

### F41 — Simplification de chemin (réduction de nœuds)
**Objectif**: réduire le nombre de points d'un chemin pour des coupes plus fluides.

**Validation**:
- [x] Algorithme Ramer-Douglas-Peucker avec tolérance configurable
- [x] Preview avant/après avec compteur de nœuds
- [x] Application sélective (par objet ou couche)
- [x] Pas de dégradation visible à tolérance raisonnable

### F42 — Post-processeurs personnalisables
**Objectif**: adapter la sortie GCode à différents firmwares/machines.

**Validation**:
- [ ] Format de post-processeur (JSON ou script) : header, footer, M-codes custom
- [ ] Presets intégrés : GRBL, Marlin, Smoothie, FluidNC
- [ ] Éditeur de post-processeur en UI
- [ ] Test : même design → GCode correct pour chaque firmware

### F43 — Assistant premier lancement (startup wizard)
**Objectif**: guider les nouveaux utilisateurs lors du premier démarrage.

**Validation**:
- [ ] Étapes : langue, type de machine, dimensions, port série, profil matériau
- [ ] Détection automatique du port série
- [ ] Création du profil machine à la fin
- [ ] Option "ne plus afficher"

### F44 — Gravure bi-directionnelle optimisée
**Objectif**: optimiser le raster en gravure bi-directionnelle avec compensation de backlash.

**Validation**:
- [ ] Mode uni/bi-directionnel sélectionnable par couche
- [ ] Paramètre de compensation backlash (µs ou mm)
- [ ] Overscan configurable (dépassement avant/après ligne)
- [ ] Test : alignement des lignes aller/retour sur gravure fine

### F45 — Import G-code depuis URL / cloud
**Objectif**: ouvrir des fichiers directement depuis une URL (GitHub, Drive, Dropbox).

**Validation**:
- [ ] Champ URL dans le dialogue Open
- [ ] Téléchargement HTTP(S) avec barre de progression
- [ ] Cache local du fichier téléchargé
- [ ] Support des liens bruts GitHub / Google Drive

### F46 — Mode multi-têtes / dual laser
**Objectif**: supporter les machines à double tête laser.

**Validation**:
- [ ] Configuration nb de têtes dans le profil machine
- [ ] Assignation couche → tête (T0, T1)
- [ ] Génération GCode avec changement d'outil (T0/T1)
- [ ] Preview différenciée par tête

### F47 — Détection automatique du firmware
**Objectif**: identifier le firmware de la machine automatiquement à la connexion.

**Validation**:
- [x] Envoi de commandes d'identification ($I, M115, version)
- [x] Parsing des réponses pour identifier GRBL/Marlin/Smoothie/FluidNC
- [x] Configuration automatique du backend controller
- [x] Fallback manuel si détection échoue

### F48 — Interpolation spline / courbes de Bézier (G5)
**Objectif**: utiliser G5 (spline cubique) au lieu de segmenter en G1.

**Validation**:
- [ ] Détection du support G5 par le firmware
- [ ] Conversion courbes SVG/DXF → G5 quand possible
- [ ] Fallback G1 si non supporté
- [ ] Test : réduction taille fichier + lissage visible

### F49 — Crosshatch / hachures pour remplissage
**Objectif**: remplissage de surfaces par hachures croisées.

**Validation**:
- [ ] Paramètres : angle, espacement, nb de directions (1, 2, 3)
- [ ] Génération GCode hachures dans le contour
- [ ] Preview des hachures en overlay
- [ ] Optimisation du parcours (minimiser les G0)

### F50 — Outil de mesure sur canvas
**Objectif**: mesurer distances, angles et surfaces dans la preview.

**Validation**:
- [ ] Mode mesure activable (bouton ou raccourci)
- [ ] Affichage distance point-à-point en mm
- [ ] Affichage angle entre deux segments
- [ ] Mesure de surface (aire d'un contour fermé)

### F51 — Groupement d'objets
**Objectif**: grouper/dégrouper des shapes pour manipulation unifiée.

**Validation**:
- [ ] Ctrl+G pour grouper, Ctrl+Shift+G pour dégrouper
- [ ] Déplacement/rotation/scale du groupe entier
- [ ] Assignation de couche au groupe
- [ ] Persistance dans le fichier projet (.a4l)

### F52 — Historique de jobs avec statistiques (dashboard)
**Objectif**: visualiser l'historique et les stats d'utilisation.

**Validation**:
- [ ] Persistance : date, durée, matériau, réussite/échec
- [ ] Dashboard : nb jobs semaine/mois, heures cumulées
- [ ] Graphiques basiques (barres/lignes)
- [ ] Export CSV de l'historique

### F53 — Mode tactile (tablette)
**Objectif**: support des gestes tactiles pour utilisation tablette.

**Validation**:
- [ ] Pinch-to-zoom sur la preview
- [ ] Pan à deux doigts
- [ ] Boutons agrandis (min 48px touch target)
- [ ] Test sur tablette Windows

### F54 — Export SVG / DXF depuis le projet
**Objectif**: exporter le design courant vers SVG ou DXF.

**Validation**:
- [ ] Export SVG avec couches → groupes colorés
- [ ] Export DXF avec couches → layers DXF
- [ ] Conservation des dimensions exactes (mm)
- [ ] Test aller-retour : export → réimport sans perte

### F55 — Palette de couleurs personnalisable par couche
**Objectif**: choisir librement les couleurs des couches.

**Validation**:
- [x] Color picker par couche dans le panneau layers
- [x] Palette de couleurs prédéfinies (LightBurn-like)
- [ ] Persistance dans le projet et les settings
- [ ] Pas de conflit visuel avec le thème actif

### F56 — Mode accessibilité (daltonien, contraste élevé)
**Objectif**: adapter l'interface pour les utilisateurs daltoniens.

**Validation**:
- [ ] 3 palettes daltonien : protanopie, deutéranopie, tritanopie
- [ ] Patterns/textures en complément des couleurs sur la preview
- [ ] Épaisseurs de trait augmentées en mode accessibilité
- [ ] Sélection dans les settings, persistance

### F57 — API REST / ligne de commande (headless mode)
**Objectif**: utiliser All4Laser sans GUI pour intégration pipelines/automatisation.

**Validation**:
- [ ] Mode CLI : `--input`, `--output`, `--profile`, `--material`
- [ ] Conversion batch SVG/DXF → GCode sans écran
- [ ] Serveur HTTP optionnel pour contrôle à distance (REST API)
- [ ] Documentation des endpoints / arguments CLI

### F58 — Planification horaire des jobs
**Objectif**: programmer l'exécution d'un job à une heure précise.

**Validation**:
- [ ] Sélecteur date/heure dans la Job Queue (F4)
- [ ] Timer avec countdown visible
- [ ] Notification avant lancement (F14)
- [ ] Annulation / report possible

### F59 — Détection de collision / zones interdites
**Objectif**: définir des zones où le laser ne doit pas aller.

**Validation**:
- [ ] Définition de zones rectangulaires/polygonales interdites
- [ ] Alerte visuelle si le toolpath traverse une zone
- [ ] Option blocage (comme F7 preflight)
- [ ] Persistance des zones dans le profil machine

### F60 — Répétition intelligente (array avancé)
**Objectif**: répéter un design avec transformations avancées.

**Validation**:
- [ ] Array circulaire (rayon, nb copies, angle)
- [ ] Array le long d'un chemin (path follow)
- [ ] Miroir alterné (flip chaque N copies)
- [ ] Preview interactive des copies

### F61 — Import PDF (vectoriel)
**Objectif**: importer les chemins vectoriels d'un fichier PDF.

**Validation**:
- [ ] Extraction des paths vectoriels du PDF
- [ ] Mapping vers couches All4Laser
- [ ] Gestion multi-pages (sélection de page)
- [ ] Ignore le texte rasterisé / images embed

### F62 — Import AI (Adobe Illustrator)
**Objectif**: importer les fichiers .ai natifs.

**Validation**:
- [ ] Parsing du format AI (PostScript/PDF-based)
- [ ] Extraction vecteurs + couleurs
- [ ] Mapping vers couches
- [ ] Fallback : suggestion de sauver en SVG si AI trop complexe

### F63 — Import HPGL (.plt)
**Objectif**: supporter le format HPGL pour plotters et machines héritées.

**Validation**:
- [ ] Parser HPGL (PU, PD, PA, CI, SP)
- [ ] Conversion vers chemins All4Laser
- [ ] Mapping pen number → couche
- [ ] Test sur fichier .plt réel

### F64 — Export GCode commenté / annoté
**Objectif**: générer du GCode lisible avec commentaires.

**Validation**:
- [x] Commentaires par section : couche, passe, matériau, paramètres
- [x] Option activable/désactivable (pour firmware qui n'aiment pas les commentaires)
- [x] Format standard (parenthèses ou point-virgule)
- [x] Header avec métadonnées projet

### F65 — Tooltips contextuels & aide intégrée
**Objectif**: aider les débutants avec des explications en contexte.

**Validation**:
- [ ] Tooltip sur chaque paramètre : description, plage, impact
- [ ] Lien "En savoir plus" vers documentation
- [ ] Mode débutant enrichi avec guides pas-à-pas
- [ ] Traduction des tooltips (i18n)

### F66 — Thèmes utilisateur importables (JSON)
**Objectif**: créer et partager des thèmes couleur personnalisés.

**Validation**:
- [ ] Format JSON définissant toutes les couleurs UI
- [ ] Import/export de thèmes
- [ ] Éditeur de thème en UI avec preview live
- [ ] Quelques thèmes communautaires inclus par défaut

### F67 — Journal d'événements machine (event log)
**Objectif**: logger tous les événements pour diagnostic.

**Validation**:
- [ ] Log : connexion, déconnexion, alarmes, erreurs, changements params
- [ ] Horodatage précis
- [ ] Filtrage par type/sévérité
- [ ] Export en fichier texte / CSV

### F68 — Système de favoris / épingles
**Objectif**: accès rapide aux éléments les plus utilisés.

**Validation**:
- [ ] Épingler : fichiers récents, presets matériaux, macros
- [ ] Barre de favoris accessible depuis la toolbar
- [ ] Persistance dans les settings
- [ ] Drag & drop pour réordonner

### F69 — Texte sur chemin (text on path)
**Objectif**: placer du texte le long d'une courbe, cercle ou chemin arbitraire.

**Validation**:
- [ ] Sélection d'un chemin de référence (cercle, arc, bézier, polyligne)
- [ ] Saisie texte avec police/taille configurable
- [ ] Espacement inter-caractères ajustable
- [ ] Retournement (texte au-dessus / en-dessous du chemin)

### F70 — Mode chambre noire (red-only UI pour lunettes laser)
**Objectif**: interface entièrement rouge/noir compatible avec les lunettes de protection laser.

**Validation**:
- [ ] Thème spécial : uniquement rouge, noir, blanc
- [ ] Preview en niveaux de rouge
- [ ] Activation rapide (raccourci ou bouton)
- [ ] Lisibilité validée avec lunettes OD5+ (532nm, 1064nm)

### F71 — Auto-save & récupération après crash
**Objectif**: sauvegarder automatiquement le projet pour éviter la perte de travail.

**Validation**:
- [x] Auto-save périodique configurable (intervalle en secondes)
- [x] Fichier de récupération séparé (.a4l.recovery)
- [x] Au démarrage : détection du recovery file et proposition de restauration
- [x] Pas d'écrasement du fichier original (sauvegarde à côté)

### F72 — Drag & drop natif depuis l'explorateur de fichiers
**Objectif**: ouvrir un fichier en le glissant-déposant dans la fenêtre.

**Validation**:
- [x] Support drag & drop OS natif (Windows + Linux)
- [x] Types acceptés : .gcode, .nc, .svg, .dxf, .png, .jpg, .bmp, .a4l
- [ ] Feedback visuel pendant le drag (zone de drop)
- [x] Ouverture automatique après drop

### F73 — Codes-barres avancés (EAN, Code128, DataMatrix)
**Objectif**: générer des codes-barres variés en plus du QR code existant.

**Validation**:
- [ ] Formats : EAN-13, Code 128, DataMatrix, Code 39
- [ ] Paramètres : taille, hauteur barres, quiet zone
- [ ] Preview en temps réel
- [ ] Intégration avec la sérialisation CSV (F21)

### F74 — Remplissage spiralé / radial
**Objectif**: remplir les formes fermées en spirale au lieu de lignes parallèles.

**Validation**:
- [ ] Mode spirale : du contour vers le centre
- [ ] Mode radial : lignes depuis le centre vers les bords
- [ ] Espacement configurable
- [ ] Optimisé pour formes circulaires (moins de G0)

### F75 — Mode tampon / stamp (négatif pour caoutchouc)
**Objectif**: inverser automatiquement un design pour la création de tampons en caoutchouc.

**Validation**:
- [ ] Miroir horizontal automatique du design
- [ ] Inversion noir/blanc pour gravure négative
- [ ] Ajout automatique de la bordure du tampon
- [ ] Profondeurs de gravure adaptées (presets caoutchouc)

### F76 — Templates de gabarits / fixations (jigs)
**Objectif**: sauvegarder et réutiliser des positions de fixation sur le workspace.

**Validation**:
- [ ] Définir des zones de fixation nommées (position, dimensions)
- [ ] Bibliothèque de jigs sauvegardée
- [ ] Snap du design sur le jig
- [ ] Overlay visuel des jigs sur la preview/caméra

### F77 — Contrôle ventilation / extraction fumées
**Objectif**: contrôler le ventilateur d'extraction via M-codes.

**Validation**:
- [ ] Activation auto au début du job, arrêt différé après la fin
- [ ] M-code configurable (M7, M8 ou custom)
- [ ] Délai post-job configurable (secondes de purge)
- [ ] Indicateur de statut en UI

### F78 — Détection feu / fumée via caméra
**Objectif**: détecter un début d'incendie et arrêter automatiquement la machine.

**Validation**:
- [ ] Analyse d'image temps réel : détection de flamme (orange/rouge intense)
- [ ] Seuil de luminosité anormale configurable
- [ ] Action automatique : pause + alarme sonore + notification
- [ ] Log de l'événement avec capture d'écran

### F79 — Photo engraving wizard (assistant gravure photo)
**Objectif**: guider l'utilisateur pas à pas pour obtenir la meilleure gravure photo.

**Validation**:
- [ ] Étapes : import → recadrage → contraste/luminosité → dithering → preview → params
- [ ] Recommandations auto selon le matériau (F1)
- [ ] Comparaison côte à côte des modes de tramage
- [ ] Presets : bois clair, bois foncé, ardoise, cuir, anodisé

### F80 — Vue split (design + GCode côte à côte)
**Objectif**: afficher simultanément la preview graphique et le code GCode.

**Validation**:
- [ ] Panneau splittable horizontalement ou verticalement
- [ ] Synchronisation : clic sur une ligne GCode → highlight du segment en preview
- [ ] Clic sur un segment en preview → scroll vers la ligne GCode
- [ ] Mode plein écran pour chaque vue

### F81 — Centerline trace (vectorisation au trait central)
**Objectif**: extraire la ligne médiane d'un trait pour graver en un seul passage.

**Validation**:
- [ ] Algorithme de squelettisation (thinning/medial axis)
- [ ] Application sur images bitmap importées
- [ ] Paramètre de seuil d'épaisseur de trait
- [ ] Comparaison preview : contour classique vs centerline

### F82 — Gravure cylindrique déroulée (unwrap rotary)
**Objectif**: dérouler un cylindre en 2D pour placement précis, puis recalculer les déformations.

**Validation**:
- [ ] Saisie diamètre → calcul de la circonférence déroulée
- [ ] Preview déroulée avec repères de jonction
- [ ] Compensation de distorsion pour surface courbe
- [ ] Compatible avec le rotary existant (MachineProfile)

### F83 — Mode découpe multi-matériaux (sandwich / Z-layers)
**Objectif**: gérer des empilements de matériaux avec paramètres par couche Z.

**Validation**:
- [ ] Définition de couches Z avec épaisseur et matériau
- [ ] Paramètres power/speed par couche Z
- [ ] Focus auto par couche (Z-offset)
- [ ] Preview en coupe transversale

### F84 — Détection automatique d'images dupliquées / superposées
**Objectif**: avertir si des chemins identiques ou superposés risquent de doubler la gravure.

**Validation**:
- [ ] Détection de chemins géométriquement identiques (à tolérance près)
- [ ] Détection de segments parfaitement superposés
- [ ] Alerte dans le preflight (F7)
- [ ] Action proposée : fusionner ou supprimer le doublon

### F85 — Générateur de motifs (patterns fill)
**Objectif**: remplir une forme avec des motifs décoratifs répétitifs.

**Validation**:
- [ ] Motifs intégrés : écailles, hex, chevrons, vagues, brique, diamants
- [ ] Paramètres : échelle, rotation, espacement
- [ ] Clipping au contour de la forme
- [ ] Preview temps réel du remplissage

### F86 — Weld / Trim / Extend vecteurs
**Objectif**: opérations de soudure, coupe et extension de chemins vectoriels.

**Validation**:
- [ ] Weld : fusionner les contours de formes qui se chevauchent
- [ ] Trim : couper les segments aux points d'intersection
- [ ] Extend : prolonger un chemin jusqu'à l'intersection avec un autre
- [ ] Undo/redo supporté pour ces opérations

### F87 — Gradient fill vectoriel
**Objectif**: remplissage avec densité variable pour effet d'ombre/profondeur.

**Validation**:
- [ ] Direction du gradient configurable (linéaire, radial)
- [ ] Espacement min/max des lignes de remplissage
- [ ] Preview du gradient en overlay
- [ ] Effet smooth sur la gravure finale

### F88 — Contour multi-offset (relief en escalier)
**Objectif**: générer N contours concentriques avec puissance décroissante.

**Validation**:
- [ ] Paramètres : nb de niveaux, offset entre niveaux, puissance par niveau
- [ ] Génération automatique des contours offset
- [ ] Preview avec dégradé de couleur par niveau
- [ ] Test : effet 3D visible sur bois

### F89 — Raccourcis personnalisables par l'utilisateur
**Objectif**: remapper librement tous les raccourcis clavier.

**Validation**:
- [ ] UI de configuration : action → combinaison de touches
- [ ] Détection de conflits entre raccourcis
- [ ] Reset aux valeurs par défaut
- [ ] Import/export de la config raccourcis

### F90 — Panneau de notes / annotations projet
**Objectif**: associer des notes textuelles au projet.

**Validation**:
- [ ] Zone de texte libre dans un panneau dédié
- [ ] Sauvegardé dans le fichier .a4l
- [ ] Horodatage optionnel des notes
- [ ] Recherche dans les notes

### F91 — Comparateur de paramètres avant/après
**Objectif**: visualiser l'impact d'un changement de paramètre avant de l'appliquer.

**Validation**:
- [ ] Diff visuel : temps estimé, risque thermique, coût
- [ ] Avant/après côte à côte
- [ ] Bouton "Appliquer" ou "Annuler"
- [ ] Historique des comparaisons

### F92 — Mode multi-fenêtres / panneaux détachables
**Objectif**: détacher des panneaux dans des fenêtres séparées pour multi-écrans.

**Validation**:
- [ ] Panneaux détachables : preview, console, GCode editor, caméra
- [ ] Mémorisation de la disposition des fenêtres
- [ ] Restauration de la disposition au redémarrage
- [ ] Fonctionnel sur Windows et Linux

### F93 — Générateur de test de focus Z
**Objectif**: générer un pattern de test pour trouver la hauteur focale optimale.

**Validation**:
- [ ] Grille de carrés/lignes gravés à différentes hauteurs Z
- [ ] Paramètres : plage Z, pas, puissance fixe
- [ ] Étiquetage automatique de chaque hauteur Z
- [ ] Résultat : l'utilisateur identifie visuellement le meilleur Z

### F94 — Contrôle interlock (capot, flux d'eau, température)
**Objectif**: gérer les capteurs de sécurité machine (capot ouvert, refroidissement).

**Validation**:
- [ ] Lecture des pins d'interlock via statut GRBL (Door, etc.)
- [ ] Alerte UI si capot ouvert pendant un job
- [ ] Pause automatique si interlock déclenché
- [ ] Paramétrage des pins et comportement dans le profil machine

### F95 — Langues supplémentaires (chinois, russe, turc, coréen, polonais)
**Objectif**: élargir la couverture linguistique pour les marchés clés du laser.

**Validation**:
- [ ] Ajout de 5+ langues dans le système i18n
- [ ] Traductions complètes de l'UI
- [ ] Support RTL amélioré (arabe déjà présent)
- [ ] Validation typographique (CJK, cyrillique)

### F96 — Conversion automatique unités (mm ↔ pouces)
**Objectif**: basculer entre mm et pouces dans toute l'interface.

**Validation**:
- [x] Sélecteur mm/inch global dans settings
- [ ] Conversion en temps réel de tous les champs numériques
- [x] GCode toujours généré en mm (G21) avec affichage adapté
- [x] Pas de perte de précision lors de la conversion

### F97 — Estimation usure tube laser par job
**Objectif**: estimer la durée de vie restante du tube en fonction de l'utilisation.

**Validation**:
- [ ] Calcul : énergie totale délivrée (puissance × temps) par job
- [ ] Compteur cumulé d'énergie (à vie, persisté)
- [ ] Configuration : type de tube, durée de vie nominale
- [ ] Alerte quand le seuil d'usure approche (ex: 80%)

### F98 — Mode présentation / démo (watermark + limitations)
**Objectif**: permettre de montrer le logiciel sans risque de gravure accidentelle.

**Validation**:
- [ ] Mode démo : toutes les fonctions accessibles sauf envoi GCode réel
- [ ] Watermark "DEMO" sur la preview et les exports
- [ ] Activation/désactivation via menu ou argument CLI
- [ ] Utile pour salons, formations, vidéos

### F99 — Système de plugins Lua/WASM
**Objectif**: permettre aux développeurs tiers d'étendre All4Laser.

**Validation**:
- [ ] Runtime Lua ou WASM intégré (sandboxé)
- [ ] API exposée : accès projet, couches, GCode, UI
- [ ] Chargement de plugins depuis un dossier
- [ ] Documentation API + plugin d'exemple

### F100 — Sauvegarde cloud & synchronisation multi-postes
**Objectif**: synchroniser projets et settings entre plusieurs postes via le cloud.

**Validation**:
- [ ] Backend : stockage fichiers (S3/GitHub/WebDAV)
- [ ] Sync des settings, profils machine, presets matériaux
- [ ] Sync des projets (.a4l) avec gestion de conflits
- [ ] Chiffrement des données en transit et au repos

### F101 — Stippling / gravure par points
**Objectif**: convertir une image en nuage de points (stipple) pour gravure artistique.

**Validation**:
- [ ] Algorithme de Voronoi stippling (weighted)
- [ ] Paramètres : nb de points, taille min/max, densité
- [ ] Preview interactive du résultat
- [ ] Génération GCode optimisée (G0 rapide entre points, pulse laser)

### F102 — Séparation couleurs (image → couches par couleur)
**Objectif**: séparer automatiquement une image couleur en couches par teinte dominante.

**Validation**:
- [ ] Détection des couleurs dominantes (clustering k-means)
- [ ] Création d'une couche par couleur détectée
- [ ] Paramètres de gravure différents par couche couleur
- [ ] Utile pour gravure multi-passes ou multi-matériaux

### F103 — Géométrie de construction (lignes de référence non gravées)
**Objectif**: dessiner des lignes/cercles de référence pour l'alignement, non inclus dans le GCode.

**Validation**:
- [ ] Type de couche spécial "construction" (non exportée)
- [ ] Affichage en pointillés / couleur distincte
- [ ] Snap des objets sur la géométrie de construction
- [ ] Exclusion automatique du preflight et du GCode

### F104 — Contrôle par gamepad / joystick (jog physique)
**Objectif**: jogger la machine avec un gamepad USB/Bluetooth.

**Validation**:
- [ ] Détection automatique des gamepads connectés
- [ ] Mapping axes analogiques → jog X/Y/Z
- [ ] Boutons configurables (home, start, pause, fire test)
- [ ] Dead zone et sensibilité ajustables

### F105 — Intégration MQTT / IoT
**Objectif**: publier/souscrire des événements machine via MQTT.

**Validation**:
- [ ] Client MQTT intégré (broker configurable)
- [ ] Topics publiés : statut, progression, alarmes, température
- [ ] Commandes reçues : start, pause, stop
- [ ] Intégration Home Assistant / Node-RED démontrée

### F106 — Templates de jobs réutilisables
**Objectif**: sauvegarder une configuration de job comme template réutilisable.

**Validation**:
- [ ] Sauvegarde : couches, paramètres, matériau, macros associées
- [ ] Application rapide d'un template à un nouveau design
- [ ] Bibliothèque de templates (CRUD)
- [ ] Import/export de templates (JSON)

### F107 — Halftone avancé (circulaire, linéaire, personnalisé)
**Objectif**: générer des trames halftone vectorielles à partir d'images.

**Validation**:
- [ ] Modes : points circulaires, lignes, losanges, custom SVG
- [ ] Taille des éléments proportionnelle à la luminosité
- [ ] Résolution (LPI) configurable
- [ ] Preview vectorielle en temps réel

### F108 — Snap aux points clés (centres, milieux, intersections)
**Objectif**: accrocher le curseur aux points géométriques remarquables.

**Validation**:
- [ ] Snap : endpoints, midpoints, centers, intersections, perpendiculaires
- [ ] Indicateur visuel du type de snap actif
- [ ] Activation/désactivation par type de snap
- [ ] Raccourci pour toggle snap global

### F109 — Traitement batch multi-fichiers
**Objectif**: appliquer les mêmes paramètres et traitements à plusieurs fichiers d'un coup.

**Validation**:
- [ ] Sélection de dossier ou fichiers multiples
- [ ] Application d'un template de job (F106)
- [ ] Export batch de GCode
- [ ] Rapport de traitement (succès/échecs)

### F110 — Versioning de projet (historique des révisions)
**Objectif**: sauvegarder un historique de versions du projet.

**Validation**:
- [ ] Sauvegarde incrémentale à chaque save (max N versions)
- [ ] Navigateur de versions avec preview
- [ ] Restauration d'une version antérieure
- [ ] Diff visuel entre deux versions

### F111 — Marquage industriel UDI/DMC (conformité traçabilité)
**Objectif**: générer des marquages conformes aux normes UDI (FDA) et DataMatrix.

**Validation**:
- [ ] Génération DataMatrix conforme ISO/IEC 16022
- [ ] Champs UDI : GTIN, lot, date, série
- [ ] Vérification de lisibilité (grading) en preview
- [ ] Templates pour dispositifs médicaux / aéronautique

### F112 — Calibration axes (steps/mm, équerrage, backlash)
**Objectif**: assistant de calibration précise des axes machine.

**Validation**:
- [ ] Test de déplacement : graver un carré 50mm, mesurer, corriger
- [ ] Calcul automatique du ratio steps/mm
- [ ] Test d'équerrage : diagonales d'un rectangle
- [ ] Test de backlash : aller-retour avec mesure

### F113 — Edge detection pour vectorisation (Canny, Sobel)
**Objectif**: détecter les contours d'une image pour vectorisation précise.

**Validation**:
- [ ] Algorithmes : Canny, Sobel, Laplacien
- [ ] Paramètres de seuil haut/bas ajustables
- [ ] Conversion auto contours → chemins vectoriels
- [ ] Comparaison preview : original vs contours détectés

### F114 — Workflow bijouterie (gravure intérieure bague, pendentif)
**Objectif**: outils spécifiques pour la gravure de bijoux.

**Validation**:
- [ ] Templates : intérieur bague (déroulé circulaire), pendentif, bracelet
- [ ] Polices spéciales : script, monogramme
- [ ] Paramètres optimisés métaux précieux (or, argent, titane)
- [ ] Preview 3D simplifiée sur forme bijou

### F115 — Workflow signalétique (plaques, panneaux multicouches)
**Objectif**: créer des plaques signalétiques multicouches (gravure + remplissage peinture).

**Validation**:
- [ ] Templates : plaque de porte, panneau directionnel, badge
- [ ] Gestion des profondeurs de gravure par zone
- [ ] Export multi-passes : gravure profonde + gravure fine
- [ ] Simulation du résultat avec couleurs de remplissage

### F116 — Mode textile / cuir (gestion du déplacement matériau souple)
**Objectif**: adapter le workflow pour les matériaux souples qui bougent.

**Validation**:
- [ ] Compensation de dérive via repères caméra périodiques
- [ ] Réduction de vitesse auto sur matières légères
- [ ] Presets : cuir, jeans, feutre, nylon, coton
- [ ] Option air assist réduit (pour ne pas déplacer le tissu)

### F117 — QR code scanner via caméra pour chargement de job
**Objectif**: scanner un QR code pour charger automatiquement le job associé.

**Validation**:
- [ ] Lecture QR code depuis le flux caméra
- [ ] QR contenant : chemin fichier, URL, ou ID de job
- [ ] Chargement automatique du projet/GCode
- [ ] Utile pour workflow atelier (coller un QR sur chaque pièce)

### F118 — Voix synthétisée pour statut machine (TTS)
**Objectif**: annoncer vocalement les événements machine.

**Validation**:
- [ ] TTS système (SAPI Windows / espeak Linux)
- [ ] Événements : début job, fin job, erreur, alarme
- [ ] Volume et vitesse configurables
- [ ] Option désactivable

### F119 — Pendant / boîtier de commande physique USB
**Objectif**: supporter les pendants CNC USB pour contrôle physique.

**Validation**:
- [ ] Détection HID USB (pendant type WHB04B, etc.)
- [ ] Mapping des boutons et molette
- [ ] Jog via molette avec sélection d'axe
- [ ] Override feed/spindle via potentiomètres

### F120 — Dot peen marking simulation
**Objectif**: simuler/générer du marquage par micro-percussion.

**Validation**:
- [ ] Conversion texte/vecteur → nuage de points espacés
- [ ] Paramètres : espacement, profondeur (puissance), vitesse
- [ ] Preview des points
- [ ] GCode compatible (pulsed laser mode)

### F121 — Intégration ERP / gestion de commandes
**Objectif**: connecter All4Laser à un ERP pour automatiser les commandes.

**Validation**:
- [ ] API REST pour recevoir des ordres de fabrication
- [ ] Champs : référence commande, quantité, fichier, matériau
- [ ] Retour de statut : en cours, terminé, erreur
- [ ] Compatible avec les ERP open source (ERPNext, Odoo)

### F122 — Générateur de lithophanie
**Objectif**: convertir une photo en lithophanie (gravure à travers un matériau translucide).

**Validation**:
- [ ] Conversion image → relief par variation d'épaisseur gravée
- [ ] Paramètres : épaisseur min/max, résolution
- [ ] Preview simulée avec rétroéclairage virtuel
- [ ] Optimisé pour acrylique blanc et corian

### F123 — Multi-utilisateurs temps réel (collaboration)
**Objectif**: éditer un projet à plusieurs simultanément.

**Validation**:
- [ ] Serveur de sync temps réel (WebSocket/CRDT)
- [ ] Curseurs multiples visibles
- [ ] Gestion de conflits (verrouillage par objet)
- [ ] Chat intégré entre collaborateurs

### F124 — Système de scoring qualité du design
**Objectif**: noter automatiquement la qualité/faisabilité d'un design avant gravure.

**Validation**:
- [ ] Score : complexité, risque thermique, temps estimé, compatibilité matériau
- [ ] Indicateur visuel (vert/jaune/rouge)
- [ ] Suggestions d'amélioration automatiques
- [ ] Intégration avec le preflight (F7)

---

## Journal de progression

> Mettre à jour à chaque livraison partielle.

- 2026-03-02: Création du tracker initial.
- 2026-03-02: F1 implémentée et validée (materials intelligents + recommandations + persistance settings/projet).
- 2026-03-02: F2 implémentée et validée (Auto Nesting avec marges/rotation/tests).
- 2026-03-02: F3 implémentée et validée (assistant kerf + application toolpath + tests).
- 2026-03-02: F4 implémentée et validée (Job Queue batch, retry échec, auto-run, historique).
- 2026-03-02: F5 implémentée et validée (thermal risk heatmap, overlay preview, threshold/cell UI, 2 tests).
- 2026-03-03: F8 implémentée et validée (format macro JSON, validation d’édition, exécution séquentielle filtrée, erreurs claires + exemples par défaut).
- 2026-03-03: F6 et F7 implémentées et validées (auto-détection repères caméra + mapping + alignement assisté manuel, preflight chemins/couches avec rapport UI et blocage optionnel des erreurs critiques).
- 2026-03-04: F11 implémentée (profils machine multiples : MachineProfileStore CRUD, sélecteur combo, import/export JSON, migration legacy).
- 2026-03-04: F14 implémentée (notification son système Windows MessageBeep à la fin du job + toggle mute).
- 2026-03-04: F33 implémentée (perforation/pointillés : paramètres cut_mm/gap_mm par couche, découpe dashed dans path_utils).
- 2026-03-04: F40 implémentée (corner power control : détection angle entre segments, réduction S proportionnelle, seuil configurable).
- 2026-03-04: F64 implémentée (GCode annoté : comments_enabled toggle dans GCodeBuilder, header metadata, paramètres couche en commentaires).
- 2026-03-04: F71 implémentée (auto-save toutes les 60s dans autosave.a4l.recovery, prompt de restauration au démarrage).
- 2026-03-04: F96 implémentée (DisplayUnit mm/in avec conversion, sélecteur dans la status bar, persistance settings).
- 2026-03-04: F36 implémentée (checkpoint_line dans recovery snapshot, restauration program_index au redémarrage).
- 2026-03-04: F39 implémentée (align left/right/top/bottom/center H/V + distribute H/V, boutons dans Advanced Tools avec undo).
- 2026-03-04: F47 implémentée (envoi $I à la connexion, détection GRBL/FluidNC, auto-config backend).
- 2026-03-04: F55 implémentée (color picker RGB dans Cut Settings, remplacement du swatch statique).
- 2026-03-04: F12 implémentée (power ramping : ramp_length_mm/ramp_start_pct par couche, génération GCode avec S progressif, UI dans Cut Settings).
- 2026-03-04: F22 et F23 validées (tabs/bridges + lead-in/lead-out déjà implémentés dans path_utils + layers_new + cut_settings UI).
- 2026-03-04: F43 implémentée (startup wizard 3 étapes : langue, dimensions machine, contrôleur, first_run_done persisté).
- 2026-03-04: F54 implémentée (export SVG : conversion shapes → SVG avec couleurs couches, viewBox en mm).
- 2026-03-04: F84 implémentée (détection shapes identiques/superposées dans preflight, alerte double-burn).
- 2026-03-04: F97 implémentée (tube_hours_total persisté dans profil, record_job_burn_time après chaque job, alerte >80%).
- 2026-03-04: F17 implémentée (cost_per_hour/cost_per_m2/cost_currency dans settings, affichage coût dans status bar).
- 2026-03-04: F27 implémentée (compteurs maintenance lens_clean/belt_check, alertes après N jobs, reset methods).
- 2026-03-04: F44 validée (fill_bidirectional + fill_overscan_mm déjà implémentés dans fill.rs).
- 2026-03-04: F50 implémentée (outil de mesure sur canvas : measure_mode, screen_to_world, overlay distance/dx/dy).
- 2026-03-04: F67 implémentée (event_log persistant dans event_log.txt, chargé au démarrage, sauvé avec settings).
- 2026-03-04: F77 implémentée (exhaust fan M7/M9 par couche, post-delay G4, UI dans Cut Settings).
- 2026-03-04: F90 implémentée (onglet Notes dans right panel, project_notes persisté dans ProjectFile).
- 2026-03-04: F93 implémentée (générateur Z focus test : lignes à différentes hauteurs Z).
- 2026-03-04: F15 implémentée (rapport job CSV auto-généré après chaque job : date, fichier, machine, distances, temps, couches).
- 2026-03-04: F24 implémentée (pass_offset_mm par couche, UI dans Cut Settings quand passes > 1).
- 2026-03-04: F94 implémentée (interlock lid/water dans profil machine, checks dans preflight report).
- 2026-03-04: F51 implémentée (group_id sur ShapeParams, group/ungroup/expand_group_selection, boutons UI).
- 2026-03-04: F52 implémentée (historique jobs persistant job_history.txt, stats_summary, save/load).
- 2026-03-04: F65 implémentée (tooltips contextuels sur tous les paramètres de Cut Settings : speed, power, mode, fill, kerf, lead-in/out, overscan).
- 2026-03-04: F103 implémentée (is_construction flag sur CutLayer, exclusion du GCode, checkbox dans Cut Settings).
